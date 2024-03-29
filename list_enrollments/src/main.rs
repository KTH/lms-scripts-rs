mod akt_api;
mod canvas_api;
mod kopps_api;
use chrono::NaiveDate;
use csv::Writer;
use dialoguer::{theme::ColorfulTheme, Input, Select};
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

enum UserChoice {
    CourseRoomEnrollments,
    ExamRoomEnrollments,
}

fn main() {
    dotenv().ok();

    match prompt_choice() {
        UserChoice::CourseRoomEnrollments => list_course_room_enrollments(),
        UserChoice::ExamRoomEnrollments => list_exam_room_enrollments(),
    }
}

fn list_course_room_enrollments() {
    let kopps_api_url = env("KOPPS_API_URL");
    let canvas_api_url = env("CANVAS_API_URL");
    let canvas_api_token = env("CANVAS_API_TOKEN");
    let (year_term, period) = prompt_year_term_period();

    let file_path = format!("enrollments-courserooms-{}-{}.csv", year_term, period);

    println!("Fetching data from Kopps API");
    let course_rounds = kopps_api::get_course_rounds(&kopps_api_url, &year_term, &period)
        .filter(|round| round.first_period == format!("{}{}", year_term, period));

    println!("Writing to the file `{}`", file_path);

    let mut wtr = Writer::from_path(file_path).expect("Error when creating the file");

    for round in course_rounds {
        let sis_id = kopps_api::make_sis_id(&round);
        println!("Round {}", &sis_id);
        let enrollments = canvas_api::get_enrollments(&canvas_api_url, &canvas_api_token, &sis_id)
            .expect("Error when getting enrollments");

        for enrollment in enrollments.into_iter() {
            write_enrollment(
                &mut wtr,
                &format!("{} {}", sis_id, round.first_period),
                enrollment,
            );
        }
    }
}

fn list_exam_room_enrollments() {
    let canvas_api_url = env("CANVAS_API_URL");
    let canvas_api_token = env("CANVAS_API_TOKEN");

    let akt_api_url = env("AKTIVITETSTILLFALLEN_API_URL");
    let akt_api_token = env("AKTIVITETSTILLFALLEN_API_TOKEN");

    let start_date = prompt_date("Enter the start date");
    let end_date = prompt_date("Enter the end date");

    let dates_range = start_date
        .iter_days()
        .take((end_date - start_date).num_days() as usize + 1);

    let file_path = format!("enrollments-examrooms-{}---{}.csv", start_date, end_date);
    let mut wtr = Writer::from_path(file_path).expect("Error when writing a file");

    for date in dates_range {
        println!("Getting activities for {}", date);
        let aktivitetstillfallen =
            akt_api::get_aktivitetstillfallen(&akt_api_url, &akt_api_token, &date);

        for round in aktivitetstillfallen {
            println!("- Activity {}", &round.ladok_uid);
            let sis_id1 = format!("AKT.{}", &round.ladok_uid);
            let sis_id2 = format!("AKT.{}.FUNKA", &round.ladok_uid);
            let mut enrollments =
                canvas_api::get_enrollments(&canvas_api_url, &canvas_api_token, &sis_id1)
                    .expect("Error when getting enrollments");

            enrollments.append(
                &mut canvas_api::get_enrollments(&canvas_api_url, &canvas_api_token, &sis_id2)
                    .expect("Error when getting enrollments"),
            );

            for enrollment in enrollments.into_iter() {
                write_enrollment(&mut wtr, &sis_id1, enrollment);
            }
        }
    }
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

fn prompt_choice() -> UserChoice {
    let options = vec![
        "Course room enrollments".to_string(),
        "Exam room enrollments".to_string(),
    ];

    let choice = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What do you want to do?")
        .items(&options)
        .default(0)
        .interact()
        .expect("Failed");

    match choice {
        0 => UserChoice::CourseRoomEnrollments,
        _ => UserChoice::ExamRoomEnrollments,
    }
}

fn prompt_date(prompt: &str) -> NaiveDate {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .interact_text()
        .expect("Failed to prompt date")
}

fn write_enrollment(
    wtr: &mut Writer<std::fs::File>,
    course_code: &str,
    enrollment: canvas_api::Enrollment,
) {
    wtr.serialize(Row {
        course: course_code,
        name: enrollment.user.sortable_name.as_deref().unwrap_or("??"),
        role: &enrollment.role,
        section: enrollment.sis_section_id.as_deref().unwrap_or("??"),
        mail1: enrollment
            .sis_user_id
            .map(|u| format!("{}@kth.se", u))
            .as_deref()
            .unwrap_or("??"),
        mail2: enrollment.user.login_id.as_deref().unwrap_or("??"),
    })
    .expect("Error when writing a row");
}
