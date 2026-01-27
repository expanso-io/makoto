//! Stream window predicate types for streaming data integrity attestations.

use super::common::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Predicate type URI for stream window attestations.
pub const STREAM_WINDOW_PREDICATE_TYPE: &str = "https://makoto.dev/stream-window/v1";

/// Stream window attestation (in-toto Statement with stream-window predicate).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamWindowAttestation {
    /// in-toto Statement type identifier.
    #[serde(rename = "_type")]
    pub statement_type: String,

    /// The window being attested.
    pub subject: Vec<Subject>,

    /// Predicate type identifier.
    pub predicate_type: String,

    /// The stream window predicate.
    pub predicate: StreamWindowPredicate,
}

impl StreamWindowAttestation {
    /// Create a new stream window attestation builder.
    pub fn builder() -> StreamWindowAttestationBuilder {
        StreamWindowAttestationBuilder::default()
    }

    /// Validate the attestation structure.
    pub fn validate(&self) -> crate::error::Result<()> {
        if self.statement_type != IN_TOTO_STATEMENT_TYPE {
            return Err(crate::error::MakotoError::InvalidAttestation(format!(
                "Invalid statement type: expected {}, got {}",
                IN_TOTO_STATEMENT_TYPE, self.statement_type
            )));
        }

        if self.predicate_type != STREAM_WINDOW_PREDICATE_TYPE {
            return Err(crate::error::MakotoError::InvalidPredicateType {
                expected: STREAM_WINDOW_PREDICATE_TYPE.to_string(),
                actual: self.predicate_type.clone(),
            });
        }

        if self.subject.is_empty() {
            return Err(crate::error::MakotoError::MissingField(
                "subject".to_string(),
            ));
        }

        Ok(())
    }
}

/// Builder for creating stream window attestations.
#[derive(Debug, Default)]
pub struct StreamWindowAttestationBuilder {
    subjects: Vec<Subject>,
    stream: Option<StreamDescriptor>,
    window: Option<WindowDescriptor>,
    integrity: Option<IntegrityDescriptor>,
    aggregates: Option<AggregatesDescriptor>,
    collector: Option<CollectorDescriptor>,
    metadata: Option<WindowMetadata>,
    verification: Option<WindowVerification>,
}

impl StreamWindowAttestationBuilder {
    /// Add a subject.
    pub fn subject(mut self, subject: Subject) -> Self {
        self.subjects.push(subject);
        self
    }

    /// Set stream descriptor.
    pub fn stream(mut self, stream: StreamDescriptor) -> Self {
        self.stream = Some(stream);
        self
    }

    /// Set window descriptor.
    pub fn window(mut self, window: WindowDescriptor) -> Self {
        self.window = Some(window);
        self
    }

    /// Set integrity descriptor.
    pub fn integrity(mut self, integrity: IntegrityDescriptor) -> Self {
        self.integrity = Some(integrity);
        self
    }

    /// Set aggregates.
    pub fn aggregates(mut self, aggregates: AggregatesDescriptor) -> Self {
        self.aggregates = Some(aggregates);
        self
    }

    /// Set collector.
    pub fn collector(mut self, collector: CollectorDescriptor) -> Self {
        self.collector = Some(collector);
        self
    }

    /// Set metadata.
    pub fn metadata(mut self, metadata: WindowMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    /// Set verification.
    pub fn verification(mut self, verification: WindowVerification) -> Self {
        self.verification = Some(verification);
        self
    }

    /// Build the attestation.
    pub fn build(self) -> crate::error::Result<StreamWindowAttestation> {
        let stream = self
            .stream
            .ok_or_else(|| crate::error::MakotoError::MissingField("stream".to_string()))?;
        let window = self
            .window
            .ok_or_else(|| crate::error::MakotoError::MissingField("window".to_string()))?;
        let integrity = self
            .integrity
            .ok_or_else(|| crate::error::MakotoError::MissingField("integrity".to_string()))?;

        if self.subjects.is_empty() {
            return Err(crate::error::MakotoError::MissingField(
                "subject".to_string(),
            ));
        }

        Ok(StreamWindowAttestation {
            statement_type: IN_TOTO_STATEMENT_TYPE.to_string(),
            subject: self.subjects,
            predicate_type: STREAM_WINDOW_PREDICATE_TYPE.to_string(),
            predicate: StreamWindowPredicate {
                stream,
                window,
                integrity,
                aggregates: self.aggregates,
                collector: self.collector,
                metadata: self.metadata,
                verification: self.verification,
            },
        })
    }
}

/// Stream window predicate for streaming data integrity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StreamWindowPredicate {
    /// Stream identification.
    pub stream: StreamDescriptor,

    /// Window parameters.
    pub window: WindowDescriptor,

    /// Cryptographic integrity.
    pub integrity: IntegrityDescriptor,

    /// Aggregate values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregates: Option<AggregatesDescriptor>,

    /// Collector information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collector: Option<CollectorDescriptor>,

    /// Window metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<WindowMetadata>,

    /// Verification info.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<WindowVerification>,
}

/// Identifies the data stream being attested.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StreamDescriptor {
    /// Unique identifier for the stream.
    pub id: String,

    /// URI of the stream source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,

    /// Topic pattern or name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,

    /// Partition identifiers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partitions: Option<Vec<String>>,
}

impl StreamDescriptor {
    /// Create a new stream descriptor.
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            source: None,
            topic: None,
            partitions: None,
        }
    }

    /// Set source URI.
    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }

    /// Set topic.
    pub fn with_topic(mut self, topic: impl Into<String>) -> Self {
        self.topic = Some(topic.into());
        self
    }

    /// Set partitions.
    pub fn with_partitions(mut self, partitions: Vec<String>) -> Self {
        self.partitions = Some(partitions);
        self
    }
}

/// Window parameters for stream aggregation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowDescriptor {
    /// Window type.
    #[serde(rename = "type")]
    pub window_type: WindowType,

    /// Window duration (ISO 8601).
    pub duration: String,

    /// Slide interval for sliding windows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slide: Option<String>,

    /// Time alignment strategy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alignment: Option<TimeAlignment>,

    /// Watermark timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub watermark: Option<DateTime<Utc>>,

    /// Maximum allowed lateness.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_lateness: Option<String>,
}

impl WindowDescriptor {
    /// Create a tumbling window.
    pub fn tumbling(duration: impl Into<String>) -> Self {
        Self {
            window_type: WindowType::Tumbling,
            duration: duration.into(),
            slide: None,
            alignment: None,
            watermark: None,
            allowed_lateness: None,
        }
    }

    /// Create a sliding window.
    pub fn sliding(duration: impl Into<String>, slide: impl Into<String>) -> Self {
        Self {
            window_type: WindowType::Sliding,
            duration: duration.into(),
            slide: Some(slide.into()),
            alignment: None,
            watermark: None,
            allowed_lateness: None,
        }
    }

    /// Create a session window.
    pub fn session(gap_duration: impl Into<String>) -> Self {
        Self {
            window_type: WindowType::Session,
            duration: gap_duration.into(),
            slide: None,
            alignment: None,
            watermark: None,
            allowed_lateness: None,
        }
    }

    /// Set alignment.
    pub fn with_alignment(mut self, alignment: TimeAlignment) -> Self {
        self.alignment = Some(alignment);
        self
    }

    /// Set watermark.
    pub fn with_watermark(mut self, watermark: DateTime<Utc>) -> Self {
        self.watermark = Some(watermark);
        self
    }

    /// Set allowed lateness.
    pub fn with_allowed_lateness(mut self, lateness: impl Into<String>) -> Self {
        self.allowed_lateness = Some(lateness.into());
        self
    }
}

/// Cryptographic integrity information for the window.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IntegrityDescriptor {
    /// Merkle tree parameters.
    pub merkle_tree: MerkleTreeDescriptor,

    /// Hash chain linking to previous windows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain: Option<ChainDescriptor>,
}

impl IntegrityDescriptor {
    /// Create a new integrity descriptor.
    pub fn new(merkle_tree: MerkleTreeDescriptor) -> Self {
        Self {
            merkle_tree,
            chain: None,
        }
    }

    /// Add chain descriptor.
    pub fn with_chain(mut self, chain: ChainDescriptor) -> Self {
        self.chain = Some(chain);
        self
    }
}

/// Merkle tree parameters for window records.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MerkleTreeDescriptor {
    /// Hash algorithm for internal nodes.
    pub algorithm: HashAlgorithm,

    /// Hash algorithm for leaf nodes (if different).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leaf_hash_algorithm: Option<HashAlgorithm>,

    /// Number of leaf nodes (records).
    pub leaf_count: u64,

    /// Height of the tree.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tree_height: Option<u32>,

    /// Root hash of the Merkle tree.
    pub root: String,
}

impl MerkleTreeDescriptor {
    /// Create a new Merkle tree descriptor.
    pub fn new(algorithm: HashAlgorithm, leaf_count: u64, root: impl Into<String>) -> Self {
        Self {
            algorithm,
            leaf_hash_algorithm: None,
            leaf_count,
            tree_height: None,
            root: root.into(),
        }
    }

    /// Set tree height.
    pub fn with_height(mut self, height: u32) -> Self {
        self.tree_height = Some(height);
        self
    }

    /// Set leaf hash algorithm.
    pub fn with_leaf_algorithm(mut self, algorithm: HashAlgorithm) -> Self {
        self.leaf_hash_algorithm = Some(algorithm);
        self
    }
}

/// Hash chain linking windows for tamper-evident sequencing.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChainDescriptor {
    /// ID of the previous window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_window_id: Option<String>,

    /// Merkle root of the previous window.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_merkle_root: Option<String>,

    /// Position in the chain (1 = genesis).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_length: Option<u64>,

    /// ID of the first window in the chain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genesis_window_id: Option<String>,
}

impl ChainDescriptor {
    /// Create genesis (first) chain descriptor.
    pub fn genesis(genesis_window_id: impl Into<String>) -> Self {
        Self {
            previous_window_id: None,
            previous_merkle_root: None,
            chain_length: Some(1),
            genesis_window_id: Some(genesis_window_id.into()),
        }
    }

    /// Create a linked chain descriptor.
    pub fn linked(
        previous_window_id: impl Into<String>,
        previous_merkle_root: impl Into<String>,
        chain_length: u64,
    ) -> Self {
        Self {
            previous_window_id: Some(previous_window_id.into()),
            previous_merkle_root: Some(previous_merkle_root.into()),
            chain_length: Some(chain_length),
            genesis_window_id: None,
        }
    }
}

/// Aggregate values for quick verification.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AggregatesDescriptor {
    /// Simple checksum.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,

    /// Statistical aggregates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statistics: Option<WindowStatistics>,
}

/// Statistical aggregates over window data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowStatistics {
    /// Earliest record timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_timestamp: Option<DateTime<Utc>>,

    /// Latest record timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_timestamp: Option<DateTime<Utc>>,

    /// Average interval between records (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_interval_ms: Option<f64>,

    /// Additional statistics.
    #[serde(flatten)]
    pub additional: HashMap<String, serde_json::Value>,
}

/// Collector information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CollectorDescriptor {
    /// Unique identifier URI.
    pub id: String,

    /// Version information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<HashMap<String, String>>,

    /// Location identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

impl CollectorDescriptor {
    /// Create a new collector descriptor.
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            version: None,
            location: None,
        }
    }
}

/// Operational metadata about window processing.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowMetadata {
    /// Processing latency (ISO 8601 duration).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_latency: Option<String>,

    /// Late records included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub late_records: Option<u64>,

    /// Dropped records.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropped_records: Option<u64>,

    /// Backpressure events.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backpressure_events: Option<u64>,
}

impl Default for WindowMetadata {
    fn default() -> Self {
        Self {
            processing_latency: None,
            late_records: None,
            dropped_records: None,
            backpressure_events: None,
        }
    }
}

/// Verification information for individual records.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowVerification {
    /// Whether Merkle proofs are available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merkle_proof_available: Option<bool>,

    /// Endpoint for retrieving proofs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proof_endpoint: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stream_window_builder() {
        let stream = StreamDescriptor::new("iot_sensors")
            .with_source("mqtt://broker.example.com")
            .with_topic("sensors/#");

        let window = WindowDescriptor::tumbling("PT1M");

        let merkle = MerkleTreeDescriptor::new(HashAlgorithm::Sha256, 1000, "a".repeat(64));

        let integrity = IntegrityDescriptor::new(merkle);

        let attestation = StreamWindowAttestation::builder()
            .subject(Subject::new("stream:iot_sensors:window_20251220_100000", Digest::new("b".repeat(64))))
            .stream(stream)
            .window(window)
            .integrity(integrity)
            .build()
            .unwrap();

        assert_eq!(attestation.statement_type, IN_TOTO_STATEMENT_TYPE);
        assert_eq!(attestation.predicate_type, STREAM_WINDOW_PREDICATE_TYPE);
        assert!(attestation.validate().is_ok());
    }

    #[test]
    fn test_window_types() {
        let tumbling = WindowDescriptor::tumbling("PT1M");
        assert_eq!(tumbling.window_type, WindowType::Tumbling);
        assert!(tumbling.slide.is_none());

        let sliding = WindowDescriptor::sliding("PT5M", "PT1M");
        assert_eq!(sliding.window_type, WindowType::Sliding);
        assert_eq!(sliding.slide, Some("PT1M".to_string()));

        let session = WindowDescriptor::session("PT30S");
        assert_eq!(session.window_type, WindowType::Session);
    }

    #[test]
    fn test_chain_descriptor() {
        let genesis = ChainDescriptor::genesis("stream:test:window_001");
        assert_eq!(genesis.chain_length, Some(1));
        assert!(genesis.previous_window_id.is_none());

        let linked = ChainDescriptor::linked(
            "stream:test:window_001",
            "a".repeat(64),
            2,
        );
        assert_eq!(linked.chain_length, Some(2));
        assert!(linked.previous_window_id.is_some());
    }
}
