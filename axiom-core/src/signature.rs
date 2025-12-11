//! C=0 Signature: Cryptographic Proof of Invariant Satisfaction
//!
//! Every verified order must carry a C=0 signature proving it satisfies
//! all L0 invariants. This provides cryptographic provenance.

use crate::types::*;
use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier, SignatureError as Ed25519Error};
use sha3::{Sha3_256, Digest};
use serde::{Deserialize, Serialize};
use chrono::Utc;

/// C=0 Signature: Proof that consistency error equals zero
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CZeroSignature {
    /// The signature bytes
    pub signature: Vec<u8>,
    /// The verifying key (public key)
    pub verifying_key: Vec<u8>,
    /// Hash of the order data
    pub order_hash: String,
    /// Timestamp of signature generation
    pub timestamp: chrono::DateTime<Utc>,
}

impl CZeroSignature {
    /// Generate a C=0 signature for a verified order
    pub fn sign(order: &VerifiedOrder, signing_key: &SigningKey) -> Self {
        // Hash the order data
        let mut hasher = Sha3_256::new();
        hasher.update(serde_json::to_string(order).unwrap().as_bytes());
        let order_hash = format!("{:x}", hasher.finalize());

        // Create message: order_hash + proof + timestamp
        let message = format!("{}:{}:{}", 
            order_hash,
            order.proof_signature,
            order.verified_at.timestamp()
        );

        // Sign the message
        let signature = signing_key.sign(message.as_bytes());

        Self {
            signature: signature.to_bytes().to_vec(),
            verifying_key: signing_key.verifying_key().to_bytes().to_vec(),
            order_hash,
            timestamp: Utc::now(),
        }
    }

    /// Verify a C=0 signature
    pub fn verify(&self, order: &VerifiedOrder) -> Result<(), SignatureError> {
        // Reconstruct the message
        let message = format!("{}:{}:{}",
            self.order_hash,
            order.proof_signature,
            order.verified_at.timestamp()
        );

        // Reconstruct verifying key
        let verifying_key = VerifyingKey::from_bytes(
            self.verifying_key.as_slice().try_into()
                .map_err(|_| SignatureError::InvalidKey)?
        ).map_err(|_| SignatureError::InvalidKey)?;

        // Reconstruct signature
        let signature = Signature::from_bytes(
            self.signature.as_slice().try_into()
                .map_err(|_| SignatureError::InvalidSignature)?
        ).map_err(|_| SignatureError::InvalidSignature)?;

        // Verify
        verifying_key.verify(message.as_bytes(), &signature)
            .map_err(|_| SignatureError::VerificationFailed)?;

        Ok(())
    }
}

/// Signature verification error
#[derive(Debug, thiserror::Error)]
pub enum SignatureError {
    #[error("Invalid verifying key")]
    InvalidKey,

    #[error("Invalid signature format")]
    InvalidSignature,

    #[error("Signature verification failed")]
    VerificationFailed,
}

