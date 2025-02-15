use colored::Colorize;
use serde_json::{json, Value};
use sqlx::types::Uuid;
use std::process::exit;
use std::sync::Arc;

use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};

use crate::{cmd::service, Config, Error, Result};

pub async fn hash(password: &str) -> Result<String> {
    let salt = SaltString::generate(rand::thread_rng());

    let pass = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| Error::PasswordHash)?
        .to_string();

    Ok(pass)
}

pub async fn get_admin_user_id(config: Arc<Config>) -> Result<String> {
    let token = get_admin_token(Arc::clone(&config)).await?;

    let response = reqwest::Client::new()
        .get(format!("{}/v1/user/me", config.cryptopay_url))
        .header("x-auth-token", token.clone())
        .header("Content-Type", "application/json")
        .send()
        .await;

    match response {
        Ok(res) => {
            if res.status().as_u16() > 201u16 {
                println!(
                    "{} Admin user info error: ({})",
                    "[SERVICE]".blue(),
                    res.text().await?
                );
                exit(0)
            }

            let id = res.json::<Value>().await?["id"]
                .to_string()
                .replace("\"", "");

            println!("{} Admin user ID: ({})", "[SERVICE]".blue(), id);

            return Ok(id);
        }
        Err(err) => {
            println!("{} System user token error: ({})", "[SERVICE]".blue(), err);
            exit(0)
        }
    }
}

pub async fn get_admin_token(config: Arc<Config>) -> Result<String> {
    let body = json!({
        "email": "admin@cryptopay.wtf",
        "password": "test1234"
    });

    let response = reqwest::Client::new()
        .post(format!("{}/v1/user/login", config.cryptopay_url))
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .send()
        .await;

    match response {
        Ok(res) => {
            if res.status().as_u16() > 201u16 {
                println!(
                    "{} Admin user token error: ({})",
                    "[SERVICE]".blue(),
                    res.text().await?
                );
                exit(0)
            }

            let token = res.json::<Value>().await?["token"]
                .to_string()
                .replace("\"", "");

            // println!(
            //     "{} Admin user token: ({} chars)",
            //     "[SERVICE]".blue(),
            //     token.len()
            // );

            return Ok(token);
        }
        Err(err) => {
            println!("{} Admin user token error: ({})", "[SERVICE]".blue(), err);
            exit(0)
        }
    }
}

pub async fn get_user_token(config: Arc<Config>) -> Result<String> {
    let body = json!({
        "email": "tester@cryptopay.wtf",
        "password": "test1234"
    });

    let response = reqwest::Client::new()
        .post(format!("{}/v1/user/login", config.cryptopay_url))
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .send()
        .await;

    match response {
        Ok(res) => {
            if res.status().as_u16() > 201u16 {
                println!(
                    "{} Test user token error: ({})",
                    "[SERVICE]".blue(),
                    res.text().await?
                );
                exit(0)
            }

            let token = res.json::<Value>().await?["token"]
                .to_string()
                .replace("\"", "");

            // println!(
            //     "{} Test user token: ({} chars)",
            //     "[SERVICE]".blue(),
            //     token.len()
            // );

            return Ok(token);
        }
        Err(err) => {
            println!("{} Test user token error: ({})", "[SERVICE]".blue(), err);
            exit(0)
        }
    }
}

pub async fn check_admin_exists(config: Arc<Config>) -> Result<()> {
    let db = service::utils::get_db(Arc::clone(&config)).await?;

    let res: Result<Uuid, sqlx::Error> =
        sqlx::query_scalar(r#"select id from "user" where email = 'admin@cryptopay.wtf'"#)
            .fetch_one(&db)
            .await;

    match res {
        Ok(data) => {
            println!("{} Admin user exists: ({})", "[SERVICE]".blue(), data);
        }
        Err(err) => {
            println!(
                "{} Admin user does not exist, creating ({err})",
                "[SERVICE]".blue(),
            );

            super::create::exec(
                Arc::clone(&config),
                "Admin".into(),
                "admin@cryptopay.wtf".into(),
                true,
            )
            .await?;
        }
    }

    Ok(())
}

pub async fn check_tester_exists(config: Arc<Config>) -> Result<()> {
    let db = service::utils::get_db(Arc::clone(&config)).await?;

    let res: Result<Uuid, sqlx::Error> =
        sqlx::query_scalar(r#"select id from "user" where email = 'tester@cryptopay.wtf'"#)
            .fetch_one(&db)
            .await;

    match res {
        Ok(data) => {
            println!("{} Tester user exists: ({})", "[SERVICE]".blue(), data);
        }
        Err(err) => {
            println!(
                "{} Tester user does not exist, creating ({err})",
                "[SERVICE]".blue(),
            );

            super::create::exec(
                Arc::clone(&config),
                "Tester".into(),
                "tester@cryptopay.wtf".into(),
                false,
            )
            .await?;

            // println!("{} Tester user created", "[SERVICE]".blue());
        }
    }

    Ok(())
}
