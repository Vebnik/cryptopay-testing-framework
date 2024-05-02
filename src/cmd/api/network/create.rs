use colored::Colorize;
use std::error::Error;
use reqwest::{self, StatusCode};
use serde_json::{json, Value};

use crate::config::State;


pub async fn exec(
    name: String,
    kind: String,
    endpoint: String,
    _sate: State
) -> Result<(), Box<dyn Error>> {

    let body = json!({
        "name": name,
        "kind": kind,
        "endpoint": endpoint,
        "startBlock": 0
    });

    let response = reqwest::Client::new()
        .post("")
        .header("x-auth-token", "")
        .header("Content-Type", "application/json")
        .body(body.to_string())
        .send()
        .await?;

    match response.status() {
        StatusCode::CREATED => {
            let data = response.json::<Value>().await?;
            println!("{} Network created: {}", "[API - NETWORK]".blue(), data["id"])
        },
        _ => println!("{} Network not created", "[API - NETWORK]".blue()),
    };

    Ok(())
}