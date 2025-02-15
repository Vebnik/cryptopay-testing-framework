use colored::Colorize;
use reqwest::{self, StatusCode};
use serde_json::{json, Value};
use std::sync::Arc;

use crate::{cmd::api, config::Config, Result};

pub async fn exec(config: Arc<Config>, network_id: String) -> Result<String> {
    let jwt = api::user::utils::get_user_token(Arc::clone(&config)).await?;

    let body = json!({
        "networkId": network_id,
        "password": "test1234test",
    });

    let response = reqwest::Client::new()
        .post("http://localhost:9999/v1/wallet/create")
        .header("Content-Type", "application/json")
        .header("x-auth-token", jwt)
        .body(body.to_string())
        .send()
        .await;

    match response {
        Ok(res) => match res.status() {
            StatusCode::CREATED => {
                let wallet = res.json::<Value>().await?;
                println!(
                    "{} Wallet created: {} ({})",
                    "[API - WALLET]".blue(),
                    wallet["id"],
                    wallet["address"]
                );

                return Ok(wallet["address"].to_string().replace("\"", ""));
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

    Ok("None".into())
}

pub async fn check_wallets_exist(
    config: Arc<Config>,
    networks: Vec<String>,
) -> Result<Vec<String>> {
    let mut wallets = Vec::new();

    for network_id in networks {
        let wallet = exec(Arc::clone(&config), network_id).await?;
        wallets.push(wallet);
    }

    Ok(wallets)
}
