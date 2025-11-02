#[cfg(feature = "server")]
use axum::routing::post;
#[cfg(feature = "server")]
use serde::{Deserialize, Serialize};

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

// ---------- Wizard data model ----------
#[cfg(feature = "server")]
#[derive(Deserialize, Serialize, Debug)]
pub struct WizardData {
    pub name: Option<String>,
    pub number: Option<u8>,
    pub color: Option<String>,
    pub date: Option<String>,
}

// ---------- Stub POST handler ----------
#[cfg(feature = "server")]
async fn wizard_submit(axum::Json(data): axum::Json<WizardData>) -> axum::http::StatusCode {
    println!("Wizard submitted: {:?}", data);
    axum::http::StatusCode::OK
}

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
        .route("/wizard/submit", post(wizard_submit)) 
        .layer(
            AuthSessionLayer::<User, i64, SessionSqlitePool, SqlitePool>::new(Some(pool))
                .with_config(auth_config),
        )
        .layer(SessionLayer::new(session_store))
}
