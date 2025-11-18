# Level 2: The Decentralized Network (The City Grid)

This level takes the "single skyscraper" from Level 1 and decentralizes it. We give our node the ability to discover, connect to, and sync with thousands of other Yotquitas nodes.

### Analogy: The City Grid

We've perfected the "Yotquitas Tower" (Level 1). Now we're building 1,000 copies and connecting them all with a "city grid" of P2P roads, mail routes, and a phone network. The city can now run without a central operator.

### "Why?" (The Link to the Next Level)

This is the most critical step for any real blockchain. It removes the single point of failure (centralization) from Level 1. This turns our "product" into a "protocol." The platform is now resilient and censorship-resistant.

### Core Components & Specs

- Core Logic: This level is a direct upgrade to the yotquitas-node from Level 1.
- P2P Stack (The "Roads"):

- Spec: rust-libp2p (as per our blueprint). This is the battle-tested Rust networking stack.

- P2P Protocols (The "Services"):

1.  Peer Discovery (Kademlia): The "GPS" for nodes to find each other on the internet.
2.  Gossip (Gossipsub): The "mail system" to broadcast new transactions to the network's mempool and new Blocks to all peers.

- Consensus (The "First City Law"):

- Spec: Proof-of-Authority (PoA). To start, this is a simple, temporary consensus. We (the founders) will hard-code a list of 20 "authority" public keys. Only these nodes are trusted to create and sign new blocks.
- Fork-Choice: We will use a simple "Longest Chain Rule" for resolving forks.

- Mempool: A thread-safe, in-memory (Arc<Mutex<...>>) pool in each node to store pending transactions received from the Gossipsub network.

### Developer Use Case

- "Dapp Developer" (Diana): This is a massive upgrade for Diana.
- Use Case: Diana no longer has to trust your API. She can now run her own Yotquitas node and point her Dapp's API calls to localhost.
- The Benefit: Her app is now fully decentralized. If your node (or any other node) goes offline, her app keeps working by talking to its own local node or any other node in the "city grid."
