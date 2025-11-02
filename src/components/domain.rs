// src/components/domain.rs
use crate::backend::server_functions::get_domain;
use dioxus::hooks::use_resource;
use dioxus::prelude::*;
use serde::Deserialize;

/// The shape of the JSON that the server returns
#[derive(Debug, Clone, Deserialize)]
pub struct DomainResponse {
    pub name: String,
    pub full_name: String,
    pub uuid: String,
    pub state: String,
    pub time: u64,
    pub memory: u64,
    pub max_mem: u64,
}

#[component]
pub fn Domain() -> Element {
    // 1️⃣  Kick off the async request as soon as the component mounts
    let domain_result = use_resource(|| async {
        // Uncomment to use real API call:
        get_domain().await
        // For testing, return dummy data:
        // Ok(DomainResponse { test: true })
    });

    // --------------------------------------------------------------------
    // 2️⃣  Render the page layout + async‑state panel
    // --------------------------------------------------------------------
    rsx! {
        // document::Stylesheet { href: asset!("/assets/tailwind.css") },
        div { class: "w-full h-full",
            div { class: "screen flex justify-center items-start bg-slate-50",
                // --------------------------------------------------------------------
                // 3️⃣  Show loading/data state based on the resource value
                // --------------------------------------------------------------------
                {
                    match domain_result.read().as_ref() {
                        None => rsx! {
                            div { class: "mt-4 p-4 bg-yellow-100 rounded", "Loading domain info..." }
                        },
                        Some(Ok(response)) => {
                            match serde_json::from_str::<Vec<DomainResponse>>(response) {
                                Ok(domains) if domains.is_empty() => {
                                    rsx! {
                                        div { class: "mt-4 p-4 bg-yellow-100 rounded", "No domains found" }
                                    }
                                }
                                Ok(domains) => rsx! {
                                    div { class: "mt-4 p-4 bg-green-100 rounded",
                                        table { class: "min-w-full",
                                            thead {
                                                tr { class: "border-b",
                                                    th { class: "text-left p-2", "Name" }
                                                    th { class: "text-left p-2", "State" }
                                                    th { class: "text-left p-2", "Total Memory" }
                                                    th { class: "text-left p-2", "Max Memory" }
                                                }
                                            }
                                            tbody {
                                                for domain in domains.iter() {
                                                    tr { 
                                                        key: "{domain.uuid}",
                                                        td { class: "p-2", "{domain.name}" }
                                                        td { class: "p-2", "{domain.state}" }
                                                        td { class: "p-2", "{domain.memory} MB" }
                                                        td { class: "p-2", "{domain.max_mem} MB" }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                },
                                Err(e) => rsx! {
                                    div { class: "mt-4 p-4 bg-red-100 rounded", "Error parsing response: {e}" }
                                },
                            }
                        }
                        Some(Err(err)) => rsx! {
                            div { class: "mt-4 p-4 bg-red-100 rounded", "Error: {err}" }
                        },
                    }
                }
            }
        }
    }
}
