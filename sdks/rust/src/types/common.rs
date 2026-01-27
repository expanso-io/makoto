//! Common types shared across Makoto attestation types.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The in-toto Statement v1 type identifier.
pub const IN_TOTO_STATEMENT_TYPE: &str = "https://in-toto.io/Statement/v1";

/// Makoto attestation levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MakotoLevel {
    /// Level 1: Attestation exists (provenance available).
    L1,
    /// Level 2: Signed attestation (tamper-evident).
    L2,
    /// Level 3: Hardened signing (non-falsifiable).
    L3,
}

impl std::fmt::Display for MakotoLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MakotoLevel::L1 => write!(f, "L1"),
            MakotoLevel::L2 => write!(f, "L2"),
            MakotoLevel::L3 => write!(f, "L3"),
        }
    }
}

/// Cryptographic digest of data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Digest {
    /// SHA-256 hash (required).
    pub sha256: String,

    /// SHA-512 hash (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha512: Option<String>,

    /// Number of records (as string for large numbers).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_count: Option<String>,

    /// Merkle tree root hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merkle_root: Option<String>,

    /// Additional digest algorithms.
    #[serde(flatten)]
    pub additional: HashMap<String, String>,
}

impl Digest {
    /// Create a new digest with only SHA-256.
    pub fn new(sha256: impl Into<String>) -> Self {
        Self {
            sha256: sha256.into(),
            sha512: None,
            record_count: None,
            merkle_root: None,
            additional: HashMap::new(),
        }
    }

    /// Add SHA-512 hash.
    pub fn with_sha512(mut self, sha512: impl Into<String>) -> Self {
        self.sha512 = Some(sha512.into());
        self
    }

    /// Add record count.
    pub fn with_record_count(mut self, count: u64) -> Self {
        self.record_count = Some(count.to_string());
        self
    }

    /// Add Merkle root.
    pub fn with_merkle_root(mut self, root: impl Into<String>) -> Self {
        self.merkle_root = Some(root.into());
        self
    }
}

/// Subject of an attestation (the artifact being attested).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Subject {
    /// Identifier for the dataset.
    pub name: String,

    /// Cryptographic digests.
    pub digest: Digest,
}

impl Subject {
    /// Create a new subject.
    pub fn new(name: impl Into<String>, digest: Digest) -> Self {
        Self {
            name: name.into(),
            digest,
        }
    }
}

/// Source type for data origin.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SourceType {
    Api,
    Database,
    File,
    Stream,
    Manual,
    Sensor,
    Other,
}

/// Collection method for data origin.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CollectionMethod {
    Pull,
    Push,
    ScheduledPull,
    EventDriven,
    BatchUpload,
    Streaming,
    Manual,
}

/// Consent type (aligned with GDPR Article 6).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ConsentType {
    Contractual,
    Consent,
    LegitimateInterest,
    LegalObligation,
    PublicInterest,
    VitalInterest,
}

/// Consent information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Consent {
    /// Type of consent/legal basis.
    #[serde(rename = "type")]
    pub consent_type: ConsentType,

    /// URI to consent documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,

    /// When consent was obtained.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obtained: Option<chrono::DateTime<chrono::Utc>>,

    /// When consent expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<chrono::DateTime<chrono::Utc>>,
}

/// Deployment environment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Production,
    Staging,
    Development,
    Test,
}

/// Isolation level for execution environments.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum IsolationLevel {
    None,
    Process,
    Container,
    Vm,
    Hardware,
}

/// Confidentiality classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConfidentialityClassification {
    Public,
    Internal,
    Confidential,
    Restricted,
}

/// Hash algorithm identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HashAlgorithm {
    Sha256,
    Sha384,
    Sha512,
    #[serde(rename = "sha3-256")]
    Sha3_256,
    #[serde(rename = "sha3-384")]
    Sha3_384,
    #[serde(rename = "sha3-512")]
    Sha3_512,
    Blake2b,
    Blake3,
}

impl Default for HashAlgorithm {
    fn default() -> Self {
        Self::Sha256
    }
}

/// Window type for streaming attestations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WindowType {
    Tumbling,
    Sliding,
    Session,
}

/// Time alignment strategy for windows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TimeAlignment {
    WallClock,
    EventTime,
}

impl Default for TimeAlignment {
    fn default() -> Self {
        Self::EventTime
    }
}
