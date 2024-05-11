use colored::Colorize;
use reqwest::{self, StatusCode};
use serde_json::{json, Value};
use std::{error::Error, sync::Arc};

use crate::{cmd::api::utils::user::get_admin_token, config::Config};

pub async fn exec(
    config: Arc<Config>,
    network_id: String,
    name: String,
    symbol: String,
    address: String,
) -> Result<String, Box<dyn Error>> {
    let user_token = get_admin_token(Arc::clone(&config)).await?;

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
        .header("x-auth-token", user_token.clone())
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
