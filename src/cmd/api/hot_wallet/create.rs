use colored::Colorize;
use reqwest::{self, StatusCode};
use serde_json::{json, Value};
use std::sync::Arc;

use crate::{cmd::api, config::Config, Result};

pub async fn exec(config: Arc<Config>, network_id: String) -> Result<String> {
    let jwt = api::user::utils::get_admin_token(Arc::clone(&config)).await?;

    let body = json!({
        "networkId": network_id,
    });

    let response = reqwest::Client::new()
        .post("http://localhost:9999/v1/admin/wallet/create")
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
                    "{} Hot Wallet created: {} ({})",
                    "[API - HOT WALLET]".blue(),
                    wallet["id"],
                    wallet["address"]
                );

                return Ok(wallet["address"].to_string().replace("\"", ""));
            }
            _ => {
                println!(
                    "{} Wallet not created: {}",
                    "[API - HOT WALLET]".blue(),
                    res.status()
                );
            }
        },
        Err(err) => {
            println!(
                "{} Wallet not created: {}",
                "[API - HOT WALLET]".blue(),
                err
            )
        }
    };

    Ok("None".into())
}
