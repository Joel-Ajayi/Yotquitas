# Level 1: The Execution Node (The First Skyscraper)

This is the first runnable, usable product in the Yotquitas plan. This level takes the "blueprints" from Level 0 and builds a single, functional, centralized "node" server.

### Analogy: The First Skyscraper

We have built the first "Yotquitas Tower." It's a standalone marvel of engineering. It runs its own internal power and security. It has:

- A Hyper-Secure Vault: The MoveVM (Execution Layer)
- A Live Status-Board: The RocksDB database (State Layer)
- A Public Lobby: The JSON-RPC API (API Layer)

### "Why?" (The Link to the Next Level)

This level builds and proves the "brain" of the entire blockchain. Before we can build a decentralized city (Level 2), we must first perfect the design of a single building. This node is the "brain" that will be copied and given to all participants in the P2P network.

## Installation

### Prerequisites

- Rust 1.70+
- Level 0 (`yotquitas-core`) must be built
- Move VM dependencies (Aptos or Sui Move VM)

### Build from Source

```bash
cd levels/yotquitas-node
cargo build --release
```

### Run the Node

```bash
# Default configuration
cargo run --release

# With custom config
cargo run --release -- --config config/dev.toml
```

### Docker

```bash
docker build -t yotquitas-node .
docker run -p 8545:8545 yotquitas-node
```

## Quick Start

### 1. Start the Node

```bash
cd levels/yotquitas-node
cargo run --release
```

The node will start on `http://localhost:8545` with JSON-RPC enabled.

### 2. Send a Transaction

```bash
curl -X POST http://localhost:8545 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "eth_sendRawTransaction",
    "params": ["0x..."],
    "id": 1
  }'
```

### 3. Query Account Balance

```bash
curl -X POST http://localhost:8545 \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "aequitas_getAccountBalance",
    "params": ["0x..."],
    "id": 1
  }'
```

## Core Components & Specs

### Execution Layer (The "Brain")

- **VM**: The MoveVM (Aptos or Sui implementation)
- **Interpreter**: Rust module that executes transactions via MoveVM
- **Precompiled Contracts**: SimpleToken.move and other contracts in genesis

### State Layer (The "Live Data")

- **Database**: RocksDB for persistent storage
- **Model**: Solana-style Flat Account Model (key-value store)
- **State**: World state including all Move resources

### API Layer (The "Lobby")

- **Runtime**: tokio for async concurrency
- **Server**: axum web framework
- **Protocol**: JSON-RPC 2.0 with batching support
- **Port**: Default 8545 (configurable)

## JSON-RPC API Reference

### Supported Methods

#### `eth_sendRawTransaction`

Send a signed transaction to the node.

**Parameters:**

- `raw_transaction` (string, hex): Signed transaction bytes

**Returns:**

- `transaction_hash` (string, hex): Transaction hash

**Example:**

```json
{
  "jsonrpc": "2.0",
  "method": "eth_sendRawTransaction",
  "params": ["0x..."],
  "id": 1
}
```

#### `eth_getBlockByNumber`

Get block information by block number.

**Parameters:**

- `block_number` (string|number): Block number or "latest"
- `full_transactions` (boolean): Include full transaction objects

**Returns:**

- `block` (object): Block data with transactions

**Example:**

```json
{
  "jsonrpc": "2.0",
  "method": "eth_getBlockByNumber",
  "params": ["latest", true],
  "id": 1
}
```

#### `aequitas_getAccountBalance`

Get account balance (Yotquitas-specific method).

**Parameters:**

- `address` (string, hex): Account address

**Returns:**

- `balance` (string): Account balance in base units

**Example:**

```json
{
  "jsonrpc": "2.0",
  "method": "aequitas_getAccountBalance",
  "params": ["0x..."],
  "id": 1
}
```

## Configuration

### Configuration File

Create `config/dev.toml`:

```toml
[api]
host = "0.0.0.0"
port = 8545

[database]
path = "./data/rocksdb"

[vm]
move_stdlib_path = "./move/stdlib"
precompiled_contracts = ["./contracts/SimpleToken.move"]

[genesis]
initial_validators = []
initial_balances = {}
```

### Environment Variables

```bash
export YOTQUITAS_API_PORT=8545
export YOTQUITAS_DB_PATH=./data
export YOTQUITAS_LOG_LEVEL=info
```

### Command Line Options

```bash
yotquitas-node \
  --config config/dev.toml \
  --api-port 8545 \
  --db-path ./data \
  --log-level debug
```

## Architecture

```
┌─────────────────────────────────────┐
│         JSON-RPC API (axum)         │
│         Port: 8545                  │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│      Transaction Processor          │
│      (Validates & Routes)           │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│         MoveVM Interpreter          │
│      (Executes Transactions)        │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│      State Manager (RocksDB)        │
│    (Reads/Writes Account State)     │
└─────────────────────────────────────┘
```

## Development

### Project Structure

```
yotquitas-node/
├── src/
│   ├── main.rs          # Entry point
│   ├── node.rs          # Core node logic
│   ├── vm.rs            # MoveVM integration
│   ├── state.rs         # RocksDB state management
│   ├── api.rs           # JSON-RPC handlers
│   └── genesis.rs       # Genesis block setup
├── config/              # Configuration files
├── contracts/           # Move contracts
└── data/                # Database directory
```

### Running Tests

```bash
cargo test
cargo test -- --nocapture
```

### Debugging

Enable debug logging:

```bash
RUST_LOG=debug cargo run --release
```

Or in code:

```rust
env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();
```

## Troubleshooting

### Common Issues

**Issue**: Port 8545 already in use

- **Solution**: Change port in config or use `--api-port` flag

**Issue**: MoveVM initialization fails

- **Solution**: Ensure Move stdlib path is correct in config

**Issue**: Database corruption

- **Solution**: Delete `data/` directory and restart (will regenerate from genesis)

**Issue**: Transaction execution fails

- **Solution**: Check logs with `RUST_LOG=debug`, verify transaction format

### Log Levels

- `error`: Only errors
- `warn`: Warnings and errors
- `info`: Informational messages (default)
- `debug`: Detailed debugging information
- `trace`: Very verbose output

## Performance Tuning

### Database

- Use SSD storage for RocksDB
- Adjust RocksDB options in `state.rs` for your workload
- Monitor database size and compaction

### API

- Enable connection pooling in axum
- Use async handlers for all endpoints
- Consider rate limiting for production

### MoveVM

- Cache compiled modules
- Reuse VM instances when possible
- Profile execution time

## Developer Use Case

**Dapp Developer (Diana)**: Diana can now build a fully functional application on your platform.

**Use Cases:**

1. **Wallet**: She can build a React/JS wallet that calls `aequitas_getAccountBalance` to read a user's balance from your node's State Layer.
2. **Dapp**: Her app can call `eth_sendRawTransaction` to POST a transaction to your node's API. Your node will execute this using the MoveVM.

**The Limitation**: This platform is centralized. Diana must trust your single server. If your server goes down, her app goes down.

## Next Steps

Once Level 1 is complete and tested, proceed to [Level 2: The Decentralized Network](L2_README.md) to add P2P networking and remove the centralization limitation.
