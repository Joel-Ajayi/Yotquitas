use anyhow::Result;
use clap::Parser;
use serde::Deserialize;
use std::sync::Arc;

mod api;
mod genesis;
mod node;
mod state;
mod vm;

use node::Node;
use state::StateDB;

/// Configuration structure matching TOML files
#[derive(Debug, Deserialize, Clone)]
struct Config {
    network: NetworkConfig,
    node: NodeConfig,
    genesis: GenesisConfigToml,
    logging: LoggingConfig,
}

#[derive(Debug, Deserialize, Clone)]
struct NetworkConfig {
    chain_id: u64,
    network_id: u64,
}

#[derive(Debug, Deserialize, Clone)]
struct NodeConfig {
    data_dir: String,
    rpc_addr: String,
    rpc_cors: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GenesisConfigToml {
    treasury_address: String,
    initial_supply: u64,
}

#[derive(Debug, Deserialize, Clone)]
struct LoggingConfig {
    level: String,
}

/// Yotquitas Node - Execution Node for Yotquitas Blockchain
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Configuration type: 'dev' or 'mainnet'
    #[arg(long, default_value = "dev")]
    config_type: String,
}

/// Load configuration from TOML file
fn load_config(config_type: &str) -> Result<Config> {
    let config_path = format!("config/{}.toml", config_type);
    let config_content = std::fs::read_to_string(&config_path)
        .map_err(|e| anyhow::anyhow!("Failed to read config file {}: {}", config_path, e))?;

    let config: Config = toml::from_str(&config_content)
        .map_err(|e| anyhow::anyhow!("Failed to parse config file: {}", e))?;

    Ok(config)
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments
    let args = Args::parse();

    // Load configuration
    let config = load_config(&args.config_type)?;

    // Initialize tracing with config level
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new(&config.logging.level));
    tracing_subscriber::fmt().with_env_filter(filter).init();

    tracing::info!("Starting Yotquitas Node...");
    tracing::info!("Configuration type: {}", args.config_type);
    tracing::info!("Chain ID: {}", config.network.chain_id);
    tracing::info!("Network ID: {}", config.network.network_id);
    tracing::info!("Data directory: {}", config.node.data_dir);
    tracing::info!("RPC address: {}", config.node.rpc_addr);

    // Initialize state database
    let state = Arc::new(StateDB::open(&config.node.data_dir)?);
    tracing::info!("State database initialized");

    // Create node with config
    let node = Arc::new(Node::new(
        state,
        &config.genesis,
        config.network.chain_id,
        config.network.network_id,
    )?);
    tracing::info!("Node initialized");

    // Create API router
    let app = api::create_router(node);

    // Start server
    let listener = tokio::net::TcpListener::bind(&config.node.rpc_addr).await?;
    tracing::info!("JSON-RPC server listening on {}", config.node.rpc_addr);

    axum::serve(listener, app).await?;

    Ok(())
}
