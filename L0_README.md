# Level 0: The Core Ledger (The Blueprints)

This level is the foundational crate for the entire Yotquitas project. It is not a runnable program but a Rust library that defines the core data structures, or "atoms," of our blockchain.

### Analogy: The Architectural Blueprints

This is not a building. This is the set of architectural blueprints and physics principles (crypto) for the entire city. Before we can build a "skyscraper" (Level 1), we must first define what a "floor" (Block), a "room" (Transaction), and "unbreakable glass" (Signature) are.

### "Why?" (The Link to the Next Level)

This library is the core dependency that all other levels will import. It ensures that every single component in the entire Yotquitas ecosystem (every node, every wallet, every tool) speaks the same language and agrees on the same fundamental data shapes.

### Core Components & Specs

- Crate: yotquitas-core (Rust Library)
- block.rs:

- BlockHeader struct: Contains index, timestamp, previous_hash, merkle_root, etc.
- Block struct: Contains a BlockHeader and transactions: Vec<Transaction>.

- transaction.rs:

- Transaction struct: Contains sender_pubkey, signature, payload, fee.
- TransactionPayload enum: Defines what a transaction can do.

- crypto.rs:

- Utility functions for hashing (using sha2).
- Signature creation and verification (using ed25519-dalek).

- Serialization:

- All data structures will #\[derive(Serialize, Deserialize)\] using serde for easy conversion to/from JSON and binary.

### Developer Use Case

- "Blockchain Builder" (You): You will use this crate (cargo add yotquitas-core) as the core dependency for your yotquitas-node in Level 1.
- "Dapp Developer" (Diana): She can do nothing with this. There is no server to connect to, no API to call.
