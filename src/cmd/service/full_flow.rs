use std::{error::Error, sync::Arc};

use alloy::network::EthereumSigner;
use alloy::providers::ProviderBuilder;
use alloy::signers::wallet::LocalWallet;
use colored::Colorize;
use std::io;

use crate::config::{Config, ProviderType, TEST_WALLETS};
use crate::{
    cmd, cmd::api::asset, cmd::api::intent, cmd::api::network, cmd::api::user, cmd::api::utils,
    cmd::api::wallet, cmd::evm::deploy, cmd::evm::mint, cmd::evm::transfer,
};

#[derive(Debug, Clone)]
struct NetworkAsset {
    pub network_id: String,
    pub asset_id: String,
}

pub async fn exec(config: Arc<Config>) -> Result<(), Box<dyn Error>> {
    // Test wallet
    let _core_wallet = config.core_priv_key.parse::<LocalWallet>()?;

    // Test data
    let test_user_name = "test_user";
    let test_user_email = "test_user@localhost.com";
    let test_wallet_pass = "test1234test";

    // Test config
    let wallets_count: usize = 1;

    let mut wallets: Vec<String> = Vec::new();
    let mut assets_networks: Vec<NetworkAsset> = Vec::new();
    let mut intents_id: Vec<String> = Vec::new();

    // drop exist db
    cmd::db::drop::exec(Arc::clone(&config)).await?;

    // check and create system user
    utils::user::check_exist_system_user(Arc::clone(&config)).await?;

    // create simple user
    user::create::exec(
        Arc::clone(&config),
        test_user_name.into(),
        false,
        Some(test_user_email.into()),
    )
    .await?;

    // create networks
    let networks_id =
        network::create::exec(Arc::clone(&config), "Local ETH".into(), "EVM".into()).await?;

    let mut confirm = String::new();
    println!(
        "{} Await for restart cryptopay ... (press enter)",
        "[SERVICE]".blue()
    );
    io::stdin().read_line(&mut confirm).unwrap();
    println!("{} Restarted ...", "[SERVICE]".blue());

    // create wallet with network and test_system_user
    for network_id in networks_id.clone() {
        let wallet =
            wallet::create::exec(Arc::clone(&config), network_id, test_wallet_pass.into()).await?;

        wallets.push(wallet)
    }

    // deploy contracts each anvil nodes with sigkey test_system_user
    let contracts = deploy::exec(
        Arc::clone(&config),
        "Test USDT".into(),
        "TUSDT".into(),
        10000,
    )
    .await?;

    // deploy contracts each anvil nodes with sigkey test_system_user
    for network_id in networks_id.clone() {
        let asset_id = asset::create::exec(
            Arc::clone(&config),
            network_id.clone(),
            "Test USDT".into(),
            "USDT".into(),
            contracts[0].clone(),
        )
        .await?;

        assets_networks.push(NetworkAsset {
            network_id: network_id.clone(),
            asset_id,
        });
    }

    // mint USDT token to test wallets
    for (address, _key) in TEST_WALLETS[0..wallets_count].iter() {
        mint::exec(
            Arc::clone(&config),
            address.to_string(),
            contracts[0].clone(),
            10000,
        )
        .await?;
    }

    // create intent
    for na in assets_networks {
        for (address, _key) in TEST_WALLETS[0..wallets_count].iter() {
            let id = intent::create::exec(
                Arc::clone(&config),
                na.network_id.clone(),
                na.asset_id.clone(),
                address.to_string(),
                420,
            )
            .await?;
            intents_id.push(id);
        }
    }

    // create transfer
    // i - index wallet in wallets array on each anvil nodes
    for (i, port) in (8545..(8545 + config.anvil_nodes as i32)).enumerate() {
        println!(
            "{} On anvil: {}",
            "[DUBUG]".yellow(),
            format!("http://localhost:{}", port)
        );

        // for wallet on each network -> anvil nodes
        // for test wallet on each anvil nodes
        for (address, key) in TEST_WALLETS[0..wallets_count].iter() {
            // crate wallet for wallet anvil nodes
            let test_wallet = key.parse::<LocalWallet>()?;

            // crate provider for wallet anvil nodes
            let provider: ProviderType = ProviderBuilder::new()
                .with_recommended_fillers()
                .signer(EthereumSigner::from(test_wallet))
                .on_builtin(&format!("http://localhost:{}", port))
                .await?;

            transfer::exec(
                Arc::clone(&config),
                wallets[i].clone(),
                address.to_string(),
                contracts[0].clone(),
                420,
                provider.clone(),
            )
            .await?;
        }
    }

    Ok(())
}
