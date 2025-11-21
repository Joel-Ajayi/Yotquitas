use crate::crypto::{double_sha256, Hash};
use crate::transaction::Transaction;
use serde::{Deserialize, Serialize};

/// Block header structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlockHeader {
    pub index: u64,
    pub timestamp: u64,
    pub previous_hash: Hash,
    pub merkle_root: Hash,
}

impl BlockHeader {
    /// Create a new block header
    pub fn new(index: u64, timestamp: u64, previous_hash: Hash) -> Self {
        Self {
            index,
            timestamp,
            previous_hash,
            merkle_root: [0u8; 32], // Will be computed when transactions are added
        }
    }

    /// Compute the hash of the block header
    pub fn hash(&self) -> Hash {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&self.index.to_le_bytes());
        bytes.extend_from_slice(&self.timestamp.to_le_bytes());
        bytes.extend_from_slice(&self.previous_hash);
        bytes.extend_from_slice(&self.merkle_root);
        double_sha256(&bytes)
    }
}

/// Block structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

impl Block {
    /// Create a new block
    pub fn new(header: BlockHeader, transactions: Vec<Transaction>) -> Self {
        let mut block = Self {
            header,
            transactions,
        };
        block.update_merkle_root();
        block
    }

    /// Update the merkle root in the header based on transactions
    fn update_merkle_root(&mut self) {
        self.header.merkle_root = compute_merkle_root(&self.transactions);
    }

    /// Get the block hash (hash of the header)
    pub fn hash(&self) -> Hash {
        self.header.hash()
    }

    /// Verify block integrity
    pub fn verify(&self) -> bool {
        // Verify merkle root matches transactions
        let computed_root = compute_merkle_root(&self.transactions);
        if computed_root != self.header.merkle_root {
            return false;
        }

        // Verify all transactions
        for tx in &self.transactions {
            if !tx.verify() {
                return false;
            }
        }

        true
    }

    /// Get the previous block hash
    pub fn previous_hash(&self) -> Hash {
        self.header.previous_hash
    }

    /// Get the block index
    pub fn index(&self) -> u64 {
        self.header.index
    }
}

/// Compute Merkle root from transactions
pub fn compute_merkle_root(transactions: &[Transaction]) -> Hash {
    if transactions.is_empty() {
        return [0u8; 32];
    }

    if transactions.len() == 1 {
        return transactions[0].hash();
    }

    // Build Merkle tree bottom-up
    let mut hashes: Vec<Hash> = transactions.iter().map(|tx| tx.hash()).collect();

    while hashes.len() > 1 {
        let mut next_level = Vec::new();

        // Process pairs
        for i in (0..hashes.len()).step_by(2) {
            // if length is odd, i+1 will exceed length
            if i + 1 < hashes.len() {
                // Pair: hash both together
                let mut combined = Vec::new();
                combined.extend_from_slice(&hashes[i]);
                combined.extend_from_slice(&hashes[i + 1]);
                next_level.push(double_sha256(&combined));
            } else {
                // Odd one out: hash with itself
                let mut combined = Vec::new();
                combined.extend_from_slice(&hashes[i]);
                combined.extend_from_slice(&hashes[i]);
                next_level.push(double_sha256(&combined));
            }
        }

        hashes = next_level;
    }

    hashes[0]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::generate_keypair;
    use crate::transaction::{Transaction, TransactionPayload};

    #[test]
    fn test_block_header_creation() {
        let prev_hash = [1u8; 32];
        let header = BlockHeader::new(0, 1000, prev_hash);

        assert_eq!(header.index, 0);
        assert_eq!(header.timestamp, 1000);
        assert_eq!(header.previous_hash, prev_hash);
    }

    #[test]
    fn test_block_header_hash() {
        let header1 = BlockHeader::new(0, 1000, [0u8; 32]);
        let header2 = BlockHeader::new(0, 1000, [0u8; 32]);

        // Same header should have same hash
        assert_eq!(header1.hash(), header2.hash());

        // Different index should have different hash
        let header3 = BlockHeader::new(1, 1000, [0u8; 32]);
        assert_ne!(header1.hash(), header3.hash());
    }

    #[test]
    fn test_block_creation() {
        let (_, pubkey) = generate_keypair();
        let tx = Transaction::new(
            pubkey,
            TransactionPayload::Transfer {
                to: [0u8; 32],
                amount: 100,
            },
            1,
            0,
        );

        let header = BlockHeader::new(0, 1000, [0u8; 32]);
        let block = Block::new(header, vec![tx]);

        assert_eq!(block.transactions.len(), 1);
        assert_ne!(block.header.merkle_root, [0u8; 32]);
    }

    #[test]
    fn test_merkle_root_empty() {
        let root = compute_merkle_root(&[]);
        assert_eq!(root, [0u8; 32]);
    }

    #[test]
    fn test_merkle_root_single() {
        let (_, pubkey) = generate_keypair();
        let tx = Transaction::new(
            pubkey,
            TransactionPayload::Transfer {
                to: [0u8; 32],
                amount: 100,
            },
            1,
            0,
        );

        let root = compute_merkle_root(&[tx.clone()]);
        assert_eq!(root, tx.hash());
    }

    #[test]
    fn test_merkle_root_multiple() {
        let (_, pubkey) = generate_keypair();
        let tx1 = Transaction::new(
            pubkey,
            TransactionPayload::Transfer {
                to: [0u8; 32],
                amount: 100,
            },
            1,
            0,
        );
        let tx2 = Transaction::new(
            pubkey,
            TransactionPayload::Transfer {
                to: [1u8; 32],
                amount: 200,
            },
            1,
            1,
        );

        let root = compute_merkle_root(&[tx1.clone(), tx2.clone()]);
        // Root should be deterministic
        let root2 = compute_merkle_root(&[tx1, tx2]);
        assert_eq!(root, root2);
    }

    #[test]
    fn test_block_verification() {
        let (signing_key, pubkey) = generate_keypair();
        let tx = Transaction::new(
            pubkey,
            TransactionPayload::Transfer {
                to: [0u8; 32],
                amount: 100,
            },
            1,
            0,
        )
        .sign(&signing_key);

        let header = BlockHeader::new(0, 1000, [0u8; 32]);
        let block = Block::new(header, vec![tx]);

        assert!(block.verify());
    }
}
