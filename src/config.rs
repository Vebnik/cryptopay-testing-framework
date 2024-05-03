use std::default::Default;

use crate::cli::Args;

use alloy::{
    network::{Ethereum, EthereumSigner},
    providers::{
        fillers::{ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller, SignerFiller},
        Identity, RootProvider,
    },
    transports::BoxTransport,
};
use sqlx::{Pool, Postgres};

#[derive(Debug, Clone)]
pub struct Config {
    pub anvil_endpoint: String,
    pub cryptopay_url: String,
    pub db_host: String,
    pub db_connect_url: String,
    pub core_priv_key: String,
}

#[derive(Debug)]
pub struct State {
    pub config: Config,
    pub provider: FillProvider<
        JoinFill<
            JoinFill<JoinFill<JoinFill<Identity, GasFiller>, NonceFiller>, ChainIdFiller>,
            SignerFiller<EthereumSigner>,
        >,
        RootProvider<BoxTransport>,
        BoxTransport,
        Ethereum,
    >,
    pub db: Pool<Postgres>,
    pub args: Args
}

impl Default for Config {
    fn default() -> Self {
        Self {
            cryptopay_url: "http://127.0.0.1:9999".into(),
            anvil_endpoint: "http://127.0.0.1:8545".into(),
            db_host: "127.0.0.1:5432".into(),
            db_connect_url: "postgres://postgres:postgres@localhost:5432/test".into(),
            core_priv_key: "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"
                .into(),
        }
    }
}
