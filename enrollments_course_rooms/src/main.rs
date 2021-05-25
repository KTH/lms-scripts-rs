mod canvas_api;
mod kopps_api;
use csv::Writer;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use dotenv::dotenv;
use serde::Serialize;
use std::env;

#[macro_use]
extern crate log;
extern crate pretty_env_logger;

#[derive(Serialize)]
struct Row<'a> {
    name: &'a str,
    course: &'a str,
    role: &'a str,
    section: &'a str,
    mail1: &'a str,
    mail2: &'a str,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    dotenv().ok();

    let kopps_api_url = env("KOPPS_API_URL");
    let canvas_api_url = env("CANVAS_API_URL");
    let canvas_api_token = env("CANVAS_API_TOKEN");
    let (year_term, period) = prompt_year_term_period();

    let file_path = format!("enrollments-courserooms-{}-{}.csv", year_term, period);
    let course_rounds = kopps_api::get_course_rounds(&kopps_api_url, &year_term, &period)
        .filter(|round| round.first_period == format!("{}{}", year_term, period));

    let mut wtr = Writer::from_path(file_path)?;

    for round in course_rounds {
        let sis_id = kopps_api::make_sis_id(&round);
        let enrollments =
            canvas_api::get_enrollments(canvas_api_url.clone(), canvas_api_token.clone(), &sis_id)?;

        for enrollment in enrollments.into_iter() {
            wtr.serialize(Row {
                course: &format!("{} {}", sis_id, round.first_period),
                name: &enrollment.user.sortable_name.unwrap_or("??".to_string()),
                role: &enrollment.role,
                section: &enrollment.sis_section_id.unwrap_or("??".to_string()),
                mail1: &format!(
                    "{}@kth.se",
                    enrollment.sis_user_id.unwrap_or("??".to_string())
                ),
                mail2: &enrollment.user.login_id.unwrap_or("??".to_string()),
            })?;
        }
    }

    Ok(())
}

fn env(key: &str) -> String {
    match env::var(key) {
        Ok(val) => val,
        Err(_) => {
            error!("Environmental variable {} not defined", key);
            panic!("Environmental variable {} not defined", key);
        }
    }
}

// Prompt the year, term and period
fn prompt_year_term_period() -> (String, String) {
    let year: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Write a year")
        .with_initial_text("2020")
        .interact_text()
        .expect("Failed to prompt year");

    let terms = vec!["VT (Spring)", "HT (Fall)"];
    let term_selection = Select::with_theme(&ColorfulTheme::default())
        .items(&terms)
        .default(0)
        .interact()
        .expect("Failed to get the term");

    let (term, periods) = match term_selection {
        0 => ("1", vec!["P3", "P4", "P5"]),
        1 => ("2", vec!["P0", "P1", "P2"]),
        _ => {
            panic!("Unexpected value for term option");
        }
    };

    let period_selection = Select::with_theme(&ColorfulTheme::default())
        .items(&periods)
        .default(0)
        .interact()
        .expect("Failed to get the period");

    let period = periods[period_selection];

    (format!("{}{}", year, term), period.to_string())
}
