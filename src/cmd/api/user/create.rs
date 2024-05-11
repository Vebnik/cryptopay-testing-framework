use bigdecimal::{BigDecimal, FromPrimitive};
use colored::Colorize;
use reqwest::StatusCode;
use serde_json::json;
use std::sync::Arc;

use crate::{cmd::api::utils::password, config::Config, utils, Result};

const PASSWORD: &str = "test1234";

async fn create_admin(config: Arc<Config>, name: String, email: String) -> Result<()> {
    let db = utils::get_db(Arc::clone(&config)).await?;

    let fee = BigDecimal::from_u32(2).expect("valid");
    let encrypted = password::hash(PASSWORD).await?;

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
                "{} Admin user created: (email: {email}, password: {PASSWORD})",
                "[API - USER]".blue(),
            );
        }
        Err(err) => {
            println!(
                "{} Admin user not created: ({})",
                "[API - USER]".blue(),
                err
            );
        }
    }

    Ok(())
}

async fn create_user(_state: Arc<Config>, name: String, email: String) -> Result<()> {
    let body = json!({
        "name": name,
        "email": email,
        "password": PASSWORD,
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
                    "{} Tester user created: (email: {email}, password: {PASSWORD})",
                    "[API - USER]".blue(),
                );
            }
            _ => {
                println!(
                    "{} Tester user not created: ({})",
                    "[API - USER]".blue(),
                    res.status()
                );
            }
        },
        Err(err) => {
            println!(
                "{} Tester user not created: ({})",
                "[API - USER]".blue(),
                err
            );
        }
    }

    Ok(())
}

pub async fn exec(config: Arc<Config>, name: String, email: String, is_admin: bool) -> Result<()> {
    if is_admin {
        create_admin(Arc::clone(&config), name, email).await?
    } else {
        create_user(Arc::clone(&config), name, email).await?
    }

    Ok(())
}
