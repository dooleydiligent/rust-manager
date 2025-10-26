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
    ];

    let mut data = use_signal(|| String::from("Initial"));

    use_effect(move || {
        // This effect will rerun if 'data' changes
        info!("Data changed to: {}", data.read());
    });

    rsx! {
      document::Stylesheet { href: asset!("/assets/tailwind.css") },
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
								span { style: "font-size:20px;cursor: pointer;", 
                      onclick: move |_| info!("Clicked on {text}"),
                  "{icon}" }
								span { style: format!("display:{};margin-left:8px;", text_display),
									button {
                    class: "button-link", 
                    onclick: move |_| {
                      info!("Clicked on {text}");
                      // get(api::get_domains)
                    },
                  "{text}" }
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
