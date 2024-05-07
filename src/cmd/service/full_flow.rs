use std::{error::Error, sync::Arc};

use crate::config::Config;
use crate::{
    cmd, cmd::api::asset, cmd::api::intent, cmd::api::network, cmd::api::user, cmd::api::utils,
    cmd::api::wallet, cmd::evm::deploy, cmd::evm::mint,
};

#[derive(Debug, Clone)]
struct NetworkAsset {
    pub network_id: String,
    pub asset_id: String,
}

pub async fn exec(config: Arc<Config>) -> Result<(), Box<dyn Error>> {
    let test_user_name = "test_user";
    let test_user_email = "test_user@localhost.com";
    let test_wallet_pass = "test1234test";

    let mut wallets_id: Vec<String> = Vec::new();
    let mut assets_networks: Vec<NetworkAsset> = Vec::new();
    let mut intents_id: Vec<String> = Vec::new();

    cmd::db::drop::exec(Arc::clone(&config)).await?;

    utils::user::check_exist_system_user(Arc::clone(&config)).await?;

    user::create::exec(
        Arc::clone(&config),
        test_user_name.into(),
        false,
        Some(test_user_email.into()),
    )
    .await?;

    let networks_id =
        network::create::exec(Arc::clone(&config), "Local ETH".into(), "EVM".into()).await?;

    for network_id in networks_id.clone() {
        let wallet_id =
            wallet::create::exec(Arc::clone(&config), network_id, test_wallet_pass.into()).await?;
        wallets_id.push(wallet_id)
    }

    let contracts =
        deploy::exec(Arc::clone(&config), "Test USDT".into(), "TUSDT".into(), 100).await?;

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
            asset_id: asset_id,
        });
    }

    mint::exec(
        Arc::clone(&config),
        config.test_address.clone(),
        contracts[0].clone(),
        100,
    )
    .await?;

    for na in assets_networks {
        let id = intent::create::exec(
            Arc::clone(&config),
            na.network_id,
            na.asset_id,
            config.test_address.clone(),
        )
        .await?;

        intents_id.push(id);
    }




    Ok(())
}
