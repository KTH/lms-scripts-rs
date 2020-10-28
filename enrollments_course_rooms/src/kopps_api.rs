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
}

/// Get all the course rounds in a given term that starts in a given period
pub fn get_course_rounds(
    kopps_url: &str,
    term: &str,
    _period: &str,
) -> impl Iterator<Item = CourseRound> {
    let suffix = format!("courses/offerings?from={}&skip_coordinator_info=true", term);
    let full_url = Url::parse(kopps_url).unwrap().join(&suffix).unwrap();

    Client::new()
        .get(full_url)
        .send()
        .unwrap()
        .json::<Vec<CourseRound>>()
        .unwrap()
        .into_iter()
}
