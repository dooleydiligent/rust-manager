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

// #[component]
// fn Home() -> Element {
//     rsx!(
//         div { class: "text-sky-500", "Home" }
//     )
// }

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