// src/components/domain.rs
use crate::backend::server_functions::get_domain;
use dioxus::hooks::use_resource;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::info;

/// The shape of the JSON that the server returns
#[derive(Debug, Clone, Deserialize, Serialize)]
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
    // Track both the selected domain keys (name, uuid) and the full domain object
    let mut selected = use_signal(|| Option::<(String, String)>::None); // (name, uuid)
    let mut selected_domain = use_signal(|| Option::<DomainResponse>::None);
    // 1️⃣  Kick off the async request as soon as the component mounts
    let domain_result = use_resource(|| async {
        // Uncomment to use real API call:
        get_domain().await
        // For testing, return dummy data:
        // Ok(DomainResponse { test: true })
    });

    // Sorting state: which column and whether descending
    let mut sort_col = use_signal(|| String::new()); // e.g. "name", "state", "memory", "max_mem"
    let mut sort_desc = use_signal(|| false);
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
                                Ok(domains) => {
                                    let mut sorted_domains = domains.clone();
                                    match sort_col().as_str() {
                                        "name" => {
                                            if sort_desc() {
                                                sorted_domains
                                                    .sort_by(|a, b| {
                                                        b.name.to_lowercase().cmp(&a.name.to_lowercase())
                                                    });
                                            } else {
                                                sorted_domains
                                                    .sort_by(|a, b| {
                                                        a.name.to_lowercase().cmp(&b.name.to_lowercase())
                                                    });
                                            }
                                        }
                                        "state" => {
                                            if sort_desc() {
                                                sorted_domains
                                                    .sort_by(|a, b| {
                                                        b.state.to_lowercase().cmp(&a.state.to_lowercase())
                                                    });
                                            } else {
                                                sorted_domains
                                                    .sort_by(|a, b| {
                                                        a.state.to_lowercase().cmp(&b.state.to_lowercase())
                                                    });
                                            }
                                        }
                                        "memory" => {
                                            if sort_desc() {
                                                sorted_domains.sort_by(|a, b| b.memory.cmp(&a.memory));
                                            } else {
                                                sorted_domains.sort_by(|a, b| a.memory.cmp(&b.memory));
                                            }
                                        }
                                        "max_mem" => {
                                            if sort_desc() {
                                                sorted_domains.sort_by(|a, b| b.max_mem.cmp(&a.max_mem));
                                            } else {
                                                sorted_domains.sort_by(|a, b| a.max_mem.cmp(&b.max_mem));
                                            }
                                        }
                                        _ => {}
                                    }
                                    rsx! {
                                        div { class: "mt-4 p-4 bg-green-100 rounded ",
                                            table { class: "min-w-full",
                                                thead {
                                                    tr { class: "border-b",
                                                        th { class: "text-left p-2", "Sel" }
                                                        th { class: "text-left p-2 ",
                                                            // Name column header with sort icon
                                                            button {
                                                                class: format!(
                                                                    "flex items-center gap-2 cursor-pointer {}",
                                                                    if sort_col() == "name" {
                                                                        if sort_desc() { "text-sky-600" } else { "text-sky-800" }
                                                                    } else {
                                                                        "text-slate-700"
                                                                    },
                                                                ),
                                                                onclick: move |_| {
                                                                    if sort_col() == "name" {
                                                                        sort_desc.set(!sort_desc());
                                                                    } else {
                                                                        sort_col.set("name".to_string());
                                                                        sort_desc.set(false);
                                                                    }
                                                                },
                                                                "Name"
                                                                span { class: "text-sm",
                                                                    {
                                                                        format!(
                                                                            "{}",
                                                                            if sort_col() == "name" {
                                                                                if sort_desc() { "▼" } else { "▲" }
                                                                            } else {
                                                                                "⇵"
                                                                            },
                                                                        )
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        th { class: "text-left p-2 ",
                                                            button {
                                                                class: format!(
                                                                    "flex items-center gap-2 cursor-pointer {}",
                                                                    if sort_col() == "state" {
                                                                        if sort_desc() { "text-sky-600" } else { "text-sky-800" }
                                                                    } else {
                                                                        "text-slate-700"
                                                                    },
                                                                ),
                                                                onclick: move |_| {
                                                                    if sort_col() == "state" {
                                                                        sort_desc.set(!sort_desc());
                                                                    } else {
                                                                        sort_col.set("state".to_string());
                                                                        sort_desc.set(false);
                                                                    }
                                                                },
                                                                "State"
                                                                span { class: "text-sm",
                                                                    {
                                                                        format!(
                                                                            "{}",
                                                                            if sort_col() == "state" {
                                                                                if sort_desc() { "▼" } else { "▲" }
                                                                            } else {
                                                                                "⇵"
                                                                            },
                                                                        )
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        th { class: "text-left p-2 ",
                                                            button {
                                                                class: format!(
                                                                    "flex items-center gap-2 cursor-pointer {}",
                                                                    if sort_col() == "memory" {
                                                                        if sort_desc() { "text-sky-600" } else { "text-sky-800" }
                                                                    } else {
                                                                        "text-slate-700"
                                                                    },
                                                                ),
                                                                onclick: move |_| {
                                                                    if sort_col() == "memory" {
                                                                        sort_desc.set(!sort_desc());
                                                                    } else {
                                                                        sort_col.set("memory".to_string());
                                                                        sort_desc.set(false);
                                                                    }
                                                                },
                                                                "Total Memory"
                                                                span { class: "text-sm",
                                                                    {
                                                                        format!(
                                                                            "{}",
                                                                            if sort_col() == "memory" {
                                                                                if sort_desc() { "▼" } else { "▲" }
                                                                            } else {
                                                                                "⇵"
                                                                            },
                                                                        )
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        th { class: "text-left p-2 ",
                                                            button {
                                                                class: format!(
                                                                    "flex items-center gap-2 cursor-pointer {}",
                                                                    if sort_col() == "max_mem" {
                                                                        if sort_desc() { "text-sky-600" } else { "text-sky-800" }
                                                                    } else {
                                                                        "text-slate-700"
                                                                    },
                                                                ),
                                                                onclick: move |_| {
                                                                    if sort_col() == "max_mem" {
                                                                        sort_desc.set(!sort_desc());
                                                                    } else {
                                                                        sort_col.set("max_mem".to_string());
                                                                        sort_desc.set(false);
                                                                    }
                                                                },
                                                                "Max Memory"
                                                                span { class: "text-sm",
                                                                    {
                                                                        format!(
                                                                            "{}",
                                                                            if sort_col() == "max_mem" {
                                                                                if sort_desc() { "▼" } else { "▲" }
                                                                            } else {
                                                                                "⇵"
                                                                            },
                                                                        )
                                                                    }
                                                                }
                                                            }
                                                        }
                                                        th { class: "text-left p-2", "Details" }
                                                    }
                                                }
                                                tbody {
                                                    {
                                                        sorted_domains
                                                            .iter()
                                                            .enumerate()
                                                            .map(|(idx, domain)| {
                                                                let domain = domain.clone();
                                                                let domain_for_closure = domain.clone();
                                                                rsx! {
                                                                    tr { key: "{domain.uuid}",
                                                                        td { class: "p-2",
                                                                            input {
                                                                                r#type: "radio",
                                                                                name: "select_domain",
                                                                                value: "{domain.name}",
                                                                                onchange: move |_| {
                                                                                    selected.set(Some((domain_for_closure.name.clone(), domain_for_closure.uuid.clone())));
                                                                                    selected_domain.set(Some(domain_for_closure.clone()));
                                                                                    info!("Selected domain: {} ({})", domain_for_closure.name, domain_for_closure.uuid);
                                                                                },
                                                                            }
                                                                        }
                                                                        td { class: "p-2", "{domain.name}" }
                                                                        td { class: "p-2", "{domain.state}" }
                                                                        td { class: "p-2 text-right", "{domain.memory / 1024} MB" }
                                                                        td { class: "p-2 text-right", "{domain.max_mem / 1024} MB" }
                                                                        {
                                                                            if idx == 0 {
                                                                                rsx! {
                                                                                    td {
                                                                                        class: "p-2 bg-gray-50 font-mono whitespace-pre text-left overflow-x-auto border-l",
                                                                                        rowspan: "{sorted_domains.len()}",
                                                                                        {
                                                                                            if let Some(selected_obj) = selected_domain() {
                                                                                                match serde_json::to_string_pretty(&selected_obj) {
                                                                                                    Ok(json) => json,
                                                                                                    Err(_) => "Error formatting JSON".to_string(),
                                                                                                }
                                                                                            } else {
                                                                                                "Select a domain to view details".to_string()
                                                                                            }
                                                                                        }
                                                                                    }
                                                                                }
                                                                            } else {
                                                                                rsx! {}
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            })
                                                    }
                                                }
                                                tfoot {
                                                    tr { class: "border-t",
                                                        td { class: "p-2", colspan: "2",
                                                            div { class: "flex gap-2",
                                                                button {
                                                                    class: "px-3 py-1 bg-green-500 text-white rounded hover:bg-green-600 transition-colors",
                                                                    onclick: move |_| {
                                                                        info!("Add new domain");
                                                                    },
                                                                    "Add"
                                                                }
                                                                button {
                                                                    class: format!(
                                                                        "px-3 py-1 bg-blue-500 text-white rounded hover:bg-blue-600 transition-colors {}",
                                                                        if selected().is_none() { "opacity-50 cursor-not-allowed" } else { "" },
                                                                    ),
                                                                    disabled: selected().is_none(),
                                                                    onclick: move |_| {
                                                                        if let Some((name, uuid)) = selected() {
                                                                            info!("Edit domain: {} ({})", name, uuid);
                                                                        }
                                                                    },
                                                                    "Edit"
                                                                }
                                                                button {
                                                                    class: format!(
                                                                        "px-3 py-1 bg-red-500 text-white rounded hover:bg-red-600 transition-colors {}",
                                                                        if selected().is_none() { "opacity-50 cursor-not-allowed" } else { "" },
                                                                    ),
                                                                    disabled: selected().is_none(),
                                                                    onclick: move |_| {
                                                                        if let Some((name, uuid)) = selected() {
                                                                            info!("Delete domain: {} ({})", name, uuid);
                                                                        }
                                                                    },
                                                                    "Delete"
                                                                }
                                                            }
                                                        }
                                                        td { class: "p-2", colspan: "2",
                                                            {
                                                                format!(
                                                                    "{}",
                                                                    selected().map(|(name, _)| name).unwrap_or_else(|| "None".to_string()),
                                                                )
                                                            }
                                                        }
                                                        td { class: "p-2 text-right", "Count:" }
                                                        td { class: "p-2 text-right", "{sorted_domains.len()}" }
                                                        td { class: "p-2" } // Empty cell for details column
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
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
