use sha2::{Sha256, Digest};
use ed25519_dalek::{Signature as Ed25519Signature, Signer, SigningKey, VerifyingKey};
use hex;

/// Hash type (32 bytes for SHA-256)
pub type Hash = [u8; 32];

/// Public key type (Ed25519)
pub type PublicKey = VerifyingKey;

/// Signature type (Ed25519)
pub type Signature = Ed25519Signature;

/// Compute SHA-256 hash of data
pub fn sha256(data: &[u8]) -> Hash {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().into()
}

/// Compute double SHA-256 hash (SHA256(SHA256(data)))
pub fn double_sha256(data: &[u8]) -> Hash {
    sha256(&sha256(data))
}

/// Generate a new Ed25519 key pair
pub fn generate_keypair() -> (SigningKey, VerifyingKey) {
    use rand::rngs::OsRng;
    let signing_key = SigningKey::generate(&mut OsRng);
    let verifying_key = signing_key.verifying_key();
    (signing_key, verifying_key)
}

/// Sign data with a signing key
pub fn sign(signing_key: &SigningKey, data: &[u8]) -> Signature {
    signing_key.sign(data)
}

/// Verify a signature
pub fn verify(verifying_key: &VerifyingKey, data: &[u8], signature: &Signature) -> bool {
    verifying_key.verify_strict(data, signature).is_ok()
}

/// Encode bytes to hex string
pub fn encode_hex(bytes: &[u8]) -> String {
    hex::encode(bytes)
}

/// Decode hex string to bytes
pub fn decode_hex(hex_str: &str) -> Result<Vec<u8>, hex::FromHexError> {
    hex::decode(hex_str)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256() {
        let data = b"hello world";
        let hash = sha256(data);
        assert_eq!(hash.len(), 32);
        
        // Test determinism
        let hash2 = sha256(data);
        assert_eq!(hash, hash2);
    }

    #[test]
    fn test_double_sha256() {
        let data = b"test";
        let hash1 = sha256(data);
        let hash2 = double_sha256(data);
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_keypair_generation() {
        let (signing_key, verifying_key) = generate_keypair();
        assert_eq!(signing_key.verifying_key(), verifying_key);
    }

    #[test]
    fn test_sign_verify() {
        let (signing_key, verifying_key) = generate_keypair();
        let data = b"test message";
        
        let signature = sign(&signing_key, data);
        assert!(verify(&verifying_key, data, &signature));
        
        // Test with wrong data
        let wrong_data = b"wrong message";
        assert!(!verify(&verifying_key, wrong_data, &signature));
    }

    #[test]
    fn test_hex_encoding() {
        let data = b"hello";
        let encoded = encode_hex(data);
        assert_eq!(encoded, "68656c6c6f");
        
        let decoded = decode_hex(&encoded).unwrap();
        assert_eq!(decoded, data);
    }
}

