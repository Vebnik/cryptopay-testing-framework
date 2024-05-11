use bigdecimal::{BigDecimal, FromPrimitive};
use colored::Colorize;
use reqwest::StatusCode;
use serde_json::json;
use std::sync::Arc;

use crate::{cmd::api::utils::password, config::Config, utils, Result};

async fn admin_flow(config: Arc<Config>, name: String, email: String) -> Result<()> {
    let db = utils::get_db(Arc::clone(&config)).await?;

    let fee = BigDecimal::from_u32(2).expect("valid");
    let encrypted = password::hash("test1234").await?;

    let query = format!(
        r#"
        INSERT INTO "user" ("name", "email", "password", "fee", "currency", "is_admin", "is_verified", "email_token")
        VALUES ('{}', '{}', '{}', {}, '{}', true, true, null)
        RETURNING id
        "#,
        name, email, encrypted, fee, "EUR"
    );

    let result = sqlx::raw_sql(&query).execute(&db).await;

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

async fn user_flow(_state: Arc<Config>, name: String, email: String) -> Result<()> {
    let password = "test1234";

    let body = json!({
        "name": name,
        "email": email,
        "password": password,
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
                    "{} User created: (email: {email}, password: {password})",
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

pub async fn exec(config: Arc<Config>, name: String, is_admin: bool, email: String) -> Result<()> {
    if is_admin {
        admin_flow(Arc::clone(&config), name, email).await?
    } else {
        user_flow(Arc::clone(&config), name, email).await?
    }

    Ok(())
}
