use std::sync::Arc;

use alloy::signers::wallet::LocalWallet;
use colored::Colorize;
use ethers::providers::{Provider, Ws};

use crate::config::{Config, TEST_WALLETS, TEST_TOKENS};
use crate::{
    cmd::api::{asset, intent, network, user, wallet},
    cmd::evm::{deploy, mint, transfer},
    cmd::{api, db, service},
    Result,
};

#[derive(Debug, Clone)]
struct NetworkAsset {
    pub network_id: String,
    pub asset_id: String,
}

pub async fn exec(config: Arc<Config>) -> Result<()> {
    service::utils::check(Arc::clone(&config)).await?;

    // Test wallet
    let _core_wallet = config.core_key.parse::<LocalWallet>()?;

    // Test data
    let test_user_name = "Tester";
    let test_user_email = "tester@cryptopay.wtf";

    // let test_wallet_pass = "test1234test";

    // Test config
    let wallets_count: usize = 1;

    let mut wallets: Vec<String> = Vec::new();
    let mut contracts: Vec<Vec<String>> = Vec::with_capacity(TEST_TOKENS.len());
    let mut assets_networks: Vec<NetworkAsset> = Vec::new();
    let mut intents_id: Vec<String> = Vec::new();

    // drop exist db
    db::reset::exec(Arc::clone(&config)).await?;

    // check and create admin user
    api::user::utils::check_admin_exists(Arc::clone(&config)).await?;

    // create simple user
    user::create::exec(
        Arc::clone(&config),
        test_user_name.into(),
        test_user_email.into(),
        false,
    )
    .await?;

    // create networks
    let network_ids =
        network::create::exec(Arc::clone(&config), "LOCAL_EVM".into(), "EVM".into()).await?;

    // await cryptopay back for hard reset
    service::utils::await_restart().await?;

    // create wallet with network and test_system_user
    for network_id in network_ids.clone() {
        let wallet = wallet::create::exec(Arc::clone(&config), network_id).await?;

        wallets.push(wallet)
    }

    // deploy contracts each anvil nodes with sigkey test_system_user
    for (name, symbol, decimals) in TEST_TOKENS {
        let contracts_address = deploy::exec(
            Arc::clone(&config),
            name.into(),
            symbol.into(),
            decimals,
            10000,
        )
        .await?;
        
        contracts.push(contracts_address);
    }

    // create assets
    for network_id in network_ids.clone() {
        for (i, (name, symbol, _decimals)) in TEST_TOKENS.iter().enumerate() {
            let asset_id = asset::create::exec(
                Arc::clone(&config),
                network_id.clone(),
                name.to_string(),
                symbol.to_string(),
                contracts[i][0].clone(),
            )
            .await?;
    
            assets_networks.push(NetworkAsset {
                network_id: network_id.clone(),
                asset_id,
            });
        }
    }

    service::utils::await_restart().await?;

    // mint USDT token to test wallets
    for contracts_address in contracts.clone() {
        for (address, _key) in TEST_WALLETS[0..wallets_count].iter() {
            mint::exec(
                Arc::clone(&config),
                address.to_string(),
                contracts_address[0].clone(),
                10000,
            )
            .await?;
        }
    }

    println!("AFTER");

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
    service::utils::await_restart().await?;

    // create transfer
    // i - index wallet in wallets array on each anvil nodes
    for (i, port) in (8545..(8545 + config.anvil_nodes as i32)).enumerate() {
        println!(
            "{} On anvil: {}",
            "[DUBUG]".yellow(),
            format!("ws://localhost:{port}")
        );

        let provider = Provider::<Ws>::connect(format!("ws://127.0.0.1:{port}"))
            .await
            .unwrap();

        // for wallet on each network -> anvil nodes
        // for test wallet on each anvil nodes
        for addresses in contracts.clone() {
            for (_address, key) in TEST_WALLETS[0..wallets_count].iter() {
                transfer::exec_ethers(
                    Arc::clone(&config),
                    wallets[i].clone(),
                    key.to_string(),
                    addresses[0].clone(),
                    500,
                    provider.clone(),
                )
                .await?;
            }
        }
    }

    Ok(())
}
