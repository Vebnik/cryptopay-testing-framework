use colored::Colorize;
use reqwest::{self, StatusCode};
use serde_json::{json, Value};
use std::{error::Error, sync::Arc};

use crate::config::State;

pub async fn exec(
    state: Arc<State>,
    network_id: String,
    name: String,
    symbol: String,
    address: String,
) -> Result<(), Box<dyn Error>> {
    let user_token = state.system_user_token.clone().take().unwrap();

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

    Ok(())
}
