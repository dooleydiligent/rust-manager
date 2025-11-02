Given a rust dioxus **fullstack** application, implement a route '''/wizard/example''' that will allow the user to navigate thru five different steps of a wizard using the following steps:

Step 1: Enter a name - Require the user to enter a text string into an input box;
Step 2: Enter a number - Require the user to select a numerif value from 1 to 10;
Step 3: Select from a list - provide a list of seven colors and require the user to select one of them;
Step 4: Select a date - provide a datepicker and require the user to select a date;
Step 5: Review - Display the previous selections and require the user to confirm them (e.g. 'Submit') or cancel.

Each step allows the user to go 'Back' to the previous step and 'Forward' to the next step if the current selection has been made.
Clicking 'Cancel' at any time restarts the wizard
Clicking 'Submit' at the last step will generate a POST request to the backend where the data will be processed. For now just create a "stub" endpoint for the backend POST request.

Pay attention to these:

- Ensure that new files are imported where they are used. Do **not** create unused imports;
- Use only features and functions found in the dependencies listed in Cargo.toml. Instead of use_state prefer use_signal. There is no onpress - prefer onclick;
- Assume the application is deployed at https://rust.example.com. Do not hard code 'localhost' when creating requests;
- Infer existing modules from the imports listed, below, in main.rs;
- Ensure functions are properly annotated with component, derive, and/or cfg as appropriate;
- Use tailwindcss classes where appropriate;
- This is a **dioxus** **fullstack** application. Use the existing framework where possible. Add dependencies to Cargo.toml only when necessary;
- Presence of a dependency in Cargo.toml does not obligate you to use that dependecy.
- When adding dependencies, use '''cargo add ''' from the cli instead of rewriting the entire Cargo.toml.
- Keep the response concise by only showing modified and/or added lines and files.

What follows is the existing Cargo.toml and ./src/main.rs, for your reference.

# ./Cargo.toml

```
[package]
name = "auth"
version = "0.1.0"
authors = ["mike"]
edition = "2021"

[dependencies]
dioxus = { version = "0.6.2", features = ["router", "fullstack"  ] }
tokio = { version = "1.20.0", features = ["full"], optional = true }
sqlx = { version="0.7.0", features = [ "sqlite", "runtime-tokio", "tls-native-tls" ], optional = true }
axum_session = {version = "0.12.1", features = ["sqlite-rustls"], optional = true  }
axum_session_auth = { version = "0.12.1", features = ["sqlite-rustls"], optional = true  }
axum = {version = "0.7.0", optional = true}
bcrypt = {version = "0.16.0", optional = true }
async-trait = {version = "0.1.87", optional = true  }
anyhow = {version = "1.0.97", optional = true }
tracing = "0.1"

[features]
default = []
web = ["dioxus/web"]
desktop = ["dioxus/desktop"]
mobile = ["dioxus/mobile"]
server = ["dioxus/server", "dep:tokio" , "sqlx", "axum_session", "axum_session_auth" , "axum", "bcrypt", "async-trait", "anyhow"]

[profile]

[profile.wasm-dev]
inherits = "dev"
opt-level = 1

[profile.server-dev]
inherits = "dev"

[profile.android-dev]
inherits = "dev"

```

# ./src/main.rs

```
#![allow(non_snake_case)]

use dioxus::prelude::*;

#[cfg(feature="server")]
use backend::axum_server::launch_server;

mod backend;
mod components;
use components::{register::Register, login::Login, user::User, dashboard::Dashboard};

fn main() {
    #[cfg(feature = "web")]
    LaunchBuilder::web().launch(App);

    #[cfg(feature="server")]
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        launch_server(App).await;
    });
}

#[component]
fn App() -> Element {
    rsx!(
        document::Link { rel: "icon", href: asset!("/assets/favicon.ico") }
        document::Stylesheet { href: asset!("/assets/tailwind.css") }
        document::Stylesheet { href: asset!("/assets/main.css") }
        Router::<Route> {}
    )
}

#[component]
pub fn NotFound(route: Vec<String>) -> Element {
    let route_str = route.join("/");
    rsx! {
        div { class: "flex justify-center items-center screen",
            div { class: "text-5xl", "404 - /{route_str} not found" }
        }
    }
}

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[route("/")]
    Login{},
    #[route("/register")]
    Register {},
    #[route("/user")]
    User {},
    #[route("/dashboard")]
    Dashboard {},
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}
```

# ./src/backend/axum_server.rs

```
#[cfg(feature="server")]
use dioxus::prelude::*;

#[cfg(feature="server")]
use super::{auth_session::auth_session_config, db::get_db, router::router, session::session};


#[cfg(feature="server")]
pub async fn launch_server(app: fn()-> Element) {
  let pool = get_db().await;
  let session_store = session().await;
  let auth_session_config = auth_session_config();
  let router =router(session_store, auth_session_config, pool.clone(), app);

  let addr = dioxus::cli_config::fullstack_address_or_localhost();

  let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

  axum::serve(listener, router).await.unwrap()
}
```

# ./src/backend/router.rs

```
#[cfg(feature = "server")]
use super::auth_session::User;
#[cfg(feature = "server")]
use axum::Router;
#[cfg(feature = "server")]
use axum_session::{SessionLayer, SessionSqlitePool, SessionStore};
#[cfg(feature = "server")]
use axum_session_auth::{AuthConfig, AuthSessionLayer};
#[cfg(feature = "server")]
use dioxus::prelude::*;
#[cfg(feature = "server")]
use sqlx::{Pool, Sqlite, SqlitePool};

#[cfg(feature = "server")]
pub fn router(
    session_store: SessionStore<SessionSqlitePool>,
    auth_config: AuthConfig<i64>,
    pool: Pool<Sqlite>,
    app: fn() -> Element,
) -> Router {
    let config = ServeConfig::new().unwrap();

    Router::new()
        .serve_dioxus_application(config, app)
        .layer(
            AuthSessionLayer::<User, i64, SessionSqlitePool, SqlitePool>::new(Some(pool))
                .with_config(auth_config),
        )
        .layer(SessionLayer::new(session_store))
}
```

Show the code now.
