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
        #[arg(default_value = "Local ETH")]
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
        #[arg(default_value = "test@cryptopay.wtf")]
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
}

#[derive(Subcommand, Debug, Clone)]
pub enum ProcessType {
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
    /// Test process type
    #[command(subcommand)]
    pub process: ProcessType,
}
