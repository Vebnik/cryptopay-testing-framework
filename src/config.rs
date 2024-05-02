use std::default::Default;

use alloy::{
    network::Ethereum,
    node_bindings::AnvilInstance,
    providers::{fillers::{ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller}, Identity, RootProvider},
    transports::BoxTransport,
};
use sqlx::{Pool, Postgres};

#[derive(Debug, Clone)]
pub struct Config {
    pub bsc_network_endpoint: String,
    pub eth_network_endpoint: String,
    pub test_network_endpoint: String,
    pub database_url: String,
}

#[derive(Debug)]
pub struct State {
    pub anvil: AnvilInstance,
    pub config: Config,
    pub provider: FillProvider<JoinFill<JoinFill<JoinFill<Identity, GasFiller>, NonceFiller>, ChainIdFiller>, RootProvider<BoxTransport>, BoxTransport, Ethereum>,
    pub db: Pool<Postgres>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            bsc_network_endpoint: "wss://bsc-testnet.nodereal.io/ws/v1/a9ebae1dfb1d47339003ec1724a7dbf2".into(),
            eth_network_endpoint: "wss://sepolia.infura.io/ws/v3/221ed134273943b2b7bedb4b8377761b".into(),
            test_network_endpoint: "http://127.0.0.1:8545".into(),
            database_url: "postgres://postgres:postgres@localhost:5432/test".into(),
        }
    }
}