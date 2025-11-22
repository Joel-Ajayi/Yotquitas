use rocksdb::{DB, Options};
use yotquitas_core::Address;
use std::path::Path;
use anyhow::Result;
use std::sync::Arc;

/// State database wrapper around RocksDB
pub struct StateDB {
    db: Arc<DB>,
}

impl StateDB {
    /// Open or create a new state database
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.create_missing_column_families(true);

        let db = DB::open(&opts, path)?;
        Ok(Self {
            db: Arc::new(db),
        })
    }

    /// Get account balance
    pub fn get_balance(&self, address: &Address) -> Result<u64> {
        let key = format!("balance:{}", hex::encode(address));
        match self.db.get(key.as_bytes())? {
            Some(bytes) => {
                let balance = u64::from_le_bytes(
                    bytes
                        .try_into()
                        .map_err(|_| anyhow::anyhow!("Invalid balance bytes"))?,
                );
                Ok(balance)
            }
            None => Ok(0),
        }
    }

    /// Set account balance
    pub fn set_balance(&self, address: &Address, balance: u64) -> Result<()> {
        let key = format!("balance:{}", hex::encode(address));
        self.db.put(key.as_bytes(), balance.to_le_bytes())?;
        Ok(())
    }

    /// Get account nonce
    pub fn get_nonce(&self, address: &Address) -> Result<u64> {
        let key = format!("nonce:{}", hex::encode(address));
        match self.db.get(key.as_bytes())? {
            Some(bytes) => {
                let nonce = u64::from_le_bytes(
                    bytes
                        .try_into()
                        .map_err(|_| anyhow::anyhow!("Invalid nonce bytes"))?,
                );
                Ok(nonce)
            }
            None => Ok(0),
        }
    }

    /// Increment account nonce
    pub fn increment_nonce(&self, address: &Address) -> Result<u64> {
        let current_nonce = self.get_nonce(address)?;
        let new_nonce = current_nonce + 1;
        let key = format!("nonce:{}", hex::encode(address));
        self.db.put(key.as_bytes(), new_nonce.to_le_bytes())?;
        Ok(new_nonce)
    }

    /// Store a block
    pub fn store_block(&self, block_hash: &[u8; 32], block_data: &[u8]) -> Result<()> {
        let key = format!("block:{}", hex::encode(block_hash));
        self.db.put(key.as_bytes(), block_data)?;
        Ok(())
    }

    /// Get a block
    pub fn get_block(&self, block_hash: &[u8; 32]) -> Result<Option<Vec<u8>>> {
        let key = format!("block:{}", hex::encode(block_hash));
        Ok(self.db.get(key.as_bytes())?)
    }

    /// Get the latest block hash
    pub fn get_latest_block_hash(&self) -> Result<Option<[u8; 32]>> {
        match self.db.get(b"latest_block")? {
            Some(bytes) => {
                let hash: [u8; 32] = bytes
                    .try_into()
                    .map_err(|_| anyhow::anyhow!("Invalid block hash"))?;
                Ok(Some(hash))
            }
            None => Ok(None),
        }
    }

    /// Set the latest block hash
    pub fn set_latest_block_hash(&self, block_hash: &[u8; 32]) -> Result<()> {
        self.db.put(b"latest_block", block_hash)?;
        Ok(())
    }

    /// Initialize genesis state
    pub fn initialize_genesis(
        &self,
        initial_balances: &std::collections::HashMap<Address, u64>,
    ) -> Result<()> {
        for (address, balance) in initial_balances {
            self.set_balance(address, *balance)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_state_db_operations() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let db = StateDB::open(temp_dir.path())?;

        let address = [1u8; 32];

        // Test balance operations
        assert_eq!(db.get_balance(&address)?, 0);
        db.set_balance(&address, 100)?;
        assert_eq!(db.get_balance(&address)?, 100);

        // Test nonce operations
        assert_eq!(db.get_nonce(&address)?, 0);
        assert_eq!(db.increment_nonce(&address)?, 1);
        assert_eq!(db.get_nonce(&address)?, 1);

        Ok(())
    }
}
