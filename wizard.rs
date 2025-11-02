// ──────────────────────────────────────────────────────────────────────────────
// wizard.rs
// ──────────────────────────────────────────────────────────────────────────────
use axum::{
    extract::{Form, Query},
    response::{Html, IntoResponse, Redirect},
};
use std::fmt;

use axum_session::Session;
use axum_session_sqlx::SessionSqlitePool;

use serde::{Deserialize, Serialize};
/// The data that will be stored in the session while the wizard is running.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct WizardData {
    name: Option<String>,
    number: Option<u32>,
    color: Option<String>,
    date: Option<String>,
}

impl fmt::Display for WizardData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Convert each field to a printable string
        let name = self.name.as_deref().unwrap_or("<none>");
        let number = match self.number {
            Some(n) => n.to_string(),
            None => "<none>".into(),
        };
        let color = self.color.as_deref().unwrap_or("<none>");
        let date = self.date.as_deref().unwrap_or("<none>");

        write!(
            f,
            "WizardData {{\n  name: {},\n  number: {},\n  color: {},\n  date: {}\n}}",
            name, number, color, date
        )
    }
}

/// The query string that tells the handlers which step we are on.
#[derive(Debug, Deserialize)]
pub struct WizardQuery {
    step: Option<u32>, // 1‑5
}

/// The form payload for every step.  Only one of the fields will be filled
/// for a given step – the rest will be `None`.  All fields are optional
/// so that we can use a single struct for every POST request.
#[derive(Debug, Deserialize)]
pub struct WizardForm {
    action: String, // "next", "back", "cancel", "submit"
    name: Option<String>,
    number: Option<u32>,
    color: Option<String>,
    date: Option<String>,
}

/// -----------------------------------------------------------------------------
/// GET  /wizard/example
/// -----------------------------------------------------------------------------
/// Render the appropriate step form.
pub async fn wizard_get(
    session: Session<SessionSqlitePool>,
    query: Query<WizardQuery>,
) -> impl IntoResponse {
    let step = query.step.unwrap_or(1).clamp(1, 5);

    // Load any data that we already have.
    let data: WizardData = session.get::<WizardData>("wizard").unwrap_or_default();

    let selected_name = data.name.as_deref().unwrap_or("");
    let selected_number = data
        .number
        .map(|n| n.to_string())
        .unwrap_or("".into())
        .parse::<u32>()
        .unwrap_or(0);
    let selected_color = data.color.as_deref().unwrap_or("");
    let selected_date = data.date.as_deref().unwrap_or("");

    println!(
        "Wizard GET: step={}, data={}, number={}",
        step, data, selected_number
    );
    let html = match step {
        1 => {
            format!(
                r#"
                <h2>Step 1: Enter your name</h2>
                <form action="/wizard/example?step=1" method="post">
                    <label>Name: <input type="text" name="name" required value="{name}" /></label><br/>
                    <button type="submit" name="action" value="next">Next</button>
                    <button type="cancel" name="action" value="cancel">Cancel</button>
                </form>
                "#,
                name = selected_name,
            )
        }
        2 => {
            format!(
                r#"
                <h2>Step 2: Pick a number (1‑10)</h2>
                <form action="/wizard/example?step=2" method="post">
                    <label>Number:
                        <select name="number" required>
                            {}
                        </select>
                    </label><br/>
                    <button type="submit" name="action" value="next">Next</button>
                    <button type="submit" name="action" value="back">Back</button>
                    <button type="cancel" name="action" value="cancel">Cancel</button>
                </form>
                "#,
                (1..=10)
                    .map(|n| {
                        let selected = if n == selected_number { "selected" } else { "" };
                        format!(r#"<option value="{n}" {selected} >{n}</option>"#)
                    })
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        }
        3 => {
            let colors = [
                ("red", "Red"),
                ("orange", "Orange"),
                ("yellow", "Yellow"),
                ("green", "Green"),
                ("blue", "Blue"),
                ("indigo", "Indigo"),
                ("violet", "Violet"),
            ];
            format!(
                r#"
                <h2>Step 3: Pick a color</h2>
                <form action="/wizard/example?step=3" method="post">
                    <label>Color:
                        <select name="color" required>
                            {}
                        </select>
                    </label><br/>
                    <button type="submit" name="action" value="next">Next</button>
                    <button type="submit" name="action" value="back">Back</button>
                    <button type="cancel" name="action" value="cancel">Cancel</button>
                </form>
                "#,
                colors
                    .iter()
                    .map(|(code, name)| {
                        let selected = if *code == selected_color {
                            "selected"
                        } else {
                            ""
                        };
                        format!(r#"<option value="{code}" {selected} >{name}</option>"#)
                    })
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        }
        4 => {
            format!(
                r#"
                <h2>Step 4: Pick a date</h2>
                <form action="/wizard/example?step=4" method="post">
                    <label>Date: <input type="date" name="date" required value="{date}"/></label><br/>
                    <button type="submit" name="action" value="next">Next</button>
                    <button type="submit" name="action" value="back">Back</button>
                    <button type="cancel" name="action" value="cancel">Cancel</button>
                </form>
                "#,
                date = data.date.as_deref().unwrap_or(""),
            )
        }
        5 => {
            // Review page – show the data that has already been collected.
            format!(
                r#"
                <h2>Step 5: Review your answers</h2>
                <p><strong>Name:</strong> {name}</p>
                <p><strong>Number:</strong> {number}</p>
                <p><strong>Color:</strong> {color}</p>
                <p><strong>Date:</strong> {date}</p>
                <form action="/wizard/example?step=5" method="post">
                    <button type="submit" name="action" value="next">Finish</button>
                    <button type="submit" name="action" value="back">Back</button>
                    <button type="cancel" name="action" value="cancel">Cancel</button>
                </form>
                "#,
                name = selected_name,
                number = selected_number,
                color = selected_color,
                date = selected_date,
            )
        }
        _ => unreachable!(),
    };

    Html(html).into_response()
}

/// -----------------------------------------------------------------------------
/// POST /wizard/example
/// -----------------------------------------------------------------------------
/// Handle the form submission, update the session and redirect to the next
/// (or previous) step.  When the wizard is cancelled or finished the data is
/// cleared from the session.
pub async fn wizard_post(
    session: Session<SessionSqlitePool>,
    query: Query<WizardQuery>,
    form: Form<WizardForm>,
) -> impl IntoResponse {
    // Which step are we in?
    println!("Wizard POST received: {:?}", form);
    let current_step = query.step.unwrap_or(1).clamp(1, 5);
    println!("Wizard POST Step: {}", current_step);
    let action = form.action.to_lowercase();
    println!("Wizard POST Action: {}", action);

    // Helper to read or create wizard data in the session
    let mut data: WizardData = session.get::<WizardData>("wizard").unwrap_or_default();

    println!(
        "Wizard POST: step={}, action={}, data={}",
        current_step, action, data
    );
    // ---------- Cancel ----------
    if action == "cancel" {
        session.remove("wizard");
        return Redirect::to("/wizard/example?step=1");
    }

    // ---------- Back ----------
    if action == "back" {
        let prev_step = if current_step > 1 {
            current_step - 1
        } else {
            1
        };
        return Redirect::to(&format!("/wizard/example?step={}", prev_step));
    }

    // ---------- Validation & Store ----------
    // Store only the field that belongs to the current step
    match current_step {
        1 => {
            if let Some(name) = form.name.clone() {
                data.name = Some(name.trim().to_string());
            } else {
                return Redirect::to(&format!("/wizard/example?step={}", current_step));
            }
        }
        2 => {
            if let Some(number) = form.number {
                data.number = Some(number);
            } else {
                return Redirect::to(&format!("/wizard/example?step={}", current_step));
            }
        }
        3 => {
            if let Some(color) = form.color.clone() {
                data.color = Some(color);
            } else {
                return Redirect::to(&format!("/wizard/example?step={}", current_step));
            }
        }
        4 => {
            if let Some(date) = form.date.clone() {
                data.date = Some(date);
            } else {
                return Redirect::to(&format!("/wizard/example?step={}", current_step));
            }
        }
        _ => {}
    }

    // Persist the updated data back into the session
    session.set("wizard", data.clone());

    // ---------- Submit ----------
    if action == "submit" && current_step == 5 {
        // In a real app you'd do something useful with the data here.
        // For this PoC we just print it to the console.
        println!("Wizard finished!  Data: {:#?}", data);
        session.remove("wizard");
        // Redirect somewhere sensible after completion
        return Redirect::to("/dashboard");
    }

    // ---------- Next ----------
    let next_step = if current_step < 5 {
        current_step + 1
    } else {
        5
    };
    Redirect::to(&format!("/wizard/example?step={}", next_step))
}
