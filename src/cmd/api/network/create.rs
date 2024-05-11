use colored::Colorize;
use reqwest::{self, StatusCode};
use serde_json::{json, Value};
use std::sync::Arc;

use crate::{cmd::api::user::utils::get_admin_token, config::Config, Result};

pub async fn exec(config: Arc<Config>, name: String, kind: String) -> Result<Vec<String>> {
    let jwt = get_admin_token(Arc::clone(&config)).await?;
    let mut networks_id: Vec<String> = Vec::with_capacity(config.anvil_nodes as usize);

    for port in 8545..(8545 + config.anvil_nodes as i32) {
        let body = json!({
            "name": format!("{name}_{}", port),
            "kind": kind,
            "endpoint": format!("ws://127.0.0.1:{}", port),
            "startBlock": 0
        });

        let response = reqwest::Client::new()
            .post("http://localhost:9999/v1/network/create")
            .header("x-auth-token", jwt.clone())
            .header("Content-Type", "application/json")
            .body(body.to_string())
            .send()
            .await?;

        match response.status() {
            StatusCode::CREATED => {
                let data = response.json::<Value>().await?;
                println!(
                    "{} Network created: {}",
                    "[API - NETWORK]".blue(),
                    data["id"]
                );

                networks_id.push(data["id"].to_string().replace("\"", ""));
            }
            _ => println!(
                "{} Network not created: {}",
                "[API - NETWORK]".blue(),
                response.text().await?
            ),
        };
    }

    Ok(networks_id)
}
