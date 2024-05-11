use colored::Colorize;
use serde::Deserialize;
use serde_json::Value;
use std::sync::Arc;
use uuid::Uuid;

use crate::{cmd, Config, Result};

#[derive(Debug, Clone, Deserialize)]
struct Network {
    pub id: Uuid,
    pub name: String,
    pub kind: String,
}

pub async fn check_networks_exist(config: Arc<Config>) -> Result<()> {
    let response = reqwest::Client::new()
        .get(format!("{}/v1/network/all", config.cryptopay_url))
        .send()
        .await;

    match response {
        Ok(res) => {
            let networks = res.json::<Vec<Network>>().await?;

            if networks.iter().count() == 0 {
                println!("{} Networks not found, creating", "[SERVICE]".blue());
                let ids = cmd::api::network::create::exec(
                    Arc::clone(&config),
                    "LOCAL_EVM".into(),
                    "EVM".into(),
                )
                .await?;

                let mut confirm = String::new();
                println!(
                    "{} Await for restart cryptopay ... (press enter)",
                    "[SERVICE]".blue()
                );
                std::io::stdin().read_line(&mut confirm).unwrap();
                println!("{} Restarted ...", "[SERVICE]".blue());
            } else {
                println!(
                    "{} Networks exist: {}",
                    "[SERVICE]".blue(),
                    networks
                        .iter()
                        .map(|n| n.name.clone())
                        .collect::<Vec<_>>()
                        .join(", ")
                );
            }
        }
        Err(err) => {
            println!("{} Networks not found: ({})", "[SERVICE]".blue(), err);
            std::process::exit(0)
        }
    }

    Ok(())
}
