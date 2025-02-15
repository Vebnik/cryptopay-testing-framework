use std::{
    process::{exit, Stdio},
    sync::Arc,
};

use colored::Colorize;
use tokio::process::Command;

use crate::{config::Config, Result};

pub async fn exec<'a>(_config: Arc<Config>, amount_nodes: u32) -> Result<()> {
    if amount_nodes > 10 {
        println!("{} Node amount overhead: {amount_nodes}", "[EVM]".blue());
        exit(0)
    }

    let mut instances = Vec::with_capacity(amount_nodes as usize);

    for port in 8545u32..(8545u32 + amount_nodes) {
        let max = 8545 + amount_nodes;
        let diff = max - port;
        let chain_id = 31337 + amount_nodes - diff;
        println!(
            "{} Spawning node on port: {port} with chain_id: {chain_id} ",
            "[EVM]".blue()
        );

        let mut cmd = Command::new("anvil")
            .arg("-p")
            .arg(port.to_string())
            .arg("--block-time")
            .arg("10")
            .arg("--slots-in-an-epoch")
            .arg("1")
            .arg("--chain-id")
            .arg(chain_id.to_string())
            // .arg("--auto-impersonate")
            // .arg("--prune-history")
            .stdout(Stdio::null())
            .spawn()
            .unwrap();

        let task = tokio::spawn(async move {
            cmd.wait().await.unwrap();
        });

        instances.push(task)
    }

    for handle in instances {
        handle.await.unwrap();
    }

    Ok(())
}
