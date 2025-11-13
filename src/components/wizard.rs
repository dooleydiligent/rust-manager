#![allow(non_snake_case)]

use dioxus::prelude::*;
// use dioxus_router::prelude::use_router;
use std::borrow::BorrowMut;
use std::fmt;
use tracing::info;

use serde::{Deserialize, Serialize};
/// Holds the wizard data while the user steps through it
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WizardState {
    pub name: String,
    pub number: Option<u8>,
    pub color: Option<String>,
    pub date: Option<String>,
}

impl fmt::Display for WizardState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Convert each field to a printable string
        let name = &self.name.as_str();
        let number = match self.number {
            Some(n) => n.to_string(),
            None => "<none>".into(),
        };
        let color = self.color.as_deref().unwrap_or("<none>");
        let date = self.date.as_deref().unwrap_or("<none>");

        write!(
            f,
            "WizardState {{\n  name: {},\n  number: {},\n  color: {},\n  date: {}\n}}",
            name, number, color, date
        )
    }
}

#[component]
pub fn WizardExample() -> Element {
    let mut state = use_signal(|| WizardState::default());
    let mut step = use_signal(|| 1usize);
    // let _router = use_router(); // keep router alive for future navigation

    // Validation helpers
    let is_step1_valid = !state().name.is_empty();
    let is_step2_valid = state().number.is_some();
    let is_step3_valid = state().color.is_some();
    let is_step4_valid = state().date.is_some();
    // info!(
    //     "Rendering WizardExample component at step {}, {}",
    //     step(),
    //     state()
    // );
    let forward_disabled = match step() {
        1 => !is_step1_valid,
        2 => !is_step2_valid,
        3 => !is_step3_valid,
        4 => !is_step4_valid,
        _ => false,
    };

    // Submit action – POST to the stub backend endpoint
    let submit = move |_| {
        let _data = state().clone();
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
        div { class: "p-4 max-w-md mx-auto",
            // ---------- Step UI ----------
            match step() {
                1 => rsx! {
                    h2 { class: "text-xl font-bold mb-2", "Step 1: Enter a name" }
                    input {
                        class: "border rounded w-full p-2 mb-4",
                        placeholder: "Name",
                        value: "{state().name}",
                        oninput: move |e| {
                          state.write().name = e.value().clone();
                          // is_step1_valid = !state().name.is_empty();
                          // forward_disabled = is_step1_valid;
                          // info!("name is now {}, valid: {}", state().name, is_step1_valid);
                        },
                    }
                },
                2 => rsx! {
                    h2 { class: "text-xl font-bold mb-2", "Step 2: Enter a number" }
                    select {
                        class: "border rounded w-full p-2 mb-4",
                        oninput: move |e| {
                          state.write().number = e.value().parse::<u8>().ok();
                          // info!("number is now {:?}", state().number);
                        },
                        value: "{state().number.unwrap_or(0)}",
                        for i in 1..=10 {
                            option { value: "{i}", selected: i == state().number.unwrap_or(0), "{i}" }
                        }
                    }
                },
                3 => rsx! {
                    h2 { class: "text-xl font-bold mb-2", "Step 3: Select a color" }
                    select {
                        class: "border rounded w-full p-2 mb-4",
                        oninput: move |e| state.write().color = Some(e.value().clone()),
                        value: state().color.clone().unwrap_or_default(),
                        for c in ["Red", "Orange", "Yellow", "Green", "Blue", "Indigo", "Violet"].iter() {
                            option { value: "{c}", selected: *c == state().color.clone().unwrap_or_default(), "{c}" }
                        }
                    }
                },
                4 => rsx! {
                    h2 { class: "text-xl font-bold mb-2", "Step 4: Select a date" }
                    input {
                        class: "border rounded w-full p-2 mb-4",
                        r#type: "date",
                        oninput: move |e| state.write().date = Some(e.value().clone()),
                        value: state().date.clone().unwrap_or_default(),
                    }
                },
                5 => rsx! {
                    h2 { class: "text-xl font-bold mb-2", "Step 5: Review" }
                    ul { class: "mb-4",
                        li { "Name: {state().name}" }
                        li { "Number: {state().number.unwrap_or(0)}" }
                        li { "Color: {state().color.clone().unwrap_or_default()}" }
                        li { "Date: {state().date.clone().unwrap_or_default()}" }
                    }
                },
                _ => rsx! {},
            }
            // ---------- Navigation ----------
            div { class: "flex space-x-2 mt-4",
                button {
                    class: "px-4 py-2 bg-gray-300 rounded",
                    onclick: move |_| {
                        info!("Clicked Back");
                        let step_val = step.borrow_mut();
                        *step_val -= 1;
                    },
                    disabled: step() <= 1,
                    "Back"
                }
                button {
                    class: "px-4 py-2 bg-blue-500 text-white rounded disabled:bg-gray-400",
                    onclick: move |_| {
                        // let mut binding = step();
                        let step_val = step.borrow_mut();
                        *step_val += 1;
                        info!("Clicked Next.  Step is {}", *step_val);
                    },
                    disabled: forward_disabled,
                    "Next"
                }
                button {
                    class: "px-4 py-2 bg-red-500 text-white rounded",
                    onclick: move |_| {
                        info!("Clicked Cancel");
                        let mut binding = step();
                        let step_val = binding.borrow_mut();
                        *step_val = 1;
                    },
                    "Cancel"
                }
                if step() == 5 {
                    button {
                        class: "px-4 py-2 bg-green-500 text-white rounded",
                        onclick: submit,
                        "Submit"
                    }
                }
            }
        }
    }
}
