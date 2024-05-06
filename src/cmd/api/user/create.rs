use bigdecimal::{BigDecimal, FromPrimitive};
use colored::Colorize;
use reqwest::StatusCode;
use serde_json::json;
use std::error::Error;
use std::sync::Arc;

use crate::cmd::api::utils;
use crate::config::State;

async fn admin_flow(
    state: Arc<State>,
    name: String,
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
        name, email, encrypted, fee, "EUR", true,
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

async fn user_flow(
    _state: Arc<State>,
    name: String,
    email: Option<String>,
) -> Result<(), Box<dyn Error>> {
    let email = email.unwrap_or(format!("test_{}@localhost.com", rand::random::<u32>()));

    let body = json!({
        "name": name,
        "email": email,
        "password": "test1234",
        "currency": "EUR"
    });

    let result = reqwest::Client::new()
        .post("http://localhost:9999/v1/user/register")
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .send()
        .await;

    match result {
        Ok(res) => match res.status() {
            StatusCode::CREATED => {
                println!(
                    "{} User created: (email: {email}, password: test1234)",
                    "[API - USER]".blue(),
                );
            }
            _ => {
                println!(
                    "{} User not created: ({})",
                    "[API - USER]".blue(),
                    res.status()
                );
            }
        },
        Err(err) => {
            println!("{} User not created: ({})", "[API - USER]".blue(), err);
        }
    }

    Ok(())
}

pub async fn exec(
    state: Arc<State>,
    name: String,
    is_admin: bool,
    email: Option<String>,
) -> Result<(), Box<dyn Error>> {
    if is_admin {
        admin_flow(Arc::clone(&state), name, email).await?
    } else {
        user_flow(Arc::clone(&state), name, email).await?
    }

    Ok(())
}
