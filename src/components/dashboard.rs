use crate::{
    backend::server_functions::{is_authenticated, log_out},
    Route,
};
use dioxus::prelude::*;
use tracing::info;
#[component]
pub fn Dashboard() -> Element {
    // Reactive signals
    info!("Rendering DashboardPage component");
    let mut collapsed = use_signal(|| false); // side‑menu collapse state
    let mut user_menu_open = use_signal(|| false); // user avatar drop‑down state
    let _count = use_signal(|| 0);
    // Dynamic CSS values
    let side_menu_width = if collapsed() { "60px" } else { "200px" };
    let text_display = if collapsed() { "none" } else { "block" };

    // Signal that tells us whether the wizard should be shown
    let mut current_route = use_signal(|| None::<String>);
    let navigator = use_navigator();

    // Check authentication on mount and redirect to login if not authenticated
    let mut is_authed = use_signal(|| None::<bool>);
    let _ = use_resource(move || async move {
        match is_authenticated().await {
            Ok(v) => is_authed.set(Some(v)),
            Err(_) => is_authed.set(Some(false)),
        }
    });

    // If we have a determined auth state and it's false, redirect to Login
    if let Some(auth) = is_authed() {
        if auth == false {
            navigator.push(Route::Login {});
        }
    }

    // -------------------------------------------------------------
    // Icons (Unicode placeholders)
    let top_icons = [("file", "📄"), ("edit", "✏️"), ("help", "❓")];
    let side_items = [
        ("Domain", "🗂️"),
        ("Host", "🏠"),
        ("Network", "🌐"),
        ("Secret", "🔑"),
        ("Pool", "🔋"),
        ("Volume", "📦"),
        ("Wizard", "🧙🏿‍♂"), // when clicked the wizard will be rendered
    ];

    let data = use_signal(|| String::from("Initial"));

    use_effect(move || {
        // This effect will rerun if 'data' changes
        info!("Data changed to: {}", data.read());
    });

    rsx! {
        div { class: "app", style: "display:flex;height:100vh;",
            div {
                class: "side-menu",
                style: format!(
                    "display:flex;flex-direction:column;width:{};background:#2c3e50;",
                    side_menu_width,
                ),
                button {
                    style: format!(
                        "background:none;border:none;color:white;font-size:24px;cursor:pointer;margin:10px;{}",
                        if collapsed() { "" } else { "margin-left:auto;margin-right:0;" },
                    ),
                    onclick: move |_| {
                        collapsed.toggle();
                    },
                    "☰"
                }
                ul { style: "list-style:none;padding:0;margin:0;",
                    for (text , icon) in side_items {
                        li { style: "color:white;display:flex;align-items:center;padding:10px;",
                            span {
                                style: "font-size:20px;cursor: pointer;",
                                // If the Wizard button is clicked, switch to wizard view
                                onclick: move |_| {
                                    current_route.set(Some(text.to_string()));
                                },
                                "{icon}"
                            }
                            span { style: format!("display:{};margin-left:8px;", text_display),
                                button {
                                    class: "button-link",
                                    onclick: move |_| {
                                        info!("Clicked on {text}");
                                    },
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
                            button {
                                style: "background:none;border:none;font-size:16px;margin-right:20px;",
                                onclick: move |_| info!("Clicked top icon: {name}"),
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
                                button {
                                    style: "background:none;border:none;text-align:left;padding:8px 12px;width:100%;",
                                    onclick: move |_| user_menu_open.toggle(),
                                    "Profile"
                                }
                                button {
                                    style: "background:none;border:none;text-align:left;padding:8px 12px;width:100%;",
                                    onclick: move |_| user_menu_open.toggle(),
                                    "Settings"
                                }
                                button {
                                    style: "background:none;border:none;text-align:left;padding:8px 12px;width:100%;",
                                    onclick: move |_| async move {
                                        user_menu_open.toggle();
                                        info!("Logging out...");
                                        if let Ok(_) = log_out().await {
                                            use_navigator().push(Route::Login {});
                                        }
                                    },
                                    "Logout"
                                }
                            }
                        }
                    }
                }
                div { style: "flex:1;background:#bdc3c7;display:flex;align-items:center;justify-content:center;",
                    // Show the wizard if it is active, otherwise show default dashboard content
                    match current_route() {
                        Some(ref name) => {
                            match name.as_str() {
                                "" => rsx! {
                                    h5 { "Dashboard Content" }
                                },
                                "Domain" => rsx! {
                                    crate::components::domain::Domain {}
                                },
                                "Host" => rsx! {
                                    h5 { "Not implemented" }
                                },
                                "Network" => rsx! {
                                    h5 { "Not implemented" }
                                },
                                "Secret" => rsx! {
                                    h5 { "Not implemented" }
                                },
                                "Pool" => rsx! {
                                    h5 { "Not implemented" }
                                },
                                "Volume" => rsx! {
                                    h5 { "Not implemented" }
                                },
                                "Wizard" => rsx! {
                                    crate::components::wizard::WizardExample {}
                                },
                                _ => rsx! {
                                    h5 { "Not implemented" }
                                },
                            }
                        }
                        None => rsx! {
                            h5 { "Dashboard Content" }
                        },
                    }
                }
                div {
                    class: "footer",
                    style: "background:#ecf0f1;padding:10px;text-align:center;",
                    "© 2025 Rust Manager"
                }
            }
        }
    }
}
