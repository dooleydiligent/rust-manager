use argon2::{self, Config};
use axum::{
    Router,
    extract::{Form, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
};
use axum_session::{Session, SessionConfig, SessionLayer, SessionStore};
use axum_session_sqlx::SessionSqlitePool;

use rand::RngCore;
use rand::rngs::OsRng;

use serde::{Deserialize, Serialize};
use sqlx::{SqlitePool, sqlite::SqliteConnectOptions, sqlite::SqlitePoolOptions};
use std::net::SocketAddr;

mod api;
mod wizard;
/// User record – only the fields we need for authentication
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
struct User {
    id: i64,
    username: String,
    password_hash: String, // Argon2 hash
}

#[derive(Debug, Deserialize)]
struct LoginForm {
    username: String,
    password: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1️⃣  Connect to (and initialise) the database
    let db_path = "data.db";
    let options = SqliteConnectOptions::new()
        .filename(db_path)
        .create_if_missing(true);
    SqlitePool::connect_with(options).await?;

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&format!("sqlite://{}", db_path))
        .await?;
    init_db(&pool).await?;

    // Create table if not exists
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS session (
            session_id TEXT PRIMARY KEY,
            data BLOB NOT NULL,
            expires_at INTEGER NOT NULL
        );
        "#,
    )
    .execute(&pool)
    .await?;

    // 2️⃣  Generate a random 32‑byte key for axum_session
    let mut key_bytes = [0u8; 32];
    OsRng.fill_bytes(&mut key_bytes);

    let session_config = SessionConfig::default().with_table_name("sessions_table");

    let session_store =
        SessionStore::<SessionSqlitePool>::new(Some(pool.clone().into()), session_config)
            .await
            .unwrap();

    // 3️⃣  Build the router
    let app = Router::new()
        .route("/", get(root))
        .route("/login", get(login_page).post(login_action))
        .route("/dashboard", get(dashboard))
        .route("/logout", post(logout))
        .route("/api/domains", get(api::get_domains))
        .route(
            "/wizard/example",
            get(wizard::wizard_get).post(wizard::wizard_post),
        )
        .with_state(pool.clone())
        .layer(SessionLayer::new(session_store));
    // Register the wizard routes
    // .merge(crate::wizard::add_wizard_routes(Router::new()));

    // 4️⃣  Run
    let addr = SocketAddr::from(([0, 0, 0, 0], 3302));
    println!("🚀 Server listening on http://{}/", addr);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3302").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

/// Initialise the SQLite database (users table + default admin)
async fn init_db(pool: &SqlitePool) -> anyhow::Result<()> {
    // Create table if missing
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE,
            password_hash TEXT NOT NULL
        );
        "#,
    )
    .execute(pool)
    .await?;

    // Add a default user (`admin` / `password`) if the table is empty
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(pool)
        .await?;
    if count.0 == 0 {
        let password = b"password";
        let salt = b"admin_salt";
        let config = Config::default();
        let password_hash = argon2::hash_encoded(password, salt, &config).unwrap();

        sqlx::query("INSERT INTO users (username, password_hash) VALUES (?, ?)")
            .bind("admin")
            .bind(password_hash.to_string())
            .execute(pool)
            .await?;
        println!("🔑 Created default user `admin` with password `password`");
    }
    Ok(())
}

/// Root handler – redirects based on session
async fn root(
    session: Session<SessionSqlitePool>,
    State(_pool): State<SqlitePool>,
) -> impl IntoResponse {
    if let Some(_user_id) = session.get::<i64>("user_id") {
        // User is authenticated – go straight to dashboard
        (
            StatusCode::FOUND,
            axum::response::AppendHeaders([("location", "/dashboard")]),
        )
            .into_response()
    } else {
        // Not logged in – show landing page
        (
            StatusCode::FOUND,
            // axum::response::AppendHeaders([("location", "/landing")]),
            Html(
                r#"
                <html><head><title>Rust Manager</title><link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@picocss/pico@latest/css/pico.min.css"></head><body>
                <h1>Welcome to rust-manager</h1>
                <p><a href="/login">Login</a></p>
                </body></html>
                "#,
            ),
        )
            .into_response()
    }
}

// The login form (GET)
async fn login_page() -> Html<&'static str> {
    Html(
        r#"
        <html><head><title>Login</title><link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@picocss/pico@latest/css/pico.min.css"></head><body>
        <h2>Login</h2>
        <form action="/login" method="post">
            <label>Username: <input name="username" /></label><br/>
            <label>Password: <input name="password" type="password"/></label><br/>
            <button type="submit">Login</button>
        </form>
        </body></html>
        "#,
    )
}

/// Handle the logout action
async fn logout(
    session: Session<SessionSqlitePool>,
    State(_pool): State<SqlitePool>,
) -> impl IntoResponse {
    // Get user id from session

    session.remove("user_id");

    return (
        StatusCode::FOUND,
        axum::response::AppendHeaders([("location", "/login")]),
    )
        .into_response();
}

/// Handle the login form submission
async fn login_action(
    session: Session<SessionSqlitePool>,
    State(pool): State<SqlitePool>,
    Form(form): Form<LoginForm>,
) -> impl IntoResponse {
    // Fetch user by username
    let user: Option<User> =
        sqlx::query_as("SELECT id, username, password_hash FROM users WHERE username = ?")
            .bind(&form.username)
            .fetch_optional(&pool)
            .await
            .unwrap_or(None);

    if let Some(user) = user {
        // Verify password
        if argon2::verify_encoded(&user.password_hash, form.password.as_bytes()).unwrap_or(false) {
            // Store user id in session
            session.set("user_id", user.id);
            return (
                StatusCode::FOUND,
                axum::response::AppendHeaders([("location", "/dashboard")]),
            )
                .into_response();
        }
    }

    // Authentication failed – reload login with error
    return (
        StatusCode::OK,
        Html(format!(
            r#"
        <html><head><title>Login</title><link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@picocss/pico@latest/css/pico.min.css"></head><body>
        <h2>Login</h2>
        <p style="color:red;">Invalid username or password</p>
        <form action="/login" method="post">
            <label>Username: <input name="username" /></label><br/>
            <label>Password: <input name="password" type="password"/></label><br/>
            <button type="submit">Login</button>
        </form>
        </body></html>
        "#
        )),
    )
        .into_response();
}

/// Dashboard – only shown to authenticated users
async fn dashboard(
    session: Session<SessionSqlitePool>,
    State(pool): State<SqlitePool>,
) -> impl IntoResponse {
    // Get user id from session
    let user_id = match session.get::<i64>("user_id") {
        Some(id) => id,
        None => {
            return (
                StatusCode::FOUND,
                axum::response::AppendHeaders([("location", "/login")]),
            )
                .into_response();
        }
    };

    // Get username for display
    let user: User = sqlx::query_as("SELECT id, username, password_hash FROM users WHERE id = ?")
        .bind(user_id)
        .fetch_one(&pool)
        .await
        .unwrap();

    // Render a very simple dashboard
    return (
        StatusCode::OK,
        Html(format!(
            r#"
        <html><head><title>Dashboard</title>
        <link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@picocss/pico@latest/css/pico.min.css"></head><body>
        <h1>Welcome, {username}!</h1>
        <p>This is your Rust‑Manager dashboard.</p>
        <form action="/logout" method="post"><button type="submit">Logout</button></form>
        </body></html>
        "#,
            username = user.username
        )),
    )
        .into_response();
}
