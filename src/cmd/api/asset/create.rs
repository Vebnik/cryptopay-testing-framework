use colored::Colorize;
use reqwest::{self, StatusCode};
use serde_json::{json, Value};
use std::sync::Arc;

use crate::{cmd::api::user::utils::get_admin_token, config::{Config, TEST_TOKENS}, Result};

pub async fn exec(
    config: Arc<Config>,
    network_id: String,
    name: String,
    symbol: String,
    address: String,
) -> Result<String> {
    let jwt = get_admin_token(Arc::clone(&config)).await?;

    let body = json!({
        "networkId": network_id,
        "name": name,
        "symbol": symbol,
        "address": address,
        "decimals": 18,
        "minWithdrawal": 500000000
    });

    let response = reqwest::Client::new()
        .post("http://localhost:9999/v1/asset/create")
        .header("Content-Type", "application/json")
        .header("x-auth-token", jwt)
        .body(body.to_string())
        .send()
        .await;

    match response {
        Ok(res) => match res.status() {
            StatusCode::CREATED => {
                let asset = res.json::<Value>().await?;

                println!("{} Asset created: {}", "[API - ASSET]".blue(), asset["id"]);

                return Ok(asset["id"].to_string().replace("\"", ""));
            }
            _ => {
                println!(
                    "{} Asset not created: {}",
                    "[API - ASSET]".blue(),
                    res.status()
                );
            }
        },
        Err(err) => {
            println!("{} Asset not created: {}", "[API - ASSET]".blue(), err)
        }
    };

    Ok("None".into())
}

pub async fn check_assets_exist(
    config: Arc<Config>,
    networks: Vec<String>,
    contracts: Vec<Vec<String>>,
) -> Result<()> {
    for network_id in networks {
        for (i, address) in contracts.clone().iter().enumerate() {
            let _ = exec(
                Arc::clone(&config),
                network_id.clone(),
                TEST_TOKENS[i].0.into(),
                TEST_TOKENS[i].1.into(),
                address[i].clone(),
            )
            .await?;
        }
    }

    Ok(())
}
