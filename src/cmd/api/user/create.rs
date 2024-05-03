use bigdecimal::{BigDecimal, FromPrimitive};
use colored::Colorize;
use std::error::Error;
use std::sync::Arc;

use crate::cmd::api::utils;
use crate::config::State;

pub async fn exec(
    state: Arc<State>,
    name: String,
    is_admin: bool,
    email: Option<String>,
) -> Result<(), Box<dyn Error>> {
    let fee = BigDecimal::from_u32(2).expect("valid");
    let encrypted = utils::password::hash("test1234").await?;
    let email = email.unwrap_or(format!("test_{}@localhost.com", rand::random::<u32>()));

    let query = format!(
        r#"
        INSERT INTO "user" ("name", "email", "password", "fee", "currency", "is_admin", "is_verified", "email_token")
        VALUES ('{}', '{}', '{}', {}, '{}', {}, true, null)
        RETURNING id
        "#,
        name, email, encrypted, fee, "EUR", is_admin,
    );

    let result = sqlx::raw_sql(&query).execute(&state.db).await;

    match result {
        Ok(_raw) => {
            println!(
                "{} User created: (email: {email}, password: test1234)",
                "[API - USER]".blue(),
            );
        }
        Err(err) => {
            println!("{} User not created: ({})", "[API - USER]".blue(), err);
        }
    }

    Ok(())
}
