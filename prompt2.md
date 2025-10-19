#rustmanager

The project is to be a rust-based html server "minimum viable product" and complete proof of concept application that implements a "wizard" to demonstrate a multistep process. Navigation forward and backward thru the wizard is managed by session state on the server. The final "submit" will be captured by the server and will be printed to the console log for this poc.

Implement a route `/wizard/example` that will allow the user to navigate thru five different steps of a wizard using the following steps:
Step 1: Enter a name - Require the user to enter a text string into an input box;
Step 2: Enter a number - Require the user to select a numerif value from 1 to 10;
Step 3: Select from a list - provide a list of seven colors and require the user to select one of them;
Step 4: Select a date - provide a datepicker and require the user to select a date;
Step 5: Review - Display the previous selections and require the user to confirm them (e.g. 'Submit') or cancel.
Each step allows the user to go 'Back' to the previous step and 'Forward' to the next step if the current selection has been made.
Clicking 'Cancel' at any time restarts the wizard

The following base `((Cargo.toml))` will be used, although the presence of a dependency in this file does not obligate the project to use it. You may add to it, but not take away from it.

```
[package]
name = "rust-manager"
version = "0.1.0"
edition = "2024"

[dependencies]
axum = "0.8.6"
hyper = "1.7.0"
dioxus = { version = "0.6.2", features = ["web"] }
axum_session = { version = "0.17", features = ["key-store"] }
axum_session_sqlx = { version = "0.6.0", features = ["sqlite", "tls-rustls"]}
sqlx = { version = "0.8", features = ["sqlite", "runtime-tokio-native-tls"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
rust-argon2 = "3.0"
rand = "0.8"
uuid = "1.18.1"
hyper-util = "0.1.17"
anyhow = "1.0.100"
rand_core = "0.9.3"
virt = "0.4"
```

The following routes are already implemented.
```
let session_config = SessionConfig::default().with_table_name("sessions_table");

let session_store =
		SessionStore::<SessionSqlitePool>::new(Some(pool.clone().into()), session_config)
				.await
				.unwrap();

let app = Router::new()
        .route("/", get(root))
        .route("/login", get(login_page).post(login_action))
        .route("/dashboard", get(dashboard))
        .route("/logout", post(logout))
        .with_state(pool.clone())
        .layer(SessionLayer::new(session_store));
```
Add the new route to this existing app by creating a new module, `mod wizard;`.  Do not regenerate the full main.rs.  Instead generate only the wizard.rs and only the lines that will be added to the existing main.rs.