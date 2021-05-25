use reqwest::blocking::Client;
use serde::Deserialize;
use url::Url;

#[derive(Deserialize, Debug, Clone)]
pub struct CourseRound {
    pub course_code: String,
    pub first_semester: String,
    pub first_period: String,
    pub school_code: String,
    pub state: String,
    pub offering_id: String,
    pub offered_semesters: Vec<OfferedSemester>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct OfferedSemester {
    pub start_date: String,
    pub end_date: String,
    pub start_week: String,
    pub end_week: String,
    pub semester: String,
}

/// Get all the course rounds in a given term that starts in a given period
pub fn get_course_rounds(
    kopps_url: &str,
    term: &str,
    _period: &str,
) -> impl Iterator<Item = CourseRound> {
    let url_with_slash = match kopps_url.ends_with("/") {
        true => kopps_url.to_string(),
        false => format!("{}/", kopps_url),
    };

    let suffix = format!("courses/offerings?from={}&skip_coordinator_info=true", term);
    let full_url = Url::parse(&url_with_slash).unwrap().join(&suffix).unwrap();

    Client::new()
        .get(full_url)
        .send()
        .unwrap()
        .json::<Vec<CourseRound>>()
        .unwrap()
        .into_iter()
}

pub fn make_sis_id(course_round: &CourseRound) -> String {
    format!(
        "{}{}{}",
        course_round.course_code, course_round.first_semester, course_round.offering_id
    )
}
