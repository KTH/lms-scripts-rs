mod akt_api;
mod canvas_api;
mod kopps_api;
use chrono::NaiveDate;
use csv::Writer;
use dialoguer::{theme::ColorfulTheme, Input, Password, Select};
use dotenv::dotenv;
use serde::Serialize;
use std::env;

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
    dotenv().ok();

    let option = prompt_choice();

    if option == 0 {
        list_course_room_enrollments()
    } else {
        list_exam_room_enrollments()
    }
}

fn list_course_room_enrollments() -> Result<(), Box<dyn std::error::Error>> {
    let kopps_api_url = env("KOPPS_API_URL");
    let canvas_api_url = env("CANVAS_API_URL");
    let canvas_api_token = env("CANVAS_API_TOKEN");
    let (year_term, period) = prompt_year_term_period();

    let file_path = format!("enrollments-courserooms-{}-{}.csv", year_term, period);

    println!("Fetching data from Kopps API");
    let course_rounds = kopps_api::get_course_rounds(&kopps_api_url, &year_term, &period)
        .filter(|round| round.first_period == format!("{}{}", year_term, period));

    println!("Writing to the file `{}`", file_path);

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

fn list_exam_room_enrollments() -> Result<(), Box<dyn std::error::Error>> {
    let canvas_api_url = env("CANVAS_API_URL");
    let canvas_api_token = env("CANVAS_API_TOKEN");

    let akt_api_url = env("AKTIVITETSTILLFALLEN_API_URL");
    let akt_api_token = prompt_akt_token();

    let start_date = prompt_date("Enter the start date");
    let end_date = prompt_date("Enter the end date");

    let file_path = format!("enrollments-courserooms-{}---{}.csv", start_date, end_date);
    let mut wtr = Writer::from_path(file_path)?;

    for date in start_date
        .iter_days()
        .take((end_date - start_date).num_days() as usize + 1)
    {
        println!("Getting activities for {}", date);
        let aktivitetstillfallen =
            akt_api::get_aktivitetstillfallen(&akt_api_url, &akt_api_token, &date);

        for round in aktivitetstillfallen {
            println!("- Activity {}", &round.ladok_uid);
            let sis_id1 = format!("AKT.{}", &round.ladok_uid);
            let sis_id2 = format!("AKT.{}.FUNKA", &round.ladok_uid);
            let mut enrollments = canvas_api::get_enrollments(
                canvas_api_url.clone(),
                canvas_api_token.clone(),
                &sis_id1,
            )?;

            enrollments.append(&mut canvas_api::get_enrollments(
                canvas_api_url.clone(),
                canvas_api_token.clone(),
                &sis_id2,
            )?);

            for enrollment in enrollments.into_iter() {
                wtr.serialize(Row {
                    course: &sis_id1,
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
    }

    Ok(())
}

fn env(key: &str) -> String {
    match env::var(key) {
        Ok(val) => val,
        Err(_) => {
            println!("Environmental variable {} not defined", key);
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
        .with_prompt("Choose a term")
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
        .with_prompt("Choose a period")
        .items(&periods)
        .default(0)
        .interact()
        .expect("Failed to get the period");

    let period = periods[period_selection];

    (format!("{}{}", year, term), period.to_string())
}

fn prompt_akt_token() -> String {
    Password::with_theme(&ColorfulTheme::default())
        .with_prompt("Paste the Aktivitetstillfällen API token")
        .interact()
        .expect("Failed to prompt akt token")
}

fn prompt_choice() -> usize {
    let options = vec![
        "Course room enrollments".to_string(),
        "Exam room enrollments".to_string(),
    ];

    Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What do you want to do?")
        .items(&options)
        .default(0)
        .interact()
        .expect("Failed")
}

fn prompt_date(prompt: &str) -> NaiveDate {
    let date_str: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .interact_text()
        .expect("Failed to prompt date");

    let values: Vec<&str> = date_str
        .split("-")
        // .map(|s| s.parse::<u32>().unwrap())
        .collect();

    let (year, month, day) = (
        values
            .get(0)
            .expect("Cannot get year")
            .parse::<i32>()
            .unwrap(),
        values
            .get(1)
            .expect("Cannot get month")
            .parse::<u32>()
            .unwrap(),
        values
            .get(2)
            .expect("Cannot get day")
            .parse::<u32>()
            .unwrap(),
    );

    NaiveDate::from_ymd(year, month, day)
}
