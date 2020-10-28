use dialoguer::{theme::ColorfulTheme, Input, Select};

fn main() {
    let kopps_api_url = prompt_kopps_api_url();
    let (year_term, period) = prompt_year_term_period();

    println!("Kopps API URL: {}", kopps_api_url);
    println!("Yearterm: {}. Period: {}", year_term, period);
}

/// Prompt the KOPPS API URL
fn prompt_kopps_api_url() -> String {
    let input: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Write the URL for KOPPS API")
        .with_initial_text("https://api.kth.se/api/kopps/v2")
        .interact_text()
        .expect("Failed to prompt file name");

    if input.ends_with("/") {
        input
    } else {
        format!("{}/", input)
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
