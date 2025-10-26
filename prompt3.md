#rustmanager

The project is a rust-based html server deployed as a container. You will be asked to generate a feature for inclusion with this existing project. FOCUS: Respond only with the minimum code requirements to deliver the feature. Do not make plans for features that have not been requested.  Do not add functionality that is not requested.  Do not optimize or otherwise alter the code unless specifically requested.  Follow the provided instructions.

build platform: podman 3.4.4 (Only build a Containerfile)

The following base `((Cargo.toml))` will be used, although the presence of a dependency in this file does not obligate the project to use it. You may add to it, but not take away from it.
```
[package]
name = "rust-manager"
version = "0.1.0"
edition = "2024"

[dependencies]
axum = "0.8.6"
hyper = "1.7.0"
dioxus = { version = "0.6.2", features = ["web", "ssr", "html"] }
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
virt = "0.4.3"
dioxus-ssr = "0.6.2"
```

The following is already defined in `src/main.rs`:

```
    mod dashboard;

    let session_config = SessionConfig::default().with_table_name("sessions_table");

    let session_store =
        SessionStore::<SessionSqlitePool>::new(Some(pool.clone().into()), session_config)
            .await
            .unwrap();

    let app = Router::new()
        .route("/", get(root))
        .route("/login", get(login_page).post(login_action))
        .route("/logout", post(logout))
        .route("/dashboard", get(dashboard::dashboard_page))
        .with_state(pool.clone())
        .layer(SessionLayer::new(session_store));

```
This is the current state of `src/dashboard.rs`:
```
use axum::response::Html;
use dioxus::prelude::*;
use dioxus_ssr::render_element;

#[component]
fn DashboardPage() -> Element {
    rsx! {
      div {
        div {
          div { class: "app", style: "display:flex;height:100vh;",
            // ─ Side‑menu ───────────────────────────────────────────────────────
            div {
              class: "side-menu",
              style: "display:flex;flex-direction:column;width:200px;background:#2c3e50;",
              button { style: "background:none;border:none;color:white;font-size:24px;cursor:pointer;margin:10px;",
                "≡"
              }
              ul { style: "list-style:none;padding:0;margin:0;",
                li { style: "color:white;display:flex;align-items:center;padding:10px;",
                  "Domain"
                }
                li { style: "color:white;display:flex;align-items:center;padding:10px;",
                  "Host"
                }
                li { style: "color:white;display:flex;align-items:center;padding:10px;",
                  "Network"
                }
                li { style: "color:white;display:flex;align-items:center;padding:10px;",
                  "Secret"
                }
                li { style: "color:white;display:flex;align-items:center;padding:10px;",
                  "Pool"
                }
                li { style: "color:white;display:flex;align-items:center;padding:10px;",
                  "Volume"
                }
              }
            }
            // ─ Main content area ───────────────────────────────────────────────
            div { style: "flex:1;display:flex;flex-direction:column;",
              // ─ Top‑bar ──────────────────────────────────────────────────────
              div {
                class: "top-bar",
                style: "display:flex;align-items:center;justify-content:space-between;background:#ecf0f1;padding:10px;",
                div { style: "display:flex;align-items:center;",
                  button { style: "background:none;border:none;font-size:16px;margin-right:20px;",
                    "File"
                  }
                  button { style: "background:none;border:none;font-size:16px;margin-right:20px;",
                    "Edit"
                  }
                  button { style: "background:none;border:none;font-size:16px;",
                    "Help"
                  }
                }
                div { style: "position:relative;",
                  button { style: "background:none;border:none;cursor:pointer;",
                    "User"
                  }
                  div {
                    class: "dropdown",
                    style: "position:absolute;right:0;top:30px;background:white;border:1px solid #ccc;box-shadow:0 2px 5px rgba(0,0,0,0.2);display:flex;flex-direction:column;",
                    button { style: "background:none;border:none;text-align:left;padding:8px 12px;width:100%;",
                      "Profile"
                    }
                    button { style: "background:none;border:none;text-align:left;padding:8px 12px;width:100%;",
                      "Settings"
                    }
                    button { style: "background:none;border:none;text-align:left;padding:8px 12px;width:100%;",
                      "Logout"
                    }
                  }
                }
              }
              // ─ Content ─────────────────────────────────────────────────────────
              div { style: "flex:1;background:#bdc3c7;display:flex;align-items:center;justify-content:center;",
                h1 { "Dashboard Content" }
              }
            }
          }
        }
      }
    }
}

pub async fn dashboard_page() -> Html<String> {
    // `render_element` consumes the rsx! tree and produces an HTML string.
    let rendered_html = render_element(rsx!(DashboardPage {}));
    Html(rendered_html)
}
```
Your instructions:
Refactor """src/dashboard.rs""" module to enable reactivity to make the drop-down menus and hamburger icon responsive to the mouse.  Add an icon set and use it to provide a placeholder avatar and appropriate icons for the left-hand menu.
Features:
- Display a drop-down menu along the top of the page. The menu items are "file", "edit", and "help";
- Display a vertical menu along the left-hand side of the page. The menu items are from libvirt and are as follows: "domain", "host", "network","secret","pool" and "volume". Each menu item will have text and an appropriate icon. The width of the panel is controlled by a "hamburger" style control at the top or bottom which will collapse and expand this side menu. When collapsed only images are visible'
- Display a user avatar in the upper right, along with a drop down menu that supports options for "Profile", "Settings", and "Logout"

dioxus provides these known reactivity helpers: use_signal, use_effect, use_memo, use_resource
Additionally dioxus provides these event handlers: onclick, oninput
There is no use_state in dioxus.  Keep this in mind as you develop the response.

Compute the dynamic CSS pieces (width and show/hide styles) as Rust values before calling rsx! and inject them into the style attributes using rsx expression interpolation to avoid unescaped braces and nested quotes.

Show only the refactored implementation of `src/dashboard.rs`
