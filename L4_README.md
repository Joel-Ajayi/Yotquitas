# Level 4: The Public Metropolis (The Full Economy)

This is the final, "north star" vision for Yotquitas. This level takes the fast, fair, and decentralized city from Level 3 and transforms it into a permissionless, self-sustaining public economy.

### A Note on This Goal

Achieving this level is a massive, multi-year engineering undertaking, equivalent to building a public L1 from scratch. It is the "grand vision" that we are building towards. While we are writing it off as an immediate goal, every preceding level is architected specifically to make this final milestone possible.

### Analogy: The Public Metropolis

The "city" is now fully functional and provably fair. This level is about opening it to the public. We create a city-wide currency (AEQ). We allow anyone to build their own "factories" (deploy contracts) and anyone to apply to help run the "city hall" (become a validator) to earn a profit.

### "Why?" (The Final Goal)

This makes the platform permissionless (no one can stop you from building) and self-sustaining (validators are paid to secure the network). This is the only way to create a truly decentralized public utility.

## Installation

### Prerequisites

- Level 3 (`yotquitas-node` with Fino-Narwhal-Tusk) must be complete
- Rust 1.70+
- ZK proof libraries (Nova or arkworks)
- Node.js 18+ (for SDK development)

### Build from Source

```bash
cd levels/yotquitas-node
cargo build --release --features pos-parallel-zk-sdk
```

### Run a Validator Node

```bash
# Run with full Level 4 features
cargo run --release --features pos-parallel-zk-sdk \
  -- --validator --stake 1000000  # Stake 1M AEQ
```

## Quick Start

### 1. Become a Validator

```bash
# Generate validator keys
yotquitas-node generate-validator-keys

# Stake AEQ tokens
yotquitas-node stake --amount 1000000 --validator-key validator.key

# Start validator node
yotquitas-node --validator --stake-amount 1000000
```

### 2. Deploy a Contract

```rust
use aequitas_client::Client;

let client = Client::new("http://localhost:8545");

// Deploy Move contract
let contract = client.deploy_contract(
    "./contracts/MyContract.move",
    &deployer_keypair,
).await?;
```

### 3. Use the SDK

```rust
use aequitas_client::{Client, Account};

let client = Client::new("http://localhost:8545");
let account = Account::from_keypair(&keypair);

// Send transaction
let tx_hash = client.send_transaction(
    &account,
    TransactionPayload::Transfer {
        to: recipient,
        amount: 1000,
    },
).await?;
```

## Core Components & Specs

### 1. Economic Layer (The "Economy")

**Proof-of-Stake (PoS)**

- **Replaces**: PoA from Level 2 (already replaced by Fino-Narwhal-Tusk in L3)
- **Token**: AEQ (native token)
- **Staking**: Stake AEQ to become validator
- **Rewards**: Transaction fees + staking rewards
- **Slashing**: Penalties for misbehavior

**Token Economics**:

- Initial supply: 1 billion AEQ
- Inflation: 5% annually (to validators)
- Transaction fees: Burned or distributed to validators
- Staking minimum: 1M AEQ

### 2. Execution Upgrade: "Move-Sealevel" Static Scheduler

**Purpose**: Parallel transaction execution for maximum throughput.

**How it Works**:

1. After consensus, analyze transaction dependencies
2. Build dependency graph
3. Identify independent transactions
4. Execute in parallel on multiple MoveVM instances
5. Merge results back to state

**Implementation**:

- Static analysis of transaction reads/writes
- Multi-threaded execution (rayon)
- Conflict detection and resolution

**Performance**:

- Target: 10x improvement over sequential
- Scales with CPU cores

### 3. State Upgrade: Protocol-Native ZK Compression

**Purpose**: Solve state bloat by compressing "dust" accounts.

**How it Works**:

1. Users identify compressible accounts (e.g., empty token accounts)
2. Generate ZK proof of account state
3. Compress into Merkle root (32 bytes)
4. Original accounts can be pruned
5. Decompression via ZK proof when needed

**Implementation**:

- Nova or arkworks for ZK proofs
- Merkle tree for compressed state
- On-demand decompression

**Benefits**:

- Reduces state size by 100x+
- Lowers node storage costs
- Maintains full security

### 4. Developer Ecosystem: The "Factory Kit"

#### Anchor-Move Framework

Rust framework for writing safe Move contracts:

```rust
use anchor_move::prelude::*;

#[program]
pub mod my_contract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        // Contract logic
        Ok(())
    }
}
```

#### DeFi-Native Asset Library

Protocol-level Move library:

- `YieldBearingToken`: Interest-bearing tokens
- `CollateralizedPosition`: Collateral management
- `LiquidityPool`: AMM pools
- `Oracle`: Price feeds

#### aequitas-client SDK

Full-featured Rust client:

```rust
use aequitas_client::{Client, Account, TransactionBuilder};

let client = Client::new("http://localhost:8545");
let account = Account::from_keypair(&keypair);

// Build and send transaction
let tx = TransactionBuilder::new()
    .transfer(recipient, amount)
    .fee(1000)
    .build(&account)?;

let hash = client.send_transaction(&tx).await?;
```

## Configuration

### PoS Configuration

```toml
[staking]
# Minimum stake to become validator
min_stake = 1_000_000  # 1M AEQ

# Staking rewards
annual_inflation = 0.05  # 5%
reward_distribution = "proportional"  # or "equal"

# Slashing
slash_threshold = 0.33  # Slash if 33% misbehavior
slash_percentage = 0.10  # Slash 10% of stake
```

### Parallel Execution Configuration

```toml
[execution.parallel]
# Enable parallel execution
enabled = true

# Number of worker threads
worker_threads = 8

# Batch size for parallel processing
batch_size = 100

# Conflict resolution
conflict_strategy = "abort"  # or "retry"
```

### ZK Compression Configuration

```toml
[state.compression]
# Enable ZK compression
enabled = true

# Compression threshold (accounts to compress)
threshold = 1000

# ZK proof system
proof_system = "nova"  # or "arkworks"

# Compression cost (gas)
compression_cost = 10000
```

## Architecture

```
┌─────────────────────────────────────────────┐
│         Developer SDKs                      │
│  - Anchor-Move Framework                   │
│  - aequitas-client                         │
│  - DeFi Asset Library                      │
└──────────────┬──────────────────────────────┘
               │
┌──────────────▼──────────────────────────────┐
│         PoS Validator Selection             │
│  - Stake-weighted selection                │
│  - Slashing mechanism                       │
└──────────────┬──────────────────────────────┘
               │
┌──────────────▼──────────────────────────────┐
│    Fino-Narwhal-Tusk (Level 3)              │
└──────────────┬──────────────────────────────┘
               │
┌──────────────▼──────────────────────────────┐
│    Parallel Execution Scheduler             │
│  - Dependency analysis                      │
│  - Multi-threaded MoveVM                    │
└──────────────┬──────────────────────────────┘
               │
┌──────────────▼──────────────────────────────┐
│    ZK-Compressed State                      │
│  - Merkle roots for dust accounts          │
│  - On-demand decompression                  │
└─────────────────────────────────────────────┘
```

## Performance Metrics

### Target Performance

- **Throughput**: 1,000,000+ TPS (with parallel execution)
- **Latency**: < 2 seconds (transaction to finality)
- **State Size**: < 100GB (with ZK compression)
- **Validator Count**: 100-1000 validators

### Benchmarking

```bash
# Benchmark parallel execution
cargo bench --features pos-parallel-zk-sdk --bench parallel

# Benchmark ZK compression
cargo bench --features pos-parallel-zk-sdk --bench compression

# Load test
cargo test --features pos-parallel-zk-sdk --test load
```

## Development

### Project Structure

```
yotquitas/
├── levels/
│   └── yotquitas-node/
│       ├── src/
│       │   ├── staking.rs        # PoS implementation
│       │   ├── economics.rs      # Token economics
│       │   ├── scheduler.rs      # Parallel execution
│       │   └── zk_compression.rs # ZK state compression
│       └── sdk/
│           ├── anchor-move/      # Anchor-Move framework
│           ├── aequitas-client/ # Rust client SDK
│           └── defi-assets/     # DeFi asset library
└── website/                     # Next.js documentation site
```

### Running Tests

```bash
# Unit tests
cargo test --features pos-parallel-zk-sdk

# Integration tests
cargo test --features pos-parallel-zk-sdk --test integration

# SDK tests
cd sdk/aequitas-client && cargo test
```

## SDK Documentation

### Anchor-Move

See [Anchor-Move Documentation](https://docs.yotquitas.dev/anchor-move) for:

- Framework setup
- Contract examples
- Best practices
- Security guidelines

### aequitas-client

See [Client SDK Documentation](https://docs.yotquitas.dev/client-sdk) for:

- API reference
- Usage examples
- Error handling
- Advanced features

## Troubleshooting

### Common Issues

**Issue**: Cannot become validator

- **Solution**: Ensure minimum stake (1M AEQ), verify validator keys

**Issue**: Parallel execution conflicts

- **Solution**: Adjust batch size, check dependency analysis

**Issue**: ZK compression fails

- **Solution**: Verify proof system setup, check account eligibility

**Issue**: SDK connection errors

- **Solution**: Verify node is running, check API endpoint

## Migration from Level 3

To upgrade from Level 3 to Level 4:

1. **Token Distribution**: Distribute AEQ tokens to initial validators
2. **Enable PoS**: Configure staking parameters
3. **Deploy SDKs**: Publish SDK packages
4. **Upgrade Nodes**: All nodes must upgrade
5. **Testnet Launch**: Deploy to testnet first

**Breaking Changes**:

- New transaction types (staking, delegation)
- Contract deployment changes
- API additions (staking endpoints)

## Developer Use Case

**Contract Developer (Diana)**: This is the ultimate "developer."

**Use Case**: Diana can now use our Anchor-Move framework to write her own novel DeFi protocol (e.g., a "no-front-run" NFT marketplace). She can permissionlessly deploy this new smart contract to the Yotquitas network.

**The Benefit**: She can now build a business on Yotquitas, not just an "app." The platform is now a complete, self-sustaining, permissionless economy.

### Example: Deploy a DEX

```rust
use anchor_move::prelude::*;
use defi_assets::LiquidityPool;

#[program]
pub mod dex {
    pub fn create_pool(
        ctx: Context<CreatePool>,
        token_a: Pubkey,
        token_b: Pubkey,
    ) -> Result<()> {
        let pool = LiquidityPool::new(token_a, token_b);
        ctx.accounts.pool.set(pool)?;
        Ok(())
    }
}
```

## Resources

- [Documentation Website](https://yotquitas.dev)
- [SDK Documentation](https://docs.yotquitas.dev/sdk)
- [API Reference](https://docs.yotquitas.dev/api)
- [Move Contract Guide](https://docs.yotquitas.dev/move)
- [Validator Guide](https://docs.yotquitas.dev/validators)

## Roadmap

Level 4 is the "north star" - a long-term vision. Implementation phases:

- **Phase 1**: PoS implementation (6 months)
- **Phase 2**: Parallel execution (6 months)
- **Phase 3**: ZK compression (6 months)
- **Phase 4**: SDK ecosystem (ongoing)
- **Phase 5**: Mainnet launch (TBD)

## Conclusion

Level 4 represents the full vision of Yotquitas: a fast, fair, secure, and permissionless blockchain. This level transforms the platform from a protocol into a complete economy where developers can build, validators can secure, and users can transact with confidence.
