# Level 3: The High-Performance Consensus (The Fair Maglev System)

This level is our first major innovation. The PoA consensus from Level 2 is simple, but it's slow, centralized, and unfair (validators can see transactions and front-run them). This level replaces that "First City Law" with the high-speed, provably-fair "maglev system" from our blueprint.

### Analogy: The Fair Maglev System

The "city grid" (Level 2) is built, but it uses slow stoplights and corruptible "mailmen" (validators) who read and re-order the mail (MEV).

We are ripping out that system and installing a high-speed maglev train. All mail is sent in encrypted, locked boxes (Fino). The system orders the boxes (Tusk) before anyone gets the key to open them. This makes the city both fast (100k+ TPS) and fair (no front-running).

### "Why?" (The Link to the Next Level)

For a DeFi-first L1 ("Aequitas"), Level 2 is not good enough. Speed is essential, but trust is paramount. By building "Blind Order-Fairness" into the protocol, we create a provably fair marketplace. This is a core feature that Ethereum and Solana lack, giving developers a compelling reason to build on Yotquitas.

## Installation

### Prerequisites

- Level 2 (`yotquitas-node` with P2P) must be complete
- Rust 1.70+
- BLS cryptography libraries (for threshold encryption)

### Build from Source

```bash
cd levels/yotquitas-node
cargo build --release --features narwhal-tusk-fino
```

### Run with New Consensus

```bash
# Run node with Fino-Narwhal-Tusk consensus
cargo run --release --features narwhal-tusk-fino
```

## Quick Start

### 1. Generate Validator Keys

```bash
# Generate threshold encryption key shares
yotquitas-node generate-keys --validators 20 --threshold 14
```

This creates:

- Public key for encryption (shared with users)
- Private key shares (distributed to validators)

### 2. Configure Validators

```toml
[consensus.narwhal]
# Narwhal DAG settings
round_timeout = 2  # seconds
max_batch_size = 1000

[consensus.tusk]
# Tusk BFT settings
committee_size = 20
fault_tolerance = 6  # 3f+1 = 20, so f=6

[consensus.fino]
# Fino threshold encryption
threshold = 14  # Need 14 of 20 to decrypt
public_key = "0x..."  # Global encryption key
```

### 3. Start Validator Nodes

```bash
# Validator 1
cargo run --release --features narwhal-tusk-fino \
  -- --validator --key-share keys/validator1.key

# Validator 2
cargo run --release --features narwhal-tusk-fino \
  -- --validator --key-share keys/validator2.key
```

## Core Components & Specs

### Component 1: Narwhal (DAG-based Mempool)

**Purpose**: Decouple data dissemination from ordering for high throughput.

**How it Works**:

1. Validators create batches of transactions
2. Batches form a Directed Acyclic Graph (DAG)
3. Each batch references previous batches (parents)
4. DAG ensures data availability before ordering

**Implementation**:

- Inspired by Mysten Labs' Narwhal
- Rust implementation with async/await
- Gossipsub for batch propagation

**Key Metrics**:

- Target: 100k+ TPS
- Batch size: 1000 transactions
- Round time: 2 seconds

### Component 2: Tusk (BFT Consensus)

**Purpose**: Agree on total order of batches with zero-message overhead.

**How it Works**:

1. Runs on top of Narwhal DAG
2. Validators vote on batch ordering
3. Byzantine Fault Tolerant (BFT)
4. Finalizes order without additional messages

**Implementation**:

- Tusk algorithm (HotStuff variant)
- 3f+1 fault tolerance
- Async consensus for speed

### Component 3: Fino (Blind Order-Fairness)

**Purpose**: Prevent validators from seeing transaction content before ordering.

**How it Works**:

1. **Encryption**: Users encrypt transactions with global public key
2. **Narwhal**: Gossip encrypted blobs in DAG
3. **Tusk**: Order encrypted blobs (validators can't read content)
4. **Decryption**: After ordering, validators collectively decrypt
5. **Execution**: Execute transactions in final order

**Implementation**:

- Threshold encryption (BLS-based)
- Requires threshold (e.g., 14 of 20) to decrypt
- Distributed key generation (DKG)

## Architecture

```
┌─────────────────────────────────────────────┐
│         User Transactions                   │
│    (Encrypted with Fino Public Key)         │
└──────────────┬──────────────────────────────┘
               │
┌──────────────▼──────────────────────────────┐
│         Narwhal (DAG Mempool)               │
│  - Validators create batches                │
│  - Form DAG structure                       │
│  - Gossip encrypted batches                │
└──────────────┬──────────────────────────────┘
               │
┌──────────────▼──────────────────────────────┐
│         Tusk (BFT Consensus)                 │
│  - Order batches (still encrypted)          │
│  - BFT agreement on order                   │
│  - Finalize sequence                        │
└──────────────┬──────────────────────────────┘
               │
┌──────────────▼──────────────────────────────┐
│         Fino (Threshold Decryption)          │
│  - Collect key shares from validators       │
│  - Decrypt transactions                     │
│  - Reveal content in final order            │
└──────────────┬──────────────────────────────┘
               │
┌──────────────▼──────────────────────────────┐
│         Execution (MoveVM)                  │
│  - Execute in final order                   │
│  - Update state                             │
└─────────────────────────────────────────────┘
```

## Configuration

### Narwhal Configuration

```toml
[narwhal]
# Batch settings
max_batch_size = 1000
max_batch_bytes = 1024 * 1024  # 1MB

# Round settings
round_timeout = 2  # seconds
max_rounds = 1000

# DAG settings
max_parents = 2
```

### Tusk Configuration

```toml
[tusk]
# Committee
committee_size = 20
fault_tolerance = 6  # f = (n-1)/3

# Consensus
timeout = 5  # seconds
max_rounds = 100
```

### Fino Configuration

```toml
[fino]
# Threshold encryption
threshold = 14  # Need 14 of 20
total_validators = 20

# Key management
public_key_path = "./keys/fino_public.key"
key_share_path = "./keys/validator_{id}.key"

# Decryption
decryption_timeout = 10  # seconds
```

## Performance Metrics

### Target Performance

- **Throughput**: 100,000+ TPS
- **Latency**: < 5 seconds (transaction to finality)
- **Fault Tolerance**: 6 Byzantine validators (of 20)

### Benchmarking

```bash
# Run performance tests
cargo test --features narwhal-tusk-fino --test performance

# Benchmark Narwhal
cargo bench --features narwhal-tusk-fino --bench narwhal

# Benchmark Tusk
cargo bench --features narwhal-tusk-fino --bench tusk
```

## Development

### Project Structure

```
yotquitas-node/
├── src/
│   ├── narwhal.rs      # Narwhal DAG implementation
│   ├── tusk.rs         # Tusk BFT consensus
│   ├── fino.rs         # Threshold encryption
│   └── consensus.rs    # Consensus coordinator
└── tests/
    ├── narwhal_test.rs
    ├── tusk_test.rs
    └── fino_test.rs
```

### Running Tests

```bash
# Unit tests
cargo test --features narwhal-tusk-fino

# Integration tests
cargo test --features narwhal-tusk-fino --test integration

# Fuzz tests
cargo fuzz run narwhal_fuzz
```

## Troubleshooting

### Common Issues

**Issue**: Narwhal DAG not forming

- **Solution**: Check network connectivity, verify batch creation, check round timeout

**Issue**: Tusk consensus not finalizing

- **Solution**: Verify validator count (3f+1), check Byzantine fault tolerance

**Issue**: Fino decryption fails

- **Solution**: Ensure threshold validators are online, verify key shares

**Issue**: Performance below target

- **Solution**: Optimize batch size, adjust timeouts, profile bottlenecks

### Debugging

Enable detailed logging:

```bash
RUST_LOG=narwhal=debug,tusk=debug,fino=debug cargo run --release --features narwhal-tusk-fino
```

## Security Considerations

### Threshold Encryption

- Use secure key generation (DKG protocol)
- Protect key shares (HSM or secure storage)
- Rotate keys periodically

### BFT Consensus

- Verify validator identities
- Monitor for Byzantine behavior
- Implement slashing for misbehavior

### MEV Prevention

- Validate encryption before ordering
- Ensure decryption happens after ordering
- Audit transaction ordering fairness

## Migration from Level 2

To upgrade from Level 2 to Level 3:

1. **Backup State**: Full state backup required
2. **Generate Keys**: Run DKG for threshold encryption
3. **Update Config**: Add Narwhal/Tusk/Fino configuration
4. **Upgrade Nodes**: All validators must upgrade simultaneously
5. **Verify**: Test with small network first

**Breaking Changes**:

- Consensus protocol change (PoA → Fino-Narwhal-Tusk)
- Transaction format change (encryption required)
- API remains compatible (JSON-RPC unchanged)

## Developer Use Case

**DeFi Developer (Diana)**: Diana can now build a high-frequency Decentralized Exchange (DEX) or a lending protocol.

**Use Case**: Build MEV-resistant DeFi applications

**The Benefit**: She can cryptographically guarantee to her users that they are 100% safe from front-running and "sandwich attacks" from validators or bots. This is the core "Aequitas" (fairness) promise.

### Example: Fair DEX

```rust
// User's transaction is encrypted
let encrypted_tx = fino::encrypt(transaction, public_key);

// Submit to network (validators can't read it)
node.submit_transaction(encrypted_tx);

// After consensus, transaction is executed in fair order
// No validator could front-run because they couldn't see the content
```

## Next Steps

Once Level 3 is stable and tested, proceed to [Level 4: The Public Metropolis](L4_README.md) to add Proof-of-Stake, parallel execution, and full developer SDKs.
