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

pub async fn check_networks_exist(config: Arc<Config>) -> Result<Vec<String>> {
    let ids =
        cmd::api::network::create::exec(Arc::clone(&config), "LOCAL_EVM".into(), "EVM".into())
            .await?;

    let mut confirm = String::new();
    println!(
        "{} Await for restart cryptopay ... (press enter)",
        "[SERVICE]".blue()
    );
    std::io::stdin().read_line(&mut confirm).unwrap();
    println!("{} Restarted ...", "[SERVICE]".blue());

    return Ok(ids);
}
