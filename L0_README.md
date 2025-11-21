# Level 0: The Core Ledger (The Blueprints)

This level is the foundational crate for the entire Yotquitas project. It is not a runnable program but a Rust library that defines the core data structures, or "atoms," of our blockchain.

### Analogy: The Architectural Blueprints

This is not a building. This is the set of architectural blueprints and physics principles (crypto) for the entire city. Before we can build a "skyscraper" (Level 1), we must first define what a "floor" (Block), a "room" (Transaction), and "unbreakable glass" (Signature) are.

### "Why?" (The Link to the Next Level)

This library is the core dependency that all other levels will import. It ensures that every single component in the entire Yotquitas ecosystem (every node, every wallet, every tool) speaks the same language and agrees on the same fundamental data shapes.

## Installation

### Prerequisites

- Rust 1.70+ ([install from rustup.rs](https://rustup.rs))

### Add as Dependency

```toml
# In your Cargo.toml
[dependencies]
yotquitas-core = { path = "../yotquitas/levels/yotquitas-core" }
# Or from crates.io (when published)
# yotquitas-core = "0.1.0"
```

### Build from Source

```bash
cd levels/yotquitas-core
cargo build
cargo test
```

## Core Components & Specs

### Module Structure

- **block.rs**: Block and BlockHeader structures
- **transaction.rs**: Transaction and TransactionPayload definitions
- **crypto.rs**: Cryptographic utilities (hashing, signatures)

### Block Structure

```rust
pub struct BlockHeader {
    pub index: u64,
    pub timestamp: u64,
    pub previous_hash: Hash,
    pub merkle_root: Hash,
}

pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}
```

### Transaction Structure

```rust
pub struct Transaction {
    pub sender_pubkey: PublicKey,
    pub signature: Signature,
    pub payload: TransactionPayload,
    pub fee: u64,
}

pub enum TransactionPayload {
    MoveCall { module: String, function: String, args: Vec<u8> },
    Transfer { to: Address, amount: u64 },
    // ... other variants
}
```

### Cryptographic Functions

- **Hashing**: SHA-256 via `sha2` crate
- **Signatures**: Ed25519 via `ed25519-dalek` crate
- **Serialization**: JSON and binary via `serde`

## Quick Start Example

```rust
use yotquitas_core::{Block, Transaction, crypto::*};

// Create a transaction
let tx = Transaction::new(
    sender_pubkey,
    TransactionPayload::Transfer { to, amount },
    fee,
);

// Sign the transaction
let signed_tx = tx.sign(&private_key);

// Create a block
let block = Block::new(
    BlockHeader::new(index, timestamp, previous_hash),
    vec![signed_tx],
);

// Serialize to JSON
let json = serde_json::to_string(&block)?;
```

## API Reference

### Block

- `Block::new(header, transactions)` - Create a new block
- `Block::hash()` - Compute block hash
- `Block::verify()` - Verify block integrity

### Transaction

- `Transaction::new(pubkey, payload, fee)` - Create a new transaction
- `Transaction::sign(private_key)` - Sign a transaction
- `Transaction::verify()` - Verify transaction signature

### Crypto Utilities

- `hash(data: &[u8]) -> Hash` - Compute SHA-256 hash
- `sign(message: &[u8], private_key: &PrivateKey) -> Signature` - Create signature
- `verify(message: &[u8], signature: &Signature, public_key: &PublicKey) -> bool` - Verify signature

## Serialization

All data structures implement `Serialize` and `Deserialize` from `serde`:

```rust
// JSON serialization
let json = serde_json::to_string(&block)?;
let block: Block = serde_json::from_str(&json)?;

// Binary serialization (bincode)
let bytes = bincode::serialize(&block)?;
let block: Block = bincode::deserialize(&bytes)?;
```

## Development

### Running Tests

```bash
cargo test
cargo test -- --nocapture  # Show output
```

### Generating Documentation

```bash
cargo doc --open
```

### Code Structure

```
yotquitas-core/
├── src/
│   ├── lib.rs          # Module exports
│   ├── block.rs        # Block structures
│   ├── transaction.rs  # Transaction structures
│   └── crypto.rs       # Cryptographic utilities
└── tests/              # Integration tests
```

## Configuration

This is a pure library crate with no runtime configuration needed. All serialization formats and cryptographic algorithms are fixed:

- **Hash Algorithm**: SHA-256
- **Signature Scheme**: Ed25519
- **Serialization**: Serde with JSON and bincode support

## Troubleshooting

### Common Issues

**Issue**: `cannot find crate 'yotquitas_core'`

- **Solution**: Ensure you're in the workspace root or have added the dependency correctly

**Issue**: Serialization errors

- **Solution**: Ensure all fields are properly marked with `#[derive(Serialize, Deserialize)]`

**Issue**: Signature verification fails

- **Solution**: Ensure you're using the same keypair for signing and verification

## Developer Use Case

- **Blockchain Builder**: This crate (`cargo add yotquitas-core`) is the core dependency for the next level.
- **Dapp Developer**: This published crate can be used as the core for their own blockchain implementations.

## Next Steps

Once Level 0 is complete, proceed to [Level 1: The Execution Node](L1_README.md) which uses this core library to build a runnable node.
