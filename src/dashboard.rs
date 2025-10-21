use axum::response::Html;

use dioxus::prelude::*;
use dioxus_ssr::render_element;

#[component]
fn DashboardPage() -> Element {
    // Reactive signals
    let mut collapsed = use_signal(|| false); // side‑menu collapse state
    let mut user_menu_open = use_signal(|| false); // user avatar drop‑down state

    // Dynamic CSS values
    let side_menu_width = if collapsed() { "60px" } else { "200px" };
    let text_display = if collapsed() { "none" } else { "block" };

    // Icons (Unicode placeholders)
    let top_icons = [("file", "📄"), ("edit", "✏️"), ("help", "❓")];
    let side_items = [
        ("Domain", "🗂️"),
        ("Host", "🏠"),
        ("Network", "🌐"),
        ("Secret", "🔑"),
        ("Pool", "🔋"),
        ("Volume", "📦"),
    ];

    rsx! {
      div { class: "app", style: "display:flex;height:100vh;",
        div {
          class: "side-menu",
          style: format!(
              "display:flex;flex-direction:column;width:{};background:#2c3e50;",
              side_menu_width,
          ),
          button {
            style: "background:none;border:none;color:white;font-size:24px;cursor:pointer;margin:10px;",
            onclick: move |_| {
                collapsed.toggle();
            },
            "☰"
          }
          ul { style: "list-style:none;padding:0;margin:0;",

            for (text , icon) in side_items {
              li { style: "color:white;display:flex;align-items:center;padding:10px;",
                span { style: "font-size:20px;", "{icon}" }
                span { style: format!("display:{};margin-left:8px;", text_display),
                  button {
                    "{text}"
                  }
                }
              }
            }
          }
        }
        div { style: "flex:1;display:flex;flex-direction:column;",
          div {
            class: "top-bar",
            style: "display:flex;align-items:center;justify-content:space-between;background:#ecf0f1;padding:10px;",
            div { style: "display:flex;align-items:center;",

              for (name , icon) in top_icons {
                button { style: "background:none;border:none;font-size:16px;margin-right:20px;",
                  "{icon}-{name}"

                }
              }
            }
            div { style: "position:relative;",
              button {
                style: "border:none;background:none;cursor:pointer;",
                onclick: move |_| user_menu_open.toggle(),
                span { style: "border-radius:50%;width:32px;height:32px;background:#777;color:white;display:flex;align-items:center;justify-content:center;",
                  "🧑"
                }
              }
              if user_menu_open() {
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
          }
          div { style: "flex:1;background:#bdc3c7;display:flex;align-items:center;justify-content:center;",
            h1 { "Dashboard Content" }
            span { "toggled is {collapsed()}" }
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
