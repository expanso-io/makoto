//! Signing support for Makoto attestations.
//!
//! Provides ECDSA P-256 signing for L2+ attestations.

use crate::error::{MakotoError, Result};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use p256::ecdsa::{
    signature::{Signer, Verifier},
    Signature, SigningKey, VerifyingKey,
};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};

/// A signed attestation envelope (DSSE format).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SignedAttestation {
    /// The payload type (always "application/vnd.in-toto+json").
    pub payload_type: String,

    /// Base64-encoded payload (the attestation JSON).
    pub payload: String,

    /// Signatures over the payload.
    pub signatures: Vec<AttestationSignature>,
}

impl SignedAttestation {
    /// Create a signed attestation from a payload.
    pub fn sign<T: Serialize>(attestation: &T, signer: &MakotoSigner) -> Result<Self> {
        let payload_json = serde_json::to_string(attestation)?;
        let payload_b64 = BASE64.encode(payload_json.as_bytes());

        // DSSE signing: sign "DSSEv1 <payloadType> <payload>"
        let pae = format!(
            "DSSEv1 {} {}",
            "application/vnd.in-toto+json", payload_b64
        );

        let signature = signer.sign(pae.as_bytes())?;

        Ok(Self {
            payload_type: "application/vnd.in-toto+json".to_string(),
            payload: payload_b64,
            signatures: vec![AttestationSignature {
                keyid: signer.key_id().to_string(),
                sig: BASE64.encode(signature.to_bytes()),
            }],
        })
    }

    /// Get the decoded payload.
    pub fn decode_payload<T: for<'de> Deserialize<'de>>(&self) -> Result<T> {
        let payload_bytes = BASE64
            .decode(&self.payload)
            .map_err(|e| MakotoError::InvalidAttestation(format!("Invalid base64: {}", e)))?;

        serde_json::from_slice(&payload_bytes).map_err(MakotoError::from)
    }

    /// Verify all signatures.
    ///
    /// Returns `Ok(true)` if at least one signature from the verifier's key is valid.
    /// Returns `Ok(false)` if no matching key is found or signature verification fails.
    pub fn verify(&self, verifier: &MakotoVerifier) -> Result<bool> {
        let pae = format!("DSSEv1 {} {}", self.payload_type, self.payload);
        let mut found_matching_key = false;

        for sig in &self.signatures {
            if sig.keyid == verifier.key_id() {
                found_matching_key = true;

                let sig_bytes = BASE64
                    .decode(&sig.sig)
                    .map_err(|e| MakotoError::Signature(format!("Invalid signature base64: {}", e)))?;

                let signature = Signature::from_slice(&sig_bytes)
                    .map_err(|e| MakotoError::Signature(format!("Invalid signature format: {}", e)))?;

                if !verifier.verify(pae.as_bytes(), &signature)? {
                    return Ok(false);
                }
            }
        }

        // Return false if we didn't find any signature from this verifier
        Ok(found_matching_key)
    }
}

/// A single signature in an attestation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttestationSignature {
    /// Key identifier.
    pub keyid: String,

    /// Base64-encoded signature.
    pub sig: String,
}

/// A signer for creating attestation signatures.
#[derive(Debug)]
pub struct MakotoSigner {
    signing_key: SigningKey,
    key_id: String,
}

impl MakotoSigner {
    /// Generate a new random signing key.
    pub fn generate() -> Self {
        let signing_key = SigningKey::random(&mut OsRng);
        let key_id = compute_key_id(signing_key.verifying_key());

        Self {
            signing_key,
            key_id,
        }
    }

    /// Create a signer from a PEM-encoded private key.
    pub fn from_pem(pem: &str) -> Result<Self> {
        let signing_key = SigningKey::from_slice(
            &pem_to_der(pem)
                .map_err(|e| MakotoError::KeyError(format!("Invalid PEM: {}", e)))?,
        )
        .map_err(|e| MakotoError::KeyError(format!("Invalid private key: {}", e)))?;

        let key_id = compute_key_id(signing_key.verifying_key());

        Ok(Self {
            signing_key,
            key_id,
        })
    }

    /// Create a signer from raw key bytes.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let signing_key = SigningKey::from_slice(bytes)
            .map_err(|e| MakotoError::KeyError(format!("Invalid private key bytes: {}", e)))?;

        let key_id = compute_key_id(signing_key.verifying_key());

        Ok(Self {
            signing_key,
            key_id,
        })
    }

    /// Get the key ID.
    pub fn key_id(&self) -> &str {
        &self.key_id
    }

    /// Get the verifying (public) key.
    pub fn verifying_key(&self) -> MakotoVerifier {
        MakotoVerifier {
            verifying_key: *self.signing_key.verifying_key(),
            key_id: self.key_id.clone(),
        }
    }

    /// Sign data.
    pub fn sign(&self, data: &[u8]) -> Result<Signature> {
        Ok(self.signing_key.sign(data))
    }

    /// Export the private key as bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.signing_key.to_bytes().to_vec()
    }

    /// Export the public key as bytes.
    pub fn public_key_bytes(&self) -> Vec<u8> {
        self.signing_key
            .verifying_key()
            .to_encoded_point(false)
            .as_bytes()
            .to_vec()
    }
}

/// A verifier for checking attestation signatures.
#[derive(Debug, Clone)]
pub struct MakotoVerifier {
    verifying_key: VerifyingKey,
    key_id: String,
}

impl MakotoVerifier {
    /// Create a verifier from a PEM-encoded public key.
    pub fn from_pem(pem: &str) -> Result<Self> {
        let der = pem_to_der(pem)
            .map_err(|e| MakotoError::KeyError(format!("Invalid PEM: {}", e)))?;

        Self::from_bytes(&der)
    }

    /// Create a verifier from raw key bytes (SEC1 or compressed point format).
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        let verifying_key = VerifyingKey::from_sec1_bytes(bytes)
            .map_err(|e| MakotoError::KeyError(format!("Invalid public key bytes: {}", e)))?;

        let key_id = compute_key_id(&verifying_key);

        Ok(Self {
            verifying_key,
            key_id,
        })
    }

    /// Get the key ID.
    pub fn key_id(&self) -> &str {
        &self.key_id
    }

    /// Verify a signature.
    pub fn verify(&self, data: &[u8], signature: &Signature) -> Result<bool> {
        match self.verifying_key.verify(data, signature) {
            Ok(()) => Ok(true),
            Err(_) => Ok(false),
        }
    }

    /// Export the public key as bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.verifying_key
            .to_encoded_point(false)
            .as_bytes()
            .to_vec()
    }
}

/// Compute a key ID from a verifying key (SHA-256 of the public key bytes).
fn compute_key_id(key: &VerifyingKey) -> String {
    let bytes = key.to_encoded_point(false);
    let hash = crate::hash::sha256_hex(bytes.as_bytes());
    // Use first 16 characters as key ID
    hash[..16].to_string()
}

/// Very basic PEM to DER conversion (for private keys).
fn pem_to_der(pem: &str) -> std::result::Result<Vec<u8>, &'static str> {
    let lines: Vec<&str> = pem
        .lines()
        .filter(|l| !l.starts_with("-----"))
        .collect();

    let b64 = lines.join("");

    BASE64.decode(b64).map_err(|_| "Invalid base64 in PEM")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Digest, OriginAttestation, Subject};
    use crate::types::origin::{Collector, Origin};
    use crate::types::common::{CollectionMethod, SourceType};
    use chrono::Utc;

    #[test]
    fn test_signer_generation() {
        let signer = MakotoSigner::generate();
        assert_eq!(signer.key_id().len(), 16);
    }

    #[test]
    fn test_sign_and_verify() {
        let signer = MakotoSigner::generate();
        let verifier = signer.verifying_key();

        let data = b"test data to sign";
        let signature = signer.sign(data).unwrap();

        assert!(verifier.verify(data, &signature).unwrap());
    }

    #[test]
    fn test_signed_attestation() {
        let signer = MakotoSigner::generate();

        let origin = Origin::new(
            "https://api.example.com/data",
            SourceType::Api,
            CollectionMethod::Pull,
            Utc::now(),
        );

        let collector = Collector::new("https://example.com/collector/001");

        let attestation = OriginAttestation::builder()
            .subject(Subject::new(
                "dataset:test",
                Digest::new("a".repeat(64)),
            ))
            .origin(origin)
            .collector(collector)
            .build()
            .unwrap();

        let signed = SignedAttestation::sign(&attestation, &signer).unwrap();

        assert_eq!(signed.payload_type, "application/vnd.in-toto+json");
        assert!(!signed.signatures.is_empty());

        // Verify
        let verifier = signer.verifying_key();
        assert!(signed.verify(&verifier).unwrap());

        // Decode and check
        let decoded: OriginAttestation = signed.decode_payload().unwrap();
        assert_eq!(decoded.predicate.origin.source, attestation.predicate.origin.source);
    }

    #[test]
    fn test_signer_roundtrip() {
        let signer = MakotoSigner::generate();
        let bytes = signer.to_bytes();

        let restored = MakotoSigner::from_bytes(&bytes).unwrap();
        assert_eq!(signer.key_id(), restored.key_id());
    }

    #[test]
    fn test_invalid_signature_fails() {
        let signer1 = MakotoSigner::generate();
        let signer2 = MakotoSigner::generate();

        let data = b"test data";
        let signature = signer1.sign(data).unwrap();

        // Different verifier should fail
        let verifier2 = signer2.verifying_key();
        assert!(!verifier2.verify(data, &signature).unwrap());
    }
}
