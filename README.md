# Yotquitas (Level 0-4)

Yotquitas is a next-generation Layer-1 blockchain built in Rust, engineered for truly fair DeFi. Its core innovation is "Blind Order-Fairness," a consensus-level protocol that makes front-running (MEV) impossible. By combining high-speed async networking with the MoveVM's asset safety, Yotquitas is the fast, secure, and provably equitable platform for the future of finance.

## The Yotquitas Philosophy: A Monolithic L1

The current blockchain world is split between two designs:

1.  Modular (e.g., Ethereum): A slow, secure L1 "settlement layer" that relies on a complex, fragmented ecosystem of L2 "execution layers" (like Arbitrum). This breaks composability.
2.  Monolithic (e.g., Solana): A single, high-speed L1 that handles everything (execution, consensus, settlement). This provides perfect composability but has, to date, required trade-offs in fairness (MEV) and asset safety.

Yotquitas is a next-generation monolithic L1. Our goal is to build a single, ultra-fast platform that solves the trade-offs of the first generation. We are building a "monolithic megacity" that is simultaneously fast, fair, safe, and decentralized.

## Quick Start

### Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs))
- Git

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/yotquitas.git
cd yotquitas

# Build all workspace members
cargo build --release
```

### Running a Node (Level 1+)

```bash
# Run a single node
cd levels/yotquitas-node
cargo run --release

# Or use Docker
docker-compose up
```

The node will start on `http://localhost:8545` with JSON-RPC API enabled.

## Project Status

| Level   | Status         | Description                                        |
| ------- | -------------- | -------------------------------------------------- |
| Level 0 | 🟢 Active      | Core ledger library - foundational data structures |
| Level 1 | 🟡 In Progress | Execution node with MoveVM and JSON-RPC API        |
| Level 2 | ⚪ Planned     | Decentralized P2P network with PoA consensus       |
| Level 3 | ⚪ Planned     | High-performance Fino-Narwhal-Tusk consensus       |
| Level 4 | ⚪ Planned     | Full public network with PoS and SDKs              |

## The 5-Level Development Plan

This project is structured as a series of 5 distinct, usable, and presentable milestones. Each level builds the foundation for the next, evolving from a simple library into a full-scale public economy.

- [Level 0: The Core Ledger](L0_README.md)

- Status: The Blueprints
- What it is: A foundational Rust library (crate) defining the data structures (Block, Transaction).
- Developer Use Case: A "blockchain builder" (you).

- [Level 1: The Execution Node](L1_README.md)

- Status: The First Skyscraper
- What it is: A single, centralized server (node) with a JSON-RPC API and the MoveVM.
- Developer Use Case: A "Dapp developer" can build wallets/explorers that connect to your trusted node.

- [Level 2: The Decentralized Network](L2_README.md)

- Status: The City Grid
- What it is: A true peer-to-peer (P2P) network of Level 1 nodes that sync blocks and gossip transactions.
- Developer Use Case: A "Dapp developer" can now run their own node for a resilient, censorship-resistant app.

- [Level 3: The High-Performance Consensus](L3_README.md)

- Status: The Fair Maglev System
- What it is: Our first major innovation. We replace the basic consensus with our high-speed, MEV-resistant engine (Fino-Narwhal-Tusk).
- Developer Use Case: A "DeFi developer" can build apps (like DEXs) that are provably safe from front-running.

- [Level 4: The Public Metropolis](L4_README.md)

- Status: The North Star
- What it is: The full, public, permissionless network with Proof-of-Stake, parallel execution, and a developer SDK.
- Developer Use Case: A "Contract developer" can permissionlessly deploy their own smart contracts and build an economy.

## Architecture Overview

```
┌─────────────────────────────────────────────────────────┐
│                    Application Layer                     │
│              (DApps, Wallets, Explorers)                 │
└────────────────────┬────────────────────────────────────┘
                     │ JSON-RPC API
┌────────────────────▼────────────────────────────────────┐
│                    API Layer (Level 1+)                 │
│                    (axum, JSON-RPC)                     │
└────────────────────┬────────────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────────────┐
│              Execution Layer (Level 1+)                 │
│                    (MoveVM)                             │
└────────────────────┬────────────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────────────┐
│              Consensus Layer (Level 2+)                 │
│         PoA (L2) → Fino-Narwhal-Tusk (L3+)              │
└────────────────────┬────────────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────────────┐
│              Network Layer (Level 2+)                   │
│              (libp2p, Gossipsub, Kademlia)              │
└────────────────────┬────────────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────────────┐
│              State Layer (Level 1+)                     │
│                    (RocksDB)                            │
└────────────────────┬────────────────────────────────────┘
                     │
┌────────────────────▼────────────────────────────────────┐
│              Core Layer (Level 0)                       │
│         (Blocks, Transactions, Crypto)                  │
└─────────────────────────────────────────────────────────┘
```

## Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) for details.

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## Development

### Building from Source

```bash
# Build all crates
cargo build

# Build with optimizations
cargo build --release

# Run tests
cargo test

# Run tests for a specific level
cargo test -p yotquitas-core
```

### Project Structure

```
yotquitas/
├── levels/
│   ├── yotquitas-core/     # Level 0: Core library
│   └── yotquitas-node/      # Level 1-4: Node implementation
├── config/                  # Configuration files
├── scripts/                 # Build and deployment scripts
├── website/                 # Developer documentation site (Next.js)
├── Dockerfile               # Docker container for node
├── docker-compose.yml       # Docker Compose configuration
└── Cargo.toml              # Workspace configuration
└── Cargo.toml              # Workspace configuration
```

## Resources

- [Documentation Website](https://yotquitas.dev) (Coming Soon)
- [API Reference](https://docs.yotquitas.dev/api) (Coming Soon)
- [Download Binaries](https://github.com/yourusername/yotquitas/releases)
- [Discord Community](https://discord.gg/yotquitas) (Coming Soon)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Roadmap

- [x] Level 0: Core Ledger (Foundation)
- [ ] Level 1: Execution Node (Q1 2024)
- [ ] Level 2: Decentralized Network (Q2 2024)
- [ ] Level 3: High-Performance Consensus (Q3 2024)
- [ ] Level 4: Public Metropolis (Q4 2024+)
