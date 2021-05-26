extern crate canvas_api;

use canvas_api::CanvasApi;
use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub struct Enrollment {
    pub id: i32,
    pub sis_user_id: Option<String>,
    pub sis_section_id: Option<String>,
    pub role: String,
    pub user: User,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct User {
    pub sortable_name: Option<String>,
    pub login_id: Option<String>,
}

pub fn get_enrollments(
    canvas_api_url: &str,
    canvas_api_token: &str,
    sis_section_id: &str,
) -> Result<Vec<Enrollment>, Box<dyn std::error::Error>> {
    let canvas_api = CanvasApi::new(canvas_api_url, canvas_api_token);
    let pages = canvas_api.get_paginated(&format!(
        "/sections/sis_section_id:{}/enrollments",
        sis_section_id
    ));

    let mut all_enrollments: Vec<Enrollment> = vec![];

    for response in pages {
        let result = response?;
        let status = &result.status();

        if status == &404 {
            println!("Section {} not found", sis_section_id);
            break;
        } else if status != &200 {
            println!(
                "Unexpected response when requesting section '{}'. Status: {}",
                sis_section_id, status
            );
            panic!();
        }

        let mut enrollments = result.json::<Vec<Enrollment>>()?;
        all_enrollments.append(&mut enrollments);
    }

    Ok(all_enrollments)
}
