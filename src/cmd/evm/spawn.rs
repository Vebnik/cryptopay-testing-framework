use std::{
    error::Error,
    process::{exit, Stdio},
    sync::Arc,
};

use colored::Colorize;
use tokio::process::Command;

use crate::config::Config;

pub async fn exec<'a>(_config: Arc<Config>, amount_nodes: u32) -> Result<(), Box<dyn Error>> {
    if amount_nodes > 10 {
        println!("{} Node amount overhead: {amount_nodes}", "[EVM]".blue());
        exit(0)
    }

    let mut instances = Vec::with_capacity(amount_nodes as usize);

    for port in 8545u32..(8545u32 + amount_nodes) {
        println!("{} Try to spawn Node with port: {port}", "[EVM]".blue());

        let mut cmd = Command::new("anvil")
            .arg("-p")
            .arg(port.to_string())
            .arg("-a")
            .arg("2")
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
