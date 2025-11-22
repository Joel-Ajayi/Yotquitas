use crate::state::StateDB;
use crate::vm::VM;
use crate::genesis::{create_genesis_block, get_initial_balances, GenesisConfig};
use yotquitas_core::{Block, Transaction, Address, Hash};
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Core node structure
pub struct Node {
    state: Arc<StateDB>,
    vm: Arc<VM>,
    current_block: Arc<RwLock<Option<Block>>>,
    chain_id: u64,
    network_id: u64,
}

impl Node {
    /// Create a new node instance
    pub fn new(
        state: Arc<StateDB>,
        genesis_config: &crate::GenesisConfigToml,
        chain_id: u64,
        network_id: u64,
    ) -> Result<Self> {
        // Initialize genesis block if needed
        if state.get_latest_block_hash()?.is_none() {
            // Convert hex treasury address to Address
            let treasury_address_str = genesis_config.treasury_address.trim_start_matches("0x");
            let treasury_address_bytes = hex::decode(treasury_address_str)
                .map_err(|e| anyhow::anyhow!("Invalid treasury address format: {}", e))?;

            if treasury_address_bytes.len() != 32 {
                anyhow::bail!(
                    "Treasury address must be 32 bytes, got {} bytes",
                    treasury_address_bytes.len()
                );
            }

            let treasury_address: Address = treasury_address_bytes
                .try_into()
                .map_err(|_| anyhow::anyhow!("Failed to convert treasury address to array"))?;

            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();

            let config = GenesisConfig {
                treasury_address,
                initial_supply: genesis_config.initial_supply,
                timestamp,
            };

            // Get balances before moving config
            let balances = get_initial_balances(&config);

            let genesis = create_genesis_block(config);
            let genesis_hash = genesis.hash();

            // Store genesis block
            let block_data = serde_json::to_vec(&genesis)?;
            state.store_block(&genesis_hash, &block_data)?;
            state.set_latest_block_hash(&genesis_hash)?;

            // Initialize balances
            state.initialize_genesis(&balances)?;

            tracing::info!("Genesis block created: {}", hex::encode(genesis_hash));
            tracing::info!("Treasury address: {}", hex::encode(treasury_address));
            tracing::info!("Initial supply: {} AEQ", genesis_config.initial_supply);
        }

        Ok(Self {
            state,
            vm: Arc::new(VM::new()),
            current_block: Arc::new(RwLock::new(None)),
            chain_id,
            network_id,
        })
    }

    /// Process a transaction
    pub async fn process_transaction(&self, tx: Transaction) -> Result<()> {
        // Validate transaction
        self.vm.validate_transaction(&tx)?;

        // Check nonce
        let sender_address = tx.sender_address();
        let expected_nonce = self.state.get_nonce(&sender_address)?;
        if tx.nonce != expected_nonce {
            anyhow::bail!(
                "Invalid nonce: expected {}, got {}",
                expected_nonce,
                tx.nonce
            );
        }

        // Check balance for fee
        let balance = self.state.get_balance(&sender_address)?;
        if balance < tx.fee {
            anyhow::bail!("Insufficient balance for fee");
        }

        // Execute transaction
        let result = self.vm.execute_transaction(&tx)?;
        if !result.success {
            anyhow::bail!("Transaction execution failed");
        }

        // Apply state changes
        match &tx.payload {
            yotquitas_core::TransactionPayload::Transfer { to, amount } => {
                // Deduct from sender
                let sender_balance = self.state.get_balance(&sender_address)?;
                if sender_balance < *amount + tx.fee {
                    anyhow::bail!("Insufficient balance");
                }
                self.state
                    .set_balance(&sender_address, sender_balance - amount - tx.fee)?;

                // Add to recipient
                let recipient_balance = self.state.get_balance(to)?;
                self.state.set_balance(to, recipient_balance + amount)?;
            }
            _ => {
                // For other transaction types, just deduct fee
                let sender_balance = self.state.get_balance(&sender_address)?;
                if sender_balance < tx.fee {
                    anyhow::bail!("Insufficient balance for fee");
                }
                self.state
                    .set_balance(&sender_address, sender_balance - tx.fee)?;
            }
        }

        // Increment nonce
        self.state.increment_nonce(&sender_address)?;

        tracing::info!("Transaction processed: {}", hex::encode(tx.hash()));
        Ok(())
    }

    /// Get account balance
    pub fn get_balance(&self, address: &Address) -> Result<u64> {
        self.state.get_balance(address)
    }

    /// Get the latest block
    pub async fn get_latest_block(&self) -> Result<Option<Block>> {
        match self.state.get_latest_block_hash()? {
            Some(hash) => match self.state.get_block(&hash)? {
                Some(block_data) => {
                    let block: Block = serde_json::from_slice(&block_data)?;
                    Ok(Some(block))
                }
                None => Ok(None),
            },
            None => Ok(None),
        }
    }

    /// Get block by hash
    pub fn get_block_by_hash(&self, hash: &Hash) -> Result<Option<Block>> {
        match self.state.get_block(hash)? {
            Some(block_data) => {
                let block: Block = serde_json::from_slice(&block_data)?;
                Ok(Some(block))
            }
            None => Ok(None),
        }
    }

    /// Get chain ID
    pub fn chain_id(&self) -> u64 {
        self.chain_id
    }

    /// Get network ID
    pub fn network_id(&self) -> u64 {
        self.network_id
    }
}
