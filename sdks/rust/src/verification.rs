//! Verification utilities for Makoto attestations.
//!
//! Provides hash verification, attestation validation, and chain verification.

use crate::error::{MakotoError, Result};
use crate::hash::sha256_hex;
use crate::signing::{MakotoVerifier, SignedAttestation};
use crate::types::{
    Digest, MakotoLevel, OriginAttestation, StreamWindowAttestation, TransformAttestation,
    ORIGIN_PREDICATE_TYPE, STREAM_WINDOW_PREDICATE_TYPE, TRANSFORM_PREDICATE_TYPE,
};

/// Result of attestation verification.
#[derive(Debug, Clone)]
pub struct VerificationResult {
    /// Whether verification passed.
    pub valid: bool,
    /// Makoto level achieved.
    pub level: Option<MakotoLevel>,
    /// Verification messages.
    pub messages: Vec<String>,
    /// Warnings (non-fatal issues).
    pub warnings: Vec<String>,
}

impl VerificationResult {
    /// Create a passing result.
    pub fn pass(level: MakotoLevel) -> Self {
        Self {
            valid: true,
            level: Some(level),
            messages: vec![],
            warnings: vec![],
        }
    }

    /// Create a failing result.
    pub fn fail(message: impl Into<String>) -> Self {
        Self {
            valid: false,
            level: None,
            messages: vec![message.into()],
            warnings: vec![],
        }
    }

    /// Add a message.
    pub fn with_message(mut self, msg: impl Into<String>) -> Self {
        self.messages.push(msg.into());
        self
    }

    /// Add a warning.
    pub fn with_warning(mut self, warning: impl Into<String>) -> Self {
        self.warnings.push(warning.into());
        self
    }
}

/// Verify a digest against actual data.
pub fn verify_digest(digest: &Digest, data: &[u8]) -> Result<bool> {
    let computed = sha256_hex(data);

    if computed != digest.sha256 {
        return Err(MakotoError::HashMismatch {
            expected: digest.sha256.clone(),
            actual: computed,
        });
    }

    Ok(true)
}

/// Verify a digest against a hex-encoded hash.
pub fn verify_digest_hex(digest: &Digest, expected_sha256: &str) -> bool {
    digest.sha256.to_lowercase() == expected_sha256.to_lowercase()
}

/// Verify an origin attestation structure (L1 check).
pub fn verify_origin_structure(attestation: &OriginAttestation) -> VerificationResult {
    if let Err(e) = attestation.validate() {
        return VerificationResult::fail(format!("Structure validation failed: {}", e));
    }

    // Check required fields
    if attestation.predicate.origin.source.is_empty() {
        return VerificationResult::fail("Origin source is empty");
    }

    if attestation.subject.is_empty() {
        return VerificationResult::fail("No subjects in attestation");
    }

    // Validate subject digests
    for subject in &attestation.subject {
        if subject.digest.sha256.len() != 64 {
            return VerificationResult::fail(format!(
                "Invalid SHA-256 hash length for subject '{}': expected 64, got {}",
                subject.name,
                subject.digest.sha256.len()
            ));
        }
    }

    VerificationResult::pass(MakotoLevel::L1)
        .with_message("Origin attestation structure is valid")
}

/// Verify a transform attestation structure (L1 check).
pub fn verify_transform_structure(attestation: &TransformAttestation) -> VerificationResult {
    if let Err(e) = attestation.validate() {
        return VerificationResult::fail(format!("Structure validation failed: {}", e));
    }

    // Check inputs
    if attestation.predicate.inputs.is_empty() {
        return VerificationResult::fail("No inputs in transform attestation");
    }

    // Check transform definition
    if attestation.predicate.transform.name.is_empty() {
        return VerificationResult::fail("Transform name is empty");
    }

    // Validate digests
    for input in &attestation.predicate.inputs {
        if input.digest.sha256.len() != 64 {
            return VerificationResult::fail(format!(
                "Invalid SHA-256 hash length for input '{}': expected 64, got {}",
                input.name,
                input.digest.sha256.len()
            ));
        }
    }

    for subject in &attestation.subject {
        if subject.digest.sha256.len() != 64 {
            return VerificationResult::fail(format!(
                "Invalid SHA-256 hash length for subject '{}': expected 64, got {}",
                subject.name,
                subject.digest.sha256.len()
            ));
        }
    }

    VerificationResult::pass(MakotoLevel::L1)
        .with_message("Transform attestation structure is valid")
}

/// Verify a stream window attestation structure (L1 check).
pub fn verify_stream_window_structure(
    attestation: &StreamWindowAttestation,
) -> VerificationResult {
    if let Err(e) = attestation.validate() {
        return VerificationResult::fail(format!("Structure validation failed: {}", e));
    }

    // Check Merkle tree
    let merkle = &attestation.predicate.integrity.merkle_tree;
    if merkle.root.len() != 64 {
        return VerificationResult::fail(format!(
            "Invalid Merkle root length: expected 64, got {}",
            merkle.root.len()
        ));
    }

    if merkle.leaf_count == 0 {
        return VerificationResult::fail("Merkle tree has no leaves");
    }

    // Verify chain if present
    if let Some(chain) = &attestation.predicate.integrity.chain {
        if let (Some(prev_id), Some(prev_root)) =
            (&chain.previous_window_id, &chain.previous_merkle_root)
        {
            if prev_root.len() != 64 {
                return VerificationResult::fail(format!(
                    "Invalid previous Merkle root length: expected 64, got {}",
                    prev_root.len()
                ));
            }

            if prev_id.is_empty() {
                return VerificationResult::fail("Previous window ID is empty");
            }
        }
    }

    VerificationResult::pass(MakotoLevel::L1)
        .with_message("Stream window attestation structure is valid")
}

/// Verify a signed attestation (L2 check).
pub fn verify_signed_attestation<T>(
    signed: &SignedAttestation,
    verifier: &MakotoVerifier,
) -> VerificationResult
where
    T: serde::de::DeserializeOwned,
{
    // First verify the signature
    match signed.verify(verifier) {
        Ok(true) => {}
        Ok(false) => return VerificationResult::fail("Signature verification failed"),
        Err(e) => return VerificationResult::fail(format!("Signature error: {}", e)),
    }

    // Then decode and verify structure
    let _payload: T = match signed.decode_payload() {
        Ok(p) => p,
        Err(e) => return VerificationResult::fail(format!("Payload decode error: {}", e)),
    };

    VerificationResult::pass(MakotoLevel::L2)
        .with_message("Signed attestation is valid")
        .with_message(format!("Signature verified for key: {}", verifier.key_id()))
}

/// Detect attestation type from JSON.
pub fn detect_attestation_type(json: &str) -> Result<AttestationType> {
    let value: serde_json::Value = serde_json::from_str(json)?;

    // Check if it's a signed envelope
    if value.get("payloadType").is_some() && value.get("signatures").is_some() {
        return Ok(AttestationType::Signed);
    }

    // Check predicate type
    if let Some(pred_type) = value.get("predicateType").and_then(|v| v.as_str()) {
        match pred_type {
            ORIGIN_PREDICATE_TYPE => return Ok(AttestationType::Origin),
            TRANSFORM_PREDICATE_TYPE => return Ok(AttestationType::Transform),
            STREAM_WINDOW_PREDICATE_TYPE => return Ok(AttestationType::StreamWindow),
            _ => {}
        }
    }

    // Check for DBOM
    if value.get("dbomVersion").is_some() && value.get("dbomId").is_some() {
        return Ok(AttestationType::Dbom);
    }

    Err(MakotoError::InvalidAttestation(
        "Unknown attestation type".to_string(),
    ))
}

/// Types of Makoto attestations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttestationType {
    Origin,
    Transform,
    StreamWindow,
    Dbom,
    Signed,
}

impl std::fmt::Display for AttestationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AttestationType::Origin => write!(f, "origin"),
            AttestationType::Transform => write!(f, "transform"),
            AttestationType::StreamWindow => write!(f, "stream-window"),
            AttestationType::Dbom => write!(f, "dbom"),
            AttestationType::Signed => write!(f, "signed"),
        }
    }
}

/// Verify any attestation from JSON.
pub fn verify_attestation_json(json: &str) -> VerificationResult {
    let attestation_type = match detect_attestation_type(json) {
        Ok(t) => t,
        Err(e) => return VerificationResult::fail(format!("Type detection failed: {}", e)),
    };

    match attestation_type {
        AttestationType::Origin => {
            let attestation: OriginAttestation = match serde_json::from_str(json) {
                Ok(a) => a,
                Err(e) => return VerificationResult::fail(format!("Parse error: {}", e)),
            };
            verify_origin_structure(&attestation)
        }
        AttestationType::Transform => {
            let attestation: TransformAttestation = match serde_json::from_str(json) {
                Ok(a) => a,
                Err(e) => return VerificationResult::fail(format!("Parse error: {}", e)),
            };
            verify_transform_structure(&attestation)
        }
        AttestationType::StreamWindow => {
            let attestation: StreamWindowAttestation = match serde_json::from_str(json) {
                Ok(a) => a,
                Err(e) => return VerificationResult::fail(format!("Parse error: {}", e)),
            };
            verify_stream_window_structure(&attestation)
        }
        AttestationType::Signed => {
            VerificationResult::fail("Signed attestations require a verifier key")
        }
        AttestationType::Dbom => {
            // Basic DBOM validation
            let dbom: crate::types::Dbom = match serde_json::from_str(json) {
                Ok(d) => d,
                Err(e) => return VerificationResult::fail(format!("Parse error: {}", e)),
            };
            match dbom.validate() {
                Ok(()) => VerificationResult::pass(MakotoLevel::L1)
                    .with_message("DBOM structure is valid"),
                Err(e) => VerificationResult::fail(format!("DBOM validation failed: {}", e)),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::origin::{Collector, Origin};
    use crate::types::common::{CollectionMethod, SourceType};
    use crate::types::Subject;
    use chrono::Utc;

    #[test]
    fn test_verify_digest() {
        let data = b"hello world";
        let hash = sha256_hex(data);
        let digest = Digest::new(hash);

        assert!(verify_digest(&digest, data).unwrap());
    }

    #[test]
    fn test_verify_digest_mismatch() {
        let digest = Digest::new("a".repeat(64));
        let result = verify_digest(&digest, b"different data");
        assert!(result.is_err());
    }

    #[test]
    fn test_verify_origin_structure() {
        let origin = Origin::new(
            "https://api.example.com/data",
            SourceType::Api,
            CollectionMethod::Pull,
            Utc::now(),
        );

        let collector = Collector::new("https://example.com/collector/001");

        let attestation = OriginAttestation::builder()
            .subject(Subject::new("dataset:test", Digest::new("a".repeat(64))))
            .origin(origin)
            .collector(collector)
            .build()
            .unwrap();

        let result = verify_origin_structure(&attestation);
        assert!(result.valid);
        assert_eq!(result.level, Some(MakotoLevel::L1));
    }

    #[test]
    fn test_detect_attestation_type() {
        let origin_json = r#"{"_type":"https://in-toto.io/Statement/v1","predicateType":"https://makoto.dev/origin/v1","subject":[],"predicate":{}}"#;
        assert_eq!(
            detect_attestation_type(origin_json).unwrap(),
            AttestationType::Origin
        );

        let transform_json = r#"{"_type":"https://in-toto.io/Statement/v1","predicateType":"https://makoto.dev/transform/v1","subject":[],"predicate":{}}"#;
        assert_eq!(
            detect_attestation_type(transform_json).unwrap(),
            AttestationType::Transform
        );

        let dbom_json = r#"{"dbomVersion":"1.0.0","dbomId":"urn:dbom:test","dataset":{},"sources":[]}"#;
        assert_eq!(
            detect_attestation_type(dbom_json).unwrap(),
            AttestationType::Dbom
        );
    }
}
