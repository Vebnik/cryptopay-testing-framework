use colored::Colorize;
use reqwest::{self, StatusCode};
use serde_json::{json, Value};
use std::{error::Error, sync::Arc};

use crate::{cmd::api::utils::user::get_system_user_token, config::Config};

pub async fn exec(
    config: Arc<Config>,
    network_id: String,
    password: String,
) -> Result<(), Box<dyn Error>> {
    let user_token = get_system_user_token(Arc::clone(&config)).await?;

    let body = json!({
        "networkId": network_id,
        "password": password
    });

    let response = reqwest::Client::new()
        .post("http://localhost:9999/v1/wallet/create")
        .header("Content-Type", "application/json")
        .header("x-auth-token", user_token.clone())
        .body(body.to_string())
        .send()
        .await;

    match response {
        Ok(res) => match res.status() {
            StatusCode::CREATED => {
                let wallet = res.json::<Value>().await?;
                println!(
                    "{} Wallet created: {}",
                    "[API - WALLET]".blue(),
                    wallet["id"]
                );
            }
            _ => {
                println!(
                    "{} Wallet not created: {}",
                    "[API - WALLET]".blue(),
                    res.status()
                );
            }
        },
        Err(err) => {
            println!("{} Wallet not created: {}", "[API - WALLET]".blue(), err)
        }
    };

    Ok(())
}
