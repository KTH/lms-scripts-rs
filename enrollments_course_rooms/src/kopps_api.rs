use url::Url;

/// Get all the course rounds in a given term that starts in a given period
pub fn get_course_rounds(kopps_url: &str, term: &str, _period: &str) {
    let suffix = format!("courses/offerings?from={}&skip_coordinator_info=true", term);
    let full_url = Url::parse(kopps_url).unwrap().join(&suffix).unwrap();

    println!("{}", full_url);
}
