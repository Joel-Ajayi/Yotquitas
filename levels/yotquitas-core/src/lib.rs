//! Yotquitas Core Library
//! 
//! This crate provides the foundational data structures and cryptographic
//! primitives for the Yotquitas blockchain.

pub mod block;
pub mod transaction;
pub mod crypto;

// Re-export commonly used types
pub use block::{Block, BlockHeader, compute_merkle_root};
pub use transaction::{Transaction, TransactionPayload, Address};
pub use crypto::{Hash, PublicKey, Signature, sha256, double_sha256, generate_keypair, sign, verify, encode_hex, decode_hex};
pub use ed25519_dalek::SigningKey;
