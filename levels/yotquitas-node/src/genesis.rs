use yotquitas_core::{Block, BlockHeader, Address};
use std::collections::HashMap;

/// Genesis block configuration
pub struct GenesisConfig {
    pub treasury_address: Address,
    pub initial_supply: u64,
    pub timestamp: u64,
}

impl Default for GenesisConfig {
    fn default() -> Self {
        Self {
            treasury_address: [0u8; 32], // Will be set from keypair
            initial_supply: 1_000_000_000_000_000_000, // 1 billion AEQ (with 9 decimals)
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

/// Create the genesis block
pub fn create_genesis_block(config: GenesisConfig) -> Block {
    // Genesis block has index 0 and zero previous hash
    let header = BlockHeader::new(0, config.timestamp, [0u8; 32]);

    // Genesis block contains no transactions (treasury is pre-funded in state)
    Block::new(header, vec![])
}

/// Get initial account balances for genesis
pub fn get_initial_balances(config: &GenesisConfig) -> HashMap<Address, u64> {
    let mut balances = HashMap::new();
    balances.insert(config.treasury_address, config.initial_supply);
    balances
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genesis_block_creation() {
        let config = GenesisConfig::default();
        let genesis = create_genesis_block(config);

        assert_eq!(genesis.index(), 0);
        assert_eq!(genesis.previous_hash(), [0u8; 32]);
    }
}
