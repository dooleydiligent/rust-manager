use dioxus::prelude::*;

use crate::{backend::server_functions::log_in, Route};

#[component]
pub fn Login() -> Element {
    let mut username = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut error_msg = use_signal(|| String::new());
    let navigator = use_navigator();

    rsx!(
      document::Stylesheet { href: asset!("/assets/tailwind.css") },
      div { class: "screen flex justify-center items-center bg-slate-50",
        div { class: "border-solid border-2 border-slate-100 rounded-lg px-2 py-5 w-1/2",
          div { class: "text-center text-3xl", "Login" }
          if !error_msg.to_string().is_empty() {
            div { class: "bg-rose-100 text-rose-600 py-1 px-2 rounded-lg my-3",
              " {error_msg}"
            }
          }
          div { class: "grid grid-cols-2 gap-0",
            div { class: "bg-blue-200 p-4 text-lg py-5", "username: " },
            div { class: "bg-blue-200 p-5 text-lg py-5", 
              input {
                class: "w-3xs rounded-lg px-1 py-1",
                autofocus: "true",
                r#type: "text",
                value: username,
                oninput: move |e| username.set(e.value()),
              }
            }
          }
          div { class: "grid grid-cols-2 gap-0",
            div { class: "bg-blue-200 p-4 text-lg py-5", "password: " },
            div { class: "bg-blue-200 p-5 text-lg py-5",
              input {
                class: "w-full rounded-lg px-2 py-1",
                r#type: "password",
                value: password,
                oninput: move |e| password.set(e.value()),
              }
            }
          }
          button {
            class: "bg-sky-500 text-slate-50 px-3 py-2 rounded-lg w-full my-5 hover:bg-sky-600",
            onclick: move |_| async move {
                match log_in(username(), password()).await {
                    Ok(_) => {
                        match navigator.push(Route::Dashboard {}) {
                            Some(_) => {}
                            None => {}
                        }
                    }
                    Err(e) => {
                        error_msg
                            .set(e.to_string().split(":").collect::<Vec<&str>>()[1].to_string())
                    }
                }
            },
            "Login"
          }
          div {
            "Don't have an account? "
            Link { to: Route::Register {}, class: "text-sky-400", "register now" }
          }
        }
      }
    )
}
