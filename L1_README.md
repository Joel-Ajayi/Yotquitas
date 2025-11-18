# Level 1: The Execution Node (The First Skyscraper)

#

This is the first runnable, usable product in the Yotquitas plan. This level takes the "blueprints" from Level 0 and builds a single, functional, centralized "node" server.

### Analogy: The First Skyscraper

#

We have built the first "Yotquitas Tower." It's a standalone marvel of engineering. It runs its own internal power and security. It has:

- A Hyper-Secure Vault: The MoveVM (Execution Layer)
- A Live Status-Board: The RocksDB database (State Layer)
- A Public Lobby: The JSON-RPC API (API Layer)

### "Why?" (The Link to the Next Level)

#

This level builds and proves the "brain" of the entire blockchain. Before we can build a decentralized city (Level 2), we must first perfect the design of a single building. This node is the "brain" that will be copied and given to all participants in the P2P network.

### Core Components & Specs

#

- Core Logic: Imports and uses yotquitas-core (Level 0).
- Execution Layer (The "Brain"):

- VM: The MoveVM. (As per our blueprint, for its superior asset safety).
- Interpreter: A Rust module that takes transactions and executes them using the MoveVM.
- Precompiled Contracts: We include our first Move contracts (like SimpleToken.move) directly in the node's genesis.

- State Layer (The "Live Data"):

- Database: RocksDB (for performance).
- Model: Solana-style Flat Account Model (key-value store) to hold the "World State," including all Move resources.

- API Layer (The "Lobby"):

- Runtime: tokio (for async M:N concurrency).
- Server: axum (for the web framework).
- Protocol: JSON-RPC (with batching), mimicking the Ethereum API (eth_sendRawTransaction, eth_getBlockByNumber) for easy developer adoption.

### Developer Use Case

#

- "Dapp Developer" (Diana): Diana can now build a fully functional application on your platform.
- Use Cases:

1.  Wallet: She can build a React/JS wallet that calls aequitas_getAccountBalance to read a user's balance from your node's "State Layer."
2.  Dapp: Her app can call eth_sendRawTransaction to POST a transaction to your node's API. Your node will execute this using the MoveVM.

- The Limitation: This platform is centralized. Diana must trust your single server. If your server goes down, her app goes down.
