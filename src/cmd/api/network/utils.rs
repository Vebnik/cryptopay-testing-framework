use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::{cmd, Config, Result};

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Network {
    pub id: Uuid,
    pub name: String,
    pub kind: String,
}

pub async fn check_networks_exist(config: Arc<Config>) -> Result<Vec<String>> {
    let ids =
        cmd::api::network::create::exec(Arc::clone(&config), "LOCAL_EVM".into(), "EVM".into())
            .await?;

    return Ok(ids);
}
