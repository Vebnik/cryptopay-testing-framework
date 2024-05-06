use std::{error::Error, sync::Arc};

use crate::config::State;
use crate::{
    cmd, cmd::api::asset, cmd::api::network, cmd::api::user, cmd::api::utils, cmd::api::wallet,
    cmd::evm::deploy,
};

pub async fn exec(state: Arc<State>) -> Result<(), Box<dyn Error>> {
    let test_user_name = "test_user";
    let test_user_email = "test_user@localhost.com";
    let test_wallet_pass = "test1234test";

    cmd::db::drop::exec(Arc::clone(&state)).await?;

    utils::user::check_exist_system_user(Arc::clone(&state)).await?;

    user::create::exec(
        Arc::clone(&state),
        test_user_name.into(),
        false,
        Some(test_user_email.into()),
    )
    .await?;

    let networks_id =
        network::create::exec("Local ETH".into(), "EVM".into(), Arc::clone(&state)).await?;

    for network_id in networks_id.clone() {
        wallet::create::exec(Arc::clone(&state), network_id, test_wallet_pass.into()).await?;
    }

    let contracts =
        deploy::exec(Arc::clone(&state), "Test USDT".into(), "TUSDT".into(), 100).await?;

    for (i, contract_adress) in contracts.iter().enumerate() {
        asset::create::exec(
            Arc::clone(&state),
            networks_id[i].clone(),
            "Test USDT".into(),
            "TUSDT".into(),
            contract_adress.clone(),
        )
        .await?;
    }

    Ok(())
}
