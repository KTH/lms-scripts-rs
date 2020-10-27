use csv::{Reader, Writer};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct User {
    canvas_user_id: i32,
    user_id: Option<String>,
    login_id: Option<String>,
    email: Option<String>,
    created_by_sis: bool,
}

#[derive(Serialize)]
struct UserOut {
    user_id: i32,
    email: Option<String>,
    login_id: Option<String>,
    created_by_sis: bool,
    number: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut wtr = Writer::from_path("provisioning-output.csv")?;

    let mut all_rows: Vec<User> = vec![];

    for result in Reader::from_path("provisioning.csv")?.deserialize() {
        let user: User = result?;

        all_rows.push(user);
    }

    println!("Length is {}", all_rows.len());

    for result in Reader::from_path("provisioning.csv")?.deserialize() {
        let user: User = result?;

        if user.user_id == None && user.email != None {
            let n = all_rows
                .iter()
                .filter(|r| r.canvas_user_id == user.canvas_user_id)
                .count();

            wtr.serialize(UserOut {
                user_id: user.canvas_user_id,
                email: user.email,
                login_id: user.login_id,
                created_by_sis: user.created_by_sis,
                number: n,
            })?;
        }
    }

    Ok(())
}
