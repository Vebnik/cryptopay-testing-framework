use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug, Clone)]
pub enum EvmCommands {
    /// Deploy new contract
    Deploy {
        #[arg(default_value = "Test USDT")]
        name: String,
        #[arg(default_value = "USDT")]
        symbol: String,
        #[arg(default_value = "100")]
        amount: u32,
    },
    /// Mint token for address
    Mint {
        /// Contract address
        contract: String,
        /// Target address
        address: String,
        #[arg(default_value = "100")]
        amount: u32,
    },
    /// Transfer token from address to address
    Tx {
        /// Chain index
        /// 0 - LOCAL_EVM_8545 (31337)
        /// 1 - LOCAL_EVM_8546 (31338)
        #[arg(long, default_value = "0")]
        chain_idx: u16,
        /// Contract address
        #[arg(long, default_value = "0x5fbdb2315678afecb367f032d93f642f64180aa3")]
        contract: String,
        /// Sender account index
        /// 0 - core wallet
        /// 1 - tester wallet
        /// 2 - tester wallet
        #[arg(long, default_value = "1")]
        sender_idx: u32,
        /// Recipient address
        #[arg(long, default_value = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266")]
        recipient: String,
        #[arg(long, default_value = "100")]
        amount: u32,
    },
    /// Spawn new local blockchain node
    Spawn {
        #[arg(default_value = "2")]
        amount: u32,
    },
}

#[derive(Subcommand, Debug, Clone)]
pub enum NetworkCommands {
    /// Create new network
    Create {
        #[arg(default_value = "LOCAL_EVM")]
        name: String,
        #[arg(default_value = "EVM")]
        kind: String,
    },
}

#[derive(Subcommand, Debug, Clone)]
pub enum UserCommands {
    Create {
        /// User name
        #[arg(default_value = "Tester")]
        name: String,
        /// User email
        #[arg(default_value = "tester@cryptopay.wtf")]
        email: String,
        /// Is admin
        #[arg(short)]
        admin: bool,
    },
}

#[derive(Subcommand, Debug, Clone)]
pub enum ApiCommands {
    /// Create user -> login -> create wallet ->  with wallet
    User {
        #[command(subcommand)]
        cmd: UserCommands,
    },
    /// Network api scope command
    Network {
        #[command(subcommand)]
        cmd: NetworkCommands,
    },
}

#[derive(Subcommand, Debug, Clone)]
pub enum DbCommands {
    /// Reset the database
    Reset,
    /// Migrate all data
    Create,
}

#[derive(Subcommand, Debug, Clone)]
pub enum ServiceCommands {
    /// Exec full flow
    /// Create user -> network -> deploy contract > create asset -> Create wallet
    Full,
    /// Prepare service
    ///
    /// -> Check and create admin user
    /// -> Check and create tester user
    /// -> Check and deploy tokens to core wallet
    /// -> Check and create networks
    /// -> Check and add assets to networks
    /// -> Check and create wallet for tester user
    Prepare,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Scope {
    /// EVM scope command
    Evm {
        #[command(subcommand)]
        cmd: EvmCommands,
    },
    /// API scope command
    Api {
        #[command(subcommand)]
        cmd: ApiCommands,
    },
    /// Datebase scope command
    Db {
        #[command(subcommand)]
        cmd: DbCommands,
    },
    /// Run all service logic for create, deploy and etc
    Service {
        #[command(subcommand)]
        cmd: ServiceCommands,
    },
}

/// CLI Test framework for cryptopay
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Scope type
    #[command(subcommand)]
    pub scope: Scope,
}
