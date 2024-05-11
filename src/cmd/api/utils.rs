pub mod password {
    use std::error::Error;
    use std::io;

    use argon2::password_hash::SaltString;
    use argon2::{Argon2, PasswordHasher};

    pub async fn hash(password: &str) -> Result<String, Box<dyn Error>> {
        let salt = SaltString::generate(rand::thread_rng());

        let pass = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err.to_string()))?
            .to_string();

        Ok(pass)
    }
}

pub mod user {
    use colored::Colorize;
    use serde_json::{json, Value};
    use sqlx::types::Uuid;
    use std::sync::Arc;
    use std::{error::Error, process::exit};

    use crate::cmd::api::user;
    use crate::config::Config;
    use crate::utils;

    pub async fn get_admin_user_id(config: Arc<Config>) -> Result<String, Box<dyn Error>> {
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
                        "{} System user info error: ({})",
                        "[SERVICE]".blue(),
                        res.text().await?
                    );
                    exit(0)
                }

                let id = res.json::<Value>().await?["id"]
                    .to_string()
                    .replace("\"", "");

                println!("{} System user id: ({})", "[SERVICE]".blue(), id);

                return Ok(id);
            }
            Err(err) => {
                println!("{} System user token error: ({})", "[SERVICE]".blue(), err);
                exit(0)
            }
        }
    }

    pub async fn get_admin_token(config: Arc<Config>) -> Result<String, Box<dyn Error>> {
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
                        "{} System user token error: ({})",
                        "[SERVICE]".blue(),
                        res.text().await?
                    );
                    exit(0)
                }

                let token = res.json::<Value>().await?["token"]
                    .to_string()
                    .replace("\"", "");

                println!(
                    "{} System user token: ({} chars)",
                    "[SERVICE]".blue(),
                    token.len()
                );

                return Ok(token);
            }
            Err(err) => {
                println!("{} System user token error: ({})", "[SERVICE]".blue(), err);
                exit(0)
            }
        }
    }

    pub async fn get_user_token(config: Arc<Config>) -> Result<String, Box<dyn Error>> {
        let body = json!({
            "email": "test@cryptopay.wtf",
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
                        "{} User token error: ({})",
                        "[SERVICE]".blue(),
                        res.text().await?
                    );
                    exit(0)
                }

                let token = res.json::<Value>().await?["token"]
                    .to_string()
                    .replace("\"", "");

                println!("{} User token: ({} chars)", "[SERVICE]".blue(), token.len());

                return Ok(token);
            }
            Err(err) => {
                println!("{} User token error: ({})", "[SERVICE]".blue(), err);
                exit(0)
            }
        }
    }

    pub async fn check_admin_exists(config: Arc<Config>) -> Result<(), Box<dyn Error>> {
        let db = utils::get_db(Arc::clone(&config)).await?;

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
                    "{} Admin user does not exist, creating: {}",
                    "[SERVICE]".blue(),
                    err
                );

                user::create::exec(
                    Arc::clone(&config),
                    "Admin".into(),
                    true,
                    "admin@cryptopay.wtf".into(),
                )
                .await?;

                println!("{} Admin user created", "[SERVICE]".blue());
                // let token = get_admin_token(Arc::clone(&config)).await?;
                // *state.system_user_token.borrow_mut() = Some(token);
            }
        }

        Ok(())
    }
}
