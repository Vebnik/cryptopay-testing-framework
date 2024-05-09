use colored::Colorize;
use reqwest::{self, StatusCode};
use serde_json::{json, Value};
use std::{error::Error, sync::Arc};

use crate::{cmd::api::utils::user::get_system_user_id, config::Config};

pub async fn exec(
    config: Arc<Config>,
    network_id: String,
    asset_id: String,
    sender: String,
    amount: i32,
) -> Result<String, Box<dyn Error>> {
    let user_id = get_system_user_id(Arc::clone(&config)).await?;

    let body = json!({
        "userId": user_id,
        "networkId": network_id,
        "assetId": asset_id,
        "sender": sender,
        "amount": amount.to_string()
    });

    let response = reqwest::Client::new()
        .post("http://localhost:9999/v1/intent/create")
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .send()
        .await;

    match response {
        Ok(res) => match res.status() {
            StatusCode::CREATED | StatusCode::OK => {
                let intent = res.json::<Value>().await?;

                println!(
                    "{} Intent created: {} | status: {}",
                    "[API - INTENT]".blue(),
                    intent["id"],
                    intent["status"]
                );

                return Ok(intent["id"].to_string().replace("\"", ""));
            }
            _ => {
                println!(
                    "{} Intent not created: {}",
                    "[API - INTENT]".blue(),
                    res.status()
                );
            }
        },
        Err(err) => {
            println!("{} Intent not created: {}", "[API - INTENT]".blue(), err)
        }
    };

    Ok("None".into())
}
