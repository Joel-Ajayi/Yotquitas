# Level 2: The Decentralized Network (The City Grid)

This level takes the "single skyscraper" from Level 1 and decentralizes it. We give our node the ability to discover, connect to, and sync with thousands of other Yotquitas nodes.

### Analogy: The City Grid

We've perfected the "Yotquitas Tower" (Level 1). Now we're building 1,000 copies and connecting them all with a "city grid" of P2P roads, mail routes, and a phone network. The city can now run without a central operator.

### "Why?" (The Link to the Next Level)

This is the most critical step for any real blockchain. It removes the single point of failure (centralization) from Level 1. This turns our "product" into a "protocol." The platform is now resilient and censorship-resistant.

## Installation

### Prerequisites

- Level 1 (`yotquitas-node`) must be complete
- Rust 1.70+
- Network access for P2P connections

### Build from Source

```bash
cd levels/yotquitas-node
cargo build --release --features p2p
```

### Run a Node

```bash
# Run with P2P networking enabled
cargo run --release --features p2p

# With bootstrap peers
cargo run --release --features p2p -- --bootstrap /ip4/127.0.0.1/tcp/9000
```

## Quick Start

### 1. Start First Node (Bootstrap)

```bash
# Node 1 - acts as bootstrap
cargo run --release --features p2p -- --port 9000 --api-port 8545
```

### 2. Start Additional Nodes

```bash
# Node 2 - connects to Node 1
cargo run --release --features p2p \
  -- --port 9001 --api-port 8546 \
  --bootstrap /ip4/127.0.0.1/tcp/9000

# Node 3 - connects to network
cargo run --release --features p2p \
  -- --port 9002 --api-port 8547 \
  --bootstrap /ip4/127.0.0.1/tcp/9000
```

### 3. Verify Network Connection

Check logs for peer connections:

```
INFO  yotquitas_node::p2p] Connected to peer: /ip4/127.0.0.1/tcp/9000
INFO  yotquitas_node::p2p] Discovered 2 peers via Kademlia
```

## Core Components & Specs

### P2P Stack (The "Roads")

- **Library**: rust-libp2p (battle-tested Rust networking)
- **Transport**: TCP/IP with noise encryption
- **Multiplexing**: Yamux or mplex

### P2P Protocols (The "Services")

#### 1. Peer Discovery (Kademlia DHT)

- **Purpose**: Find other nodes on the network
- **Implementation**: libp2p-kad
- **Bootstrap**: Connect to known peers to join network

#### 2. Gossip (Gossipsub)

- **Purpose**: Broadcast transactions and blocks
- **Topics**:
  - `transactions`: New transactions for mempool
  - `blocks`: New blocks for chain sync
- **Implementation**: libp2p-gossipsub

### Consensus (The "First City Law")

- **Type**: Proof-of-Authority (PoA)
- **Validators**: Hard-coded list of 20 authority public keys
- **Block Production**: Round-robin or time-based
- **Fork Choice**: Longest chain rule

### Mempool

- **Type**: Thread-safe in-memory pool
- **Structure**: `Arc<Mutex<HashMap<TransactionHash, Transaction>>>`
- **Source**: Transactions received via Gossipsub
- **Eviction**: Time-based or size-based

## Configuration

### P2P Configuration

```toml
[p2p]
# Listening address
listen_address = "/ip4/0.0.0.0/tcp/9000"

# Bootstrap peers (for initial discovery)
bootstrap_peers = [
    "/ip4/192.168.1.100/tcp/9000/p2p/12D3KooW...",
    "/ip4/192.168.1.101/tcp/9000/p2p/12D3KooW...",
]

# Kademlia settings
kademlia_bucket_size = 20
kademlia_alpha = 3

# Gossipsub settings
gossipsub_mesh_n = 6
gossipsub_mesh_d = 4
```

### Consensus Configuration

```toml
[consensus]
# PoA validator public keys
authority_keys = [
    "0x1234...",
    "0x5678...",
    # ... 20 total
]

# Block production interval (seconds)
block_interval = 5

# Block size limit
max_block_size = 1024 * 1024  # 1MB
```

### Environment Variables

```bash
export YOTQUITAS_P2P_PORT=9000
export YOTQUITAS_BOOTSTRAP_PEERS="/ip4/127.0.0.1/tcp/9000"
export YOTQUITAS_LOG_LEVEL=info
```

## Architecture

```
┌─────────────────────────────────────────┐
│         JSON-RPC API (Level 1)          │
└──────────────┬──────────────────────────┘
               │
┌──────────────▼──────────────────────────┐
│         P2P Network Layer               │
│  ┌──────────┐  ┌──────────┐            │
│  │Kademlia │  │Gossipsub │            │
│  │  (DHT)  │  │ (Gossip) │            │
│  └──────────┘  └──────────┘            │
└──────────────┬──────────────────────────┘
               │
┌──────────────▼──────────────────────────┐
│         Consensus (PoA)                 │
│  - Validator Set                        │
│  - Block Production                     │
│  - Fork Choice                          │
└──────────────┬──────────────────────────┘
               │
┌──────────────▼──────────────────────────┐
│         Mempool                          │
│  (Thread-safe transaction pool)         │
└──────────────┬──────────────────────────┘
               │
┌──────────────▼──────────────────────────┐
│    Execution & State (Level 1)          │
└─────────────────────────────────────────┘
```

## Network Information

### Testnet

- **Network ID**: 100
- **Bootstrap Nodes**: (To be announced)
- **Block Time**: ~5 seconds
- **Chain ID**: `yotquitas-testnet-1`

### Mainnet

- **Status**: Not yet deployed
- **Network ID**: 1
- **Chain ID**: `yotquitas-mainnet-1`

## Development

### Project Structure

```
yotquitas-node/
├── src/
│   ├── p2p.rs           # P2P networking setup
│   ├── gossip.rs       # Gossipsub protocol
│   ├── discovery.rs    # Kademlia DHT
│   ├── consensus.rs    # PoA consensus
│   └── mempool.rs      # Transaction pool
└── config/
    └── network.toml    # Network configuration
```

### Running Tests

```bash
# Unit tests
cargo test --features p2p

# Integration tests (requires network)
cargo test --features p2p --test integration
```

### Debugging P2P

Enable verbose logging:

```bash
RUST_LOG=libp2p=debug,yotquitas_node::p2p=debug cargo run --release --features p2p
```

## Troubleshooting

### Common Issues

**Issue**: Cannot discover peers

- **Solution**: Check bootstrap peer addresses, ensure firewall allows P2P port

**Issue**: Mempool not receiving transactions

- **Solution**: Verify Gossipsub topic subscription, check network connectivity

**Issue**: Blocks not syncing

- **Solution**: Check consensus validator configuration, verify longest chain rule

**Issue**: High memory usage

- **Solution**: Implement mempool eviction policy, limit transaction pool size

### Network Diagnostics

```bash
# Check connected peers
curl http://localhost:8545 -X POST -d '{
  "jsonrpc": "2.0",
  "method": "net_peerCount",
  "id": 1
}'

# Get network info
curl http://localhost:8545 -X POST -d '{
  "jsonrpc": "2.0",
  "method": "net_version",
  "id": 1
}'
```

## Performance Considerations

### P2P Network

- Limit peer connections (default: 50-100)
- Use connection pooling
- Implement rate limiting for gossip

### Mempool

- Set size limits (e.g., 10,000 transactions)
- Implement TTL for old transactions
- Use efficient data structures (HashMap for O(1) lookup)

### Consensus

- Optimize block validation
- Cache validator set
- Batch signature verification

## Migration from Level 1

To upgrade from Level 1 to Level 2:

1. **Backup State**: Copy RocksDB data directory
2. **Update Code**: Pull Level 2 changes
3. **Configure P2P**: Set bootstrap peers in config
4. **Restart Node**: Node will automatically join network
5. **Verify**: Check logs for peer connections

**Breaking Changes**: None - Level 2 is backward compatible with Level 1 API.

## Developer Use Case

**Dapp Developer (Diana)**: This is a massive upgrade for Diana.

**Use Case**: Diana no longer has to trust your API. She can now run her own Yotquitas node and point her Dapp's API calls to localhost.

**The Benefit**: Her app is now fully decentralized. If your node (or any other node) goes offline, her app keeps working by talking to its own local node or any other node in the "city grid."

## Next Steps

Once Level 2 is stable, proceed to [Level 3: The High-Performance Consensus](L3_README.md) to replace PoA with the Fino-Narwhal-Tusk consensus for high-speed, MEV-resistant transaction ordering.
