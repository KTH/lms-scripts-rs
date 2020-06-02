extern crate dotenv;

use dotenv::dotenv;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
struct CourseRound {
    course_code: String,
    first_semester: String,
    first_period: String,
    school_code: String,
    state: String,
    offering_id: String,
    offered_semesters: Vec<OfferedSemester>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct OfferedSemester {
    start_date: String,
    end_date: String,
    start_week: String,
    end_week: String,
    semester: String,
}

#[derive(Serialize)]
struct OutputRow {
    sis_id: String,
    school_code: String,
    start_date: String,
    end_date: String,
    period: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let kopps_url = env::var("KOPPS_API_URL").unwrap();
    env::var("CANVAS_API_URL").unwrap();
    let client = Client::new();

    println!("Calling Kopps API to get course rounds");
    let course_rounds = client
        .get(&format!(
            "{}/courses/offerings?from=20201&skip_coordinator_info=true",
            kopps_url
        ))
        .send()?
        .json::<Vec<CourseRound>>()?;

    let total_length = course_rounds.len();

    let filtered: Vec<CourseRound> = course_rounds
        .into_iter()
        .filter(|round| round.state == "Godkänt")
        .filter(|round| round.first_period == "20201P3" || round.first_period == "20201P4")
        .collect();

    println!(
        "Course rounds in 2020-P3/P4 {}. Total course rounds in 2020-VT {}",
        filtered.len(),
        total_length
    );

    /*
    let mut wtr = Writer::from_path("foo.csv")?;
    for row in filtered.iter() {
        wtr.serialize(row)?;
    }
    */

    Ok(())
}
