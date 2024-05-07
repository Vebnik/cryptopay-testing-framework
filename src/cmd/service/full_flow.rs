use std::{error::Error, sync::Arc};

use crate::config::Config;
use crate::{
    cmd, cmd::api::asset, cmd::api::network, cmd::api::user, cmd::api::utils, cmd::api::wallet,
    cmd::evm::deploy,
};

pub async fn exec(config: Arc<Config>) -> Result<(), Box<dyn Error>> {
    let test_user_name = "test_user";
    let test_user_email = "test_user@localhost.com";
    let test_wallet_pass = "test1234test";

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
        wallet::create::exec(Arc::clone(&config), network_id, test_wallet_pass.into()).await?;
    }

    let contracts =
        deploy::exec(Arc::clone(&config), "Test USDT".into(), "TUSDT".into(), 100).await?;

    for (i, contract_adress) in contracts.iter().enumerate() {
        asset::create::exec(
            Arc::clone(&config),
            networks_id[i].clone(),
            "Test USDT".into(),
            "TUSDT".into(),
            contract_adress.clone(),
        )
        .await?;
    }

    Ok(())
}
