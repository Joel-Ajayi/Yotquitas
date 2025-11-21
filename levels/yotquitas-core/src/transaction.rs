use crate::crypto::{Hash, PublicKey, Signature, sha256, sign, verify};
use ed25519_dalek::{SigningKey, VerifyingKey};
use serde::{Deserialize, Serialize};

/// Address type (derived from public key hash)
pub type Address = Hash;

/// Helper module for serializing PublicKey as bytes (base64 in JSON)
mod pubkey_serde {
    use super::*;
    use serde::{Deserializer, Serializer};

    pub fn serialize<S>(pk: &PublicKey, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeSeq;
        let bytes = pk.to_bytes();
        let mut seq = serializer.serialize_seq(Some(bytes.len()))?;
        for byte in &bytes {
            seq.serialize_element(byte)?;
        }
        seq.end()
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<PublicKey, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::{self, Visitor};
        use std::fmt;

        struct BytesVisitor;

        impl<'de> Visitor<'de> for BytesVisitor {
            type Value = PublicKey;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a byte array of length 32")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let mut bytes = [0u8; 32];
                for i in 0..32 {
                    bytes[i] = seq
                        .next_element()?
                        .ok_or_else(|| de::Error::invalid_length(i, &self))?;
                }
                VerifyingKey::from_bytes(&bytes)
                    .map_err(|e| de::Error::custom(format!("invalid public key: {:?}", e)))
            }
        }

        deserializer.deserialize_seq(BytesVisitor)
    }
}

/// Helper module for serializing Option<Signature> as bytes (base64 in JSON)
mod option_signature_serde {
    use super::*;
    use serde::{Deserializer, Serializer};

    pub fn serialize<S>(opt: &Option<Signature>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match opt {
            Some(sig) => {
                use serde::ser::SerializeSeq;
                let bytes = sig.to_bytes();
                let mut seq = serializer.serialize_seq(Some(bytes.len()))?;
                for byte in &bytes {
                    seq.serialize_element(byte)?;
                }
                seq.end()
            }
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Signature>, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::{self, Visitor};
        use std::fmt;

        struct OptionBytesVisitor;

        impl<'de> Visitor<'de> for OptionBytesVisitor {
            type Value = Option<Signature>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an optional byte array of length 64")
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(None)
            }

            fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct BytesVisitor;

                impl<'de> Visitor<'de> for BytesVisitor {
                    type Value = Signature;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("a byte array of length 64")
                    }

                    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                    where
                        A: serde::de::SeqAccess<'de>,
                    {
                        let mut bytes = [0u8; 64];
                        for i in 0..64 {
                            bytes[i] = seq
                                .next_element()?
                                .ok_or_else(|| de::Error::invalid_length(i, &self))?;
                        }
                        Ok(ed25519_dalek::Signature::from_bytes(&bytes))
                    }
                }

                deserializer.deserialize_seq(BytesVisitor).map(Some)
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut bytes = [0u8; 64];
                for i in 0..64 {
                    bytes[i] = seq
                        .next_element()?
                        .ok_or_else(|| de::Error::invalid_length(i, &self))?;
                }
                Ok(Some(ed25519_dalek::Signature::from_bytes(&bytes)))
            }
        }

        deserializer.deserialize_option(OptionBytesVisitor)
    }
}

/// Transaction payload variants
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TransactionPayload {
    /// Move smart contract call
    MoveCall {
        module: String,
        function: String,
        args: Vec<u8>,
    },
    /// Simple token transfer
    Transfer { to: Address, amount: u64 },
    /// Deploy a Move module
    DeployModule { bytecode: Vec<u8> },
}

/// Transaction structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Transaction {
    #[serde(with = "pubkey_serde")]
    pub sender_pubkey: PublicKey,
    #[serde(
        with = "option_signature_serde",
        skip_serializing_if = "Option::is_none"
    )]
    pub signature: Option<Signature>,
    pub payload: TransactionPayload,
    pub fee: u64,
    pub nonce: u64,
}

impl Transaction {
    /// Create a new unsigned transaction
    pub fn new(
        sender_pubkey: PublicKey,
        payload: TransactionPayload,
        fee: u64,
        nonce: u64,
    ) -> Self {
        Self {
            sender_pubkey,
            signature: None,
            payload,
            fee,
            nonce,
        }
    }

    /// Sign the transaction with a signing key
    pub fn sign(mut self, signing_key: &SigningKey) -> Self {
        let tx_bytes = self.to_bytes_for_signing();
        let signature = sign(signing_key, &tx_bytes);
        self.signature = Some(signature);
        self
    }

    /// Verify the transaction signature
    pub fn verify(&self) -> bool {
        match &self.signature {
            Some(sig) => {
                let tx_bytes = self.to_bytes_for_signing();
                verify(&self.sender_pubkey, &tx_bytes, sig)
            }
            None => false,
        }
    }

    /// Get transaction hash
    pub fn hash(&self) -> Hash {
        let tx_bytes = self.to_bytes();
        sha256(&tx_bytes)
    }

    /// Get sender address (hash of public key)
    pub fn sender_address(&self) -> Address {
        let pubkey_bytes = self.sender_pubkey.to_bytes();
        sha256(&pubkey_bytes)
    }

    /// Serialize transaction for signing (without signature)
    fn to_bytes_for_signing(&self) -> Vec<u8> {
        // Create a version without signature for signing
        let tx_without_sig = Transaction {
            sender_pubkey: self.sender_pubkey,
            signature: None,
            payload: self.payload.clone(),
            fee: self.fee,
            nonce: self.nonce,
        };
        // Use serde_json for consistent serialization
        serde_json::to_vec(&tx_without_sig).unwrap_or_else(|_| Vec::new())
    }

    /// Serialize transaction to bytes using serde_json
    fn to_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).unwrap_or_else(|_| Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::generate_keypair;

    #[test]
    fn test_transaction_creation() {
        let (_, pubkey) = generate_keypair();
        let payload = TransactionPayload::Transfer {
            to: [0u8; 32],
            amount: 100,
        };

        let tx = Transaction::new(pubkey, payload, 1, 0);
        assert_eq!(tx.fee, 1);
        assert_eq!(tx.nonce, 0);
        assert!(tx.signature.is_none());
    }

    #[test]
    fn test_transaction_signing() {
        let (signing_key, pubkey) = generate_keypair();
        let payload = TransactionPayload::Transfer {
            to: [0u8; 32],
            amount: 100,
        };

        let tx = Transaction::new(pubkey, payload, 1, 0);
        let signed_tx = tx.sign(&signing_key);

        assert!(signed_tx.signature.is_some());
        assert!(signed_tx.verify());
    }

    #[test]
    fn test_transaction_verification() {
        let (signing_key, pubkey) = generate_keypair();
        let (_, wrong_pubkey) = generate_keypair();

        let payload = TransactionPayload::Transfer {
            to: [0u8; 32],
            amount: 100,
        };

        let tx = Transaction::new(pubkey, payload, 1, 0);
        let signed_tx = tx.sign(&signing_key);

        // Correct signature should verify
        assert!(signed_tx.verify());

        // Wrong public key should not verify
        let mut wrong_tx = signed_tx.clone();
        wrong_tx.sender_pubkey = wrong_pubkey;
        assert!(!wrong_tx.verify());
    }

    #[test]
    fn test_transaction_hash() {
        let (_, pubkey) = generate_keypair();
        let payload = TransactionPayload::Transfer {
            to: [0u8; 32],
            amount: 100,
        };

        let tx1 = Transaction::new(pubkey, payload.clone(), 1, 0);
        let tx2 = Transaction::new(pubkey, payload, 1, 0);

        // Same transaction should have same hash
        assert_eq!(tx1.hash(), tx2.hash());

        // Different nonce should have different hash
        let tx3 = Transaction::new(
            pubkey,
            TransactionPayload::Transfer {
                to: [0u8; 32],
                amount: 100,
            },
            1,
            1,
        );
        assert_ne!(tx1.hash(), tx3.hash());
    }

    #[test]
    fn test_transaction_serde_json() {
        let (signing_key, pubkey) = generate_keypair();
        let payload = TransactionPayload::Transfer {
            to: [0u8; 32],
            amount: 100,
        };

        let tx = Transaction::new(pubkey, payload, 1, 0);
        let signed_tx = tx.sign(&signing_key);

        // Serialize to JSON string (like your example)
        let json_string = serde_json::to_string(&signed_tx).expect("Should serialize");
        println!("Serialized JSON: {}", json_string);

        // Deserialize from JSON string (like your example)
        let deserialized_tx: Transaction =
            serde_json::from_str(&json_string).expect("Should deserialize");
        println!("Deserialized Transaction: {:?}", deserialized_tx);

        // Verify it matches
        assert_eq!(
            signed_tx.sender_pubkey.to_bytes(),
            deserialized_tx.sender_pubkey.to_bytes()
        );
        assert_eq!(
            signed_tx.signature.as_ref().map(|s| s.to_bytes()),
            deserialized_tx.signature.as_ref().map(|s| s.to_bytes())
        );
        assert_eq!(signed_tx.payload, deserialized_tx.payload);
        assert_eq!(signed_tx.fee, deserialized_tx.fee);
        assert_eq!(signed_tx.nonce, deserialized_tx.nonce);

        // Verify signature still works after deserialization
        assert!(deserialized_tx.verify());
    }
}
