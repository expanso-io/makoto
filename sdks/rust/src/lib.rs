//! # Makoto Rust SDK
//!
//! Rust SDK for creating, signing, and verifying Makoto data provenance attestations.
//!
//! ## Overview
//!
//! Makoto is a data provenance framework that provides verifiable attestations for
//! data lineage, similar to how SLSA provides supply chain security for software.
//!
//! This SDK supports:
//! - **Origin attestations** - Document where data came from
//! - **Transform attestations** - Document data transformations
//! - **Stream window attestations** - Document streaming data integrity with Merkle trees
//! - **DBOM (Data Bill of Materials)** - Comprehensive lineage manifests
//! - **Signing (L2)** - ECDSA P-256 signatures for tamper-evidence
//! - **Verification** - Structure and signature verification
//!
//! ## Makoto Levels
//!
//! - **L1**: Attestation exists (provenance available)
//! - **L2**: Signed attestation (tamper-evident)
//! - **L3**: Hardened signing (non-falsifiable, requires HSM)
//!
//! ## Quick Start
//!
//! ### Create an Origin Attestation (L1)
//!
//! ```rust
//! use makoto::types::{
//!     origin::{Origin, Collector, OriginAttestation},
//!     common::{SourceType, CollectionMethod},
//!     Subject, Digest,
//! };
//! use chrono::Utc;
//!
//! let origin = Origin::new(
//!     "https://api.example.com/data",
//!     SourceType::Api,
//!     CollectionMethod::Pull,
//!     Utc::now(),
//! );
//!
//! let collector = Collector::new("https://example.com/collector/001");
//!
//! let attestation = OriginAttestation::builder()
//!     .subject(Subject::new(
//!         "dataset:customer_transactions",
//!         Digest::new("a".repeat(64)),
//!     ))
//!     .origin(origin)
//!     .collector(collector)
//!     .build()
//!     .expect("Failed to build attestation");
//!
//! // Serialize to JSON
//! let json = serde_json::to_string_pretty(&attestation).unwrap();
//! ```
//!
//! ### Sign an Attestation (L2)
//!
//! ```rust
//! use makoto::signing::{MakotoSigner, SignedAttestation};
//! use makoto::types::{
//!     origin::{Origin, Collector, OriginAttestation},
//!     common::{SourceType, CollectionMethod},
//!     Subject, Digest,
//! };
//! use chrono::Utc;
//!
//! // Create signer (in production, load from secure storage)
//! let signer = MakotoSigner::generate();
//!
//! // Create attestation
//! let origin = Origin::new(
//!     "https://api.example.com/data",
//!     SourceType::Api,
//!     CollectionMethod::Pull,
//!     Utc::now(),
//! );
//! let collector = Collector::new("https://example.com/collector/001");
//! let attestation = OriginAttestation::builder()
//!     .subject(Subject::new("dataset:test", Digest::new("a".repeat(64))))
//!     .origin(origin)
//!     .collector(collector)
//!     .build()
//!     .unwrap();
//!
//! // Sign it
//! let signed = SignedAttestation::sign(&attestation, &signer).unwrap();
//!
//! // Verify
//! let verifier = signer.verifying_key();
//! assert!(signed.verify(&verifier).unwrap());
//! ```
//!
//! ### Verify an Attestation
//!
//! ```rust
//! use makoto::verification::{verify_attestation_json, verify_origin_structure};
//!
//! // Verify from JSON
//! let json = r#"{"_type":"https://in-toto.io/Statement/v1",...}"#;
//! // let result = verify_attestation_json(json);
//! // if result.valid {
//! //     println!("Attestation is valid at level {:?}", result.level);
//! // }
//! ```
//!
//! ### Build Merkle Trees for Streaming Data
//!
//! ```rust
//! use makoto::hash::MerkleTree;
//!
//! let records: Vec<&[u8]> = vec![b"record1", b"record2", b"record3", b"record4"];
//! let tree = MerkleTree::from_leaves(&records);
//!
//! // Get root hash for attestation
//! let root = tree.root_hex().unwrap();
//!
//! // Generate proof for a specific record
//! let proof = tree.proof(2).unwrap();
//! assert!(tree.verify_proof(&proof));
//! ```

pub mod error;
pub mod hash;
pub mod signing;
pub mod types;
pub mod verification;

// Re-export commonly used items at crate root
pub use error::{MakotoError, Result};
pub use hash::{sha256_hex, sha256_str, MerkleTree, MerkleProof};
pub use signing::{MakotoSigner, MakotoVerifier, SignedAttestation};
pub use types::{
    Dbom, Digest, MakotoLevel, OriginAttestation, StreamWindowAttestation, Subject,
    TransformAttestation,
};
pub use verification::{
    verify_attestation_json, verify_digest, verify_origin_structure,
    verify_stream_window_structure, verify_transform_structure, AttestationType,
    VerificationResult,
};

/// SDK version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// in-toto Statement type constant.
pub const IN_TOTO_STATEMENT_TYPE: &str = "https://in-toto.io/Statement/v1";

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::common::{CollectionMethod, SourceType};
    use crate::types::origin::{Collector, Origin};
    use chrono::Utc;

    #[test]
    fn test_full_workflow() {
        // 1. Create attestation
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

        // 2. Verify structure (L1)
        let result = verify_origin_structure(&attestation);
        assert!(result.valid);
        assert_eq!(result.level, Some(MakotoLevel::L1));

        // 3. Sign (L2)
        let signer = MakotoSigner::generate();
        let signed = SignedAttestation::sign(&attestation, &signer).unwrap();

        // 4. Verify signature
        let verifier = signer.verifying_key();
        assert!(signed.verify(&verifier).unwrap());

        // 5. Decode and verify
        let decoded: OriginAttestation = signed.decode_payload().unwrap();
        assert_eq!(decoded.predicate.origin.source, attestation.predicate.origin.source);
    }

    #[test]
    fn test_merkle_tree_workflow() {
        // Create records
        let records: Vec<&[u8]> = vec![b"record1", b"record2", b"record3", b"record4"];

        // Build tree
        let tree = MerkleTree::from_leaves(&records);

        // Get root for attestation
        let root = tree.root_hex().unwrap();
        assert_eq!(root.len(), 64);

        // Generate and verify proofs
        for i in 0..records.len() {
            let proof = tree.proof(i).unwrap();
            assert!(tree.verify_proof(&proof));
        }
    }
}
