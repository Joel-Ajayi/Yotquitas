# yotquitas-core

Core library for the Yotquitas blockchain providing foundational data structures, cryptographic primitives, and transaction handling. You can utilize this library if it matches your architecture.

## Features

- **Cryptographic Primitives**: SHA-256 hashing, Ed25519 digital signatures, key pair generation
- **Transaction Structures**: Transaction types with support for transfers, Move calls, and module deployment
- **Block Structures**: Block headers with Merkle root computation
- **Serialization**: Full serde support with JSON serialization/deserialization

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
yotquitas-core = "0.1.0"
```

## Quick Start

```rust
use yotquitas_core::{generate_keypair, Transaction, TransactionPayload};

// Generate a keypair
let (signing_key, pubkey) = generate_keypair();

// Create a transfer transaction
let payload = TransactionPayload::Transfer {
    to: [0u8; 32],  // recipient address
    amount: 100,     // amount in AEQ
};

let tx = Transaction::new(pubkey, payload, 1, 0);  // fee=1, nonce=0
let signed_tx = tx.sign(&signing_key);

// Verify the signature
assert!(signed_tx.verify());

// Serialize to JSON
let json = serde_json::to_string(&signed_tx).unwrap();
let deserialized: Transaction = serde_json::from_str(&json).unwrap();
```

## License

Licensed under either of:

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please see the [contributing guide](CONTRIBUTING.md) for more details.


