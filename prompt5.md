Given a rust dioxus **fullstack** application with a **/dashboard** route and a **/wizard/example** route, show how to integrate the **wizard.rs** **component** into the dashboard so that when I click the **wizard** button on the dashboard the **wizard** code is rendered in the dashboard work area.

There should be no additional imports required.  For css styling use tailwindcss classes

# ./src/main.rs
```
#![allow(non_snake_case)]

use dioxus::prelude::*;

#[cfg(feature = "server")]
use backend::axum_server::launch_server;

mod backend;
mod components;
use components::{dashboard::Dashboard, login::Login, register::Register, user::User, wizard::WizardExample};

fn main() {
    #[cfg(feature = "web")]
    LaunchBuilder::web().launch(App);

    #[cfg(feature = "server")]
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
    // #[route("/")]
    // Home {},
    #[route("/")]
    Login {},
    #[route("/register")]
    Register {},
    #[route("/user")]
    User {},
    #[route("/dashboard")]
    Dashboard {},
    #[route("/wizard/example")]
    WizardExample {},
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

```

# ./src/components/dashboard.rs
```
use dioxus::prelude::*;
use tracing::info;

#[component]
pub fn Dashboard() -> Element {
    // Reactive signals
    info!("Rendering DashboardPage component");
    let mut collapsed = use_signal(|| false); // side‑menu collapse state
    let mut user_menu_open = use_signal(|| false); // user avatar drop‑down state
    let mut count = use_signal(|| 0);
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
        ("Wizard", "🧙🏿‍♂"), // TODO: Activate the wizard.rs code by clicking on this button
    ];

    let mut data = use_signal(|| String::from("Initial"));

    use_effect(move || {
        // This effect will rerun if 'data' changes
        info!("Data changed to: {}", data.read());
    });

    rsx! {
        document::Stylesheet { href: asset!("/assets/tailwind.css") }
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
                                onclick: move |_| info!("Clicked on {text}"),
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
                                    onclick: move |_| {
                                        user_menu_open.toggle();
                                        info!("Logging out...");
                                    },
                                    "Logout"
                                }
                            }
                        }
                    }
                }
                div { style: "flex:1;background:#bdc3c7;display:flex;align-items:center;justify-content:center;",
                    h5 { "Dashboard Content" }
                                    // span { "toggled is {collapsed()}" }
                // button { onclick: move |_| count += 1, // Increment the signal on click "Increment" }
                // 	p { "Count: {count}" }
                // }

                // div {
                // 	input {
                // 		value: "{data}",
                // 		oninput: move |event| data.set(event.value()),
                // 	}
                // 	p { "Current data: {data}" }
                // }
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
```

# ./src/components/wizard.rs
```
#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus_router::prelude::use_router;
use std::borrow::BorrowMut;

/// Holds the wizard data while the user steps through it
#[derive(Clone, Default)]
pub struct WizardState {
    pub name: String,
    pub number: Option<u8>,
    pub color: Option<String>,
    pub date: Option<String>,
}

#[component]
pub fn WizardExample() -> Element {
    let state = use_signal(|| WizardState::default());
    let step = use_signal(|| 1usize);
    let _router = use_router(); // keep router alive for future navigation

    // Validation helpers
    let is_step1_valid = !state().name.is_empty();
    let is_step2_valid = state().number.is_some();
    let is_step3_valid = state().color.is_some();
    let is_step4_valid = state().date.is_some();

    let forward_disabled = match step() {
        1 => !is_step1_valid,
        2 => !is_step2_valid,
        3 => !is_step3_valid,
        4 => !is_step4_valid,
        _ => false,
    };

    // Submit action – POST to the stub backend endpoint
    let submit = move |_| {
        let data = state().clone();
        // use_future(async move {
        //     let _ = fetch(
        //         "/wizard/submit",
        //         RequestInit {
        //             method: "POST",
        //             headers: {
        //                 "Content-Type":  "application/json",
        //             },
        //             body: Some(serde_json::to_string(&data).unwrap()),
        //             ..Default::default()
        //         },
        //     )
        //     .await;
        // });
    };

    rsx! {
        div { class:"p-4 max-w-md mx-auto",
            // ---------- Step UI ----------
            match step() {
                1 => rsx!{
                    h2 { class:"text-xl font-bold mb-2", "Step 1: Enter a name" }
                    input {
                        class:"border rounded w-full p-2 mb-4",
                        placeholder:"Name",
                        value: "{state().name}",
                        oninput: move |e| state().name = e.value().clone(),
                    }
                },
                2 => rsx!{
                    h2 { class:"text-xl font-bold mb-2", "Step 2: Enter a number" }
                    select {
                        class:"border rounded w-full p-2 mb-4",
                        oninput: move |e| state().number = e.value().parse::<u8>().ok(),
                        value: "{state().number.unwrap_or(0)}",
                        for i in 1..=10 {
                            option { value: "{i}", "{i}" }
                        },
                    }
                },
                3 => rsx!{
                    h2 { class:"text-xl font-bold mb-2", "Step 3: Select a color" }
                    select {
                        class:"border rounded w-full p-2 mb-4",
                        oninput: move |e| state().color = Some(e.value().clone()),
                        value: state().color.clone().unwrap_or_default(),
                        for c in [
                            "Red","Orange","Yellow","Green","Blue","Indigo","Violet"
                        ].iter() {
                            option { value: "{c}", "{c}" }
                        },
                    }
                },
                4 => rsx!{
                    h2 { class:"text-xl font-bold mb-2", "Step 4: Select a date" }
                    input {
                        class:"border rounded w-full p-2 mb-4",
                        r#type:"date",
                        oninput: move |e| state().date = Some(e.value().clone()),
                        value: state().date.clone().unwrap_or_default(),
                    }
                },
                5 => rsx!{
                    h2 { class:"text-xl font-bold mb-2", "Step 5: Review" }
                    ul { class:"mb-4",
                        li { "Name: {state().name}" }
                        li { "Number: {state().number.unwrap_or(0)}" }
                        li { "Color: {state().color.clone().unwrap_or_default()}" }
                        li { "Date: {state().date.clone().unwrap_or_default()}" }
                    }
                },
                _ => rsx!{},
            }
            // ---------- Navigation ----------
            div { class:"flex space-x-2 mt-4",
                button {
                    class:"px-4 py-2 bg-gray-300 rounded",
                    onclick: move |_| {
                      let mut binding = step();
                      let mut step_val = binding.borrow_mut();
                      if *step_val > 1 {
                        *step_val -= 1;
                      }
                    },
                    disabled: step() <= 1,
                    "Back"
                }
                button {
                    class:"px-4 py-2 bg-blue-500 text-white rounded",
                    onclick: move |_| { 
                      let mut binding = step();
                      let mut step_val = binding.borrow_mut();
                      if *step_val > 1 {
                        *step_val += 1;
                      }
                    },
                    disabled: forward_disabled,
                    "Next"
                }
                button {
                    class:"px-4 py-2 bg-red-500 text-white rounded",
                    onclick: move |_| {
                        // state().replace(WizardState::default());
                        let mut binding = step();
                        let mut step_val = binding.borrow_mut();
                        *step_val = 1;;
                    },
                    "Cancel"
                }
                if step() == 5 {
                    button {
                        class:"px-4 py-2 bg-green-500 text-white rounded",
                        onclick: submit,
                        "Submit"
                    }
                }
            }
        }
    }
}
```