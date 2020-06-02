extern crate dotenv;

use csv::Writer;
use dotenv::dotenv;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::env;
extern crate canvas_api;
use canvas_api::CanvasApi;

#[derive(Deserialize, Debug, Clone)]
struct CourseRound {
    course_code: String,
    first_semester: String,
    first_period: String,
    school_code: String,
    state: String,
    offering_id: String,
    offered_semesters: Vec<OfferedSemester>,
}

#[derive(Deserialize, Debug, Clone)]
struct OfferedSemester {
    start_date: String,
    end_date: String,
    start_week: String,
    end_week: String,
    semester: String,
}

#[derive(Deserialize, Debug)]
struct Section {
    course_id: i32,
    sis_section_id: Option<String>,
    sis_course_id: Option<String>,
}

#[derive(Deserialize, Debug, PartialEq)]
struct Enrollment {
    id: i32,
    sis_user_id: Option<String>,
    sis_section_id: Option<String>,
    role: String,
    user: User,
}

#[derive(Deserialize, Debug, PartialEq)]
struct User {
    sortable_name: Option<String>,
    login_id: Option<String>,
}

#[derive(Serialize)]
struct Row<'a> {
    name: &'a str,
    course: &'a str,
    role: &'a str,
    section: &'a str,
    mail1: &'a str,
    mail2: &'a str,
}

#[derive(Serialize)]
struct OutputRow {
    sis_id: String,
    school_code: String,
    start_date: String,
    end_date: String,
    period: String,
}

fn valid_offered_semesters(all_semesters: Vec<OfferedSemester>) -> Vec<OfferedSemester> {
    let mut result = vec![];

    for semester in all_semesters.into_iter() {
        if semester.semester == "20201" && semester.end_date >= "2020-03-20".to_string() {
            result.push(semester);
        }
    }

    return result;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let kopps_url = env::var("KOPPS_API_URL").unwrap();
    let canvas_api = CanvasApi::new(
        env::var("CANVAS_API_URL").unwrap(),
        env::var("CANVAS_API_TOKEN").unwrap(),
    );
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

    let filtered = course_rounds
        .into_iter()
        .filter(|round| round.state == "Godkänt")
        .filter(|round| round.first_period == "20201P3" || round.first_period == "20201P4")
        .map(|round| CourseRound {
            offered_semesters: valid_offered_semesters(round.offered_semesters),
            ..round
        })
        .filter(|round| round.offered_semesters.len() > 0);

    println!(
        "Course rounds in 2020-P3/P4 {}. Total course rounds in 2020-VT {}",
        filtered.clone().count(),
        total_length
    );

    let mut wtr = Writer::from_path("list.csv")?;

    for round in filtered {
        let sis_id = format!(
            "{}{}{}",
            round.course_code, round.first_semester, round.offering_id
        );

        let pages =
            canvas_api.get_paginated(&format!("/sections/sis_section_id:{}/enrollments", sis_id));

        for response in pages {
            let result = response?;
            let status = &result.status();

            if status == &404 {
                println!("Section {} not found", sis_id);
                break;
            } else if status != &200 {
                println!(
                    "Unexpected response when requesting section '{}'. Status: {}",
                    sis_id, status
                );
                panic!();
            }

            let enrollments = result.json::<Vec<Enrollment>>()?.into_iter();

            for enrollment in enrollments {
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
    }

    /*
    let mut wtr = Writer::from_path("foo.csv")?;
    for row in filtered.iter() {
        wtr.serialize(row)?;
    }
    */

    Ok(())
}
