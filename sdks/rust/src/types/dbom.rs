//! Data Bill of Materials (DBOM) types for comprehensive lineage documentation.

use super::common::*;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

/// A comprehensive manifest documenting dataset provenance and lineage.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dbom {
    /// Version of the DBOM specification.
    pub dbom_version: String,

    /// Unique identifier for this DBOM.
    pub dbom_id: String,

    /// Information about the final dataset.
    pub dataset: DatasetInfo,

    /// Source datasets that contribute to the final dataset.
    pub sources: Vec<Source>,

    /// Transformations applied to produce the final dataset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transformations: Option<Vec<Transformation>>,

    /// Visual representation of data lineage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lineage_graph: Option<LineageGraph>,

    /// Compliance and regulatory status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compliance: Option<Compliance>,

    /// Verification results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<DbomVerification>,

    /// DBOM document metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<DbomMetadata>,
}

impl Dbom {
    /// Create a new DBOM builder.
    pub fn builder() -> DbomBuilder {
        DbomBuilder::default()
    }

    /// Validate the DBOM structure.
    pub fn validate(&self) -> crate::error::Result<()> {
        if self.sources.is_empty() {
            return Err(crate::error::MakotoError::MissingField(
                "sources".to_string(),
            ));
        }

        // Validate DBOM ID format
        if !self.dbom_id.starts_with("urn:dbom:") {
            return Err(crate::error::MakotoError::InvalidAttestation(
                "DBOM ID must start with 'urn:dbom:'".to_string(),
            ));
        }

        Ok(())
    }
}

/// Builder for creating DBOMs.
#[derive(Debug, Default)]
pub struct DbomBuilder {
    dbom_version: Option<String>,
    dbom_id: Option<String>,
    dataset: Option<DatasetInfo>,
    sources: Vec<Source>,
    transformations: Vec<Transformation>,
    lineage_graph: Option<LineageGraph>,
    compliance: Option<Compliance>,
    verification: Option<DbomVerification>,
    metadata: Option<DbomMetadata>,
}

impl DbomBuilder {
    /// Set DBOM version (defaults to "1.0.0").
    pub fn version(mut self, version: impl Into<String>) -> Self {
        self.dbom_version = Some(version.into());
        self
    }

    /// Set DBOM ID.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.dbom_id = Some(id.into());
        self
    }

    /// Set dataset info.
    pub fn dataset(mut self, dataset: DatasetInfo) -> Self {
        self.dataset = Some(dataset);
        self
    }

    /// Add a source.
    pub fn source(mut self, source: Source) -> Self {
        self.sources.push(source);
        self
    }

    /// Add a transformation.
    pub fn transformation(mut self, transformation: Transformation) -> Self {
        self.transformations.push(transformation);
        self
    }

    /// Set lineage graph.
    pub fn lineage_graph(mut self, graph: LineageGraph) -> Self {
        self.lineage_graph = Some(graph);
        self
    }

    /// Set compliance info.
    pub fn compliance(mut self, compliance: Compliance) -> Self {
        self.compliance = Some(compliance);
        self
    }

    /// Set verification results.
    pub fn verification(mut self, verification: DbomVerification) -> Self {
        self.verification = Some(verification);
        self
    }

    /// Set metadata.
    pub fn metadata(mut self, metadata: DbomMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    /// Build the DBOM.
    pub fn build(self) -> crate::error::Result<Dbom> {
        let dataset = self
            .dataset
            .ok_or_else(|| crate::error::MakotoError::MissingField("dataset".to_string()))?;
        let dbom_id = self
            .dbom_id
            .ok_or_else(|| crate::error::MakotoError::MissingField("dbom_id".to_string()))?;

        if self.sources.is_empty() {
            return Err(crate::error::MakotoError::MissingField(
                "sources".to_string(),
            ));
        }

        Ok(Dbom {
            dbom_version: self.dbom_version.unwrap_or_else(|| "1.0.0".to_string()),
            dbom_id,
            dataset,
            sources: self.sources,
            transformations: if self.transformations.is_empty() {
                None
            } else {
                Some(self.transformations)
            },
            lineage_graph: self.lineage_graph,
            compliance: self.compliance,
            verification: self.verification,
            metadata: self.metadata,
        })
    }
}

/// Information about the final dataset.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatasetInfo {
    /// Human-readable name.
    pub name: String,

    /// Dataset version.
    pub version: String,

    /// Description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// When the dataset was created.
    pub created: DateTime<Utc>,

    /// Creator information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<Creator>,

    /// Cryptographic digest.
    pub digest: DbomDigest,

    /// Makoto level achieved.
    pub makoto_level: MakotoLevel,
}

impl DatasetInfo {
    /// Create new dataset info.
    pub fn new(
        name: impl Into<String>,
        version: impl Into<String>,
        created: DateTime<Utc>,
        digest: DbomDigest,
        makoto_level: MakotoLevel,
    ) -> Self {
        Self {
            name: name.into(),
            version: version.into(),
            description: None,
            created,
            creator: None,
            digest,
            makoto_level,
        }
    }

    /// Set description.
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Set creator.
    pub fn with_creator(mut self, creator: Creator) -> Self {
        self.creator = Some(creator);
        self
    }
}

/// Creator entity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Creator {
    /// Organization name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,

    /// Contact email or URI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<String>,
}

/// Digest for DBOM (slightly different from attestation digest).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DbomDigest {
    /// SHA-256 hash.
    pub sha256: String,

    /// SHA-512 hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha512: Option<String>,

    /// Record count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_count: Option<RecordCount>,

    /// File format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,

    /// Size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<u64>,
}

impl DbomDigest {
    /// Create a new digest.
    pub fn new(sha256: impl Into<String>) -> Self {
        Self {
            sha256: sha256.into(),
            sha512: None,
            record_count: None,
            format: None,
            size_bytes: None,
        }
    }
}

/// Record count (can be string or integer in JSON).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RecordCount {
    Integer(u64),
    String(String),
}

impl From<u64> for RecordCount {
    fn from(n: u64) -> Self {
        RecordCount::Integer(n)
    }
}

/// A source dataset.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    /// Source identifier.
    pub name: String,

    /// Description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// URI to attestation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attestation_ref: Option<String>,

    /// Attestation predicate type.
    pub attestation_type: String,

    /// Makoto level.
    pub makoto_level: MakotoLevel,

    /// Geographic region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geography: Option<String>,

    /// Consent information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent: Option<SourceConsent>,

    /// License information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<License>,

    /// Contribution info.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contribution: Option<Contribution>,
}

impl Source {
    /// Create a new source.
    pub fn new(
        name: impl Into<String>,
        attestation_type: impl Into<String>,
        makoto_level: MakotoLevel,
    ) -> Self {
        Self {
            name: name.into(),
            description: None,
            attestation_ref: None,
            attestation_type: attestation_type.into(),
            makoto_level,
            geography: None,
            consent: None,
            license: None,
            contribution: None,
        }
    }
}

/// Consent information for source.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SourceConsent {
    /// Consent type.
    #[serde(rename = "type")]
    pub consent_type: SourceConsentType,

    /// Reference URI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

/// Source consent types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SourceConsentType {
    Explicit,
    Contractual,
    LegitimateInterest,
    PublicTask,
}

/// License information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct License {
    /// License type.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub license_type: Option<String>,

    /// SPDX identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,

    /// Reference URI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

/// Contribution information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contribution {
    /// Records contributed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_count: Option<u64>,

    /// Percentage of final dataset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_percentage: Option<f64>,
}

/// A transformation step.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transformation {
    /// Sequence number.
    pub order: u32,

    /// Transformation name.
    pub name: String,

    /// Description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Attestation reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attestation_ref: Option<String>,

    /// Attestation type.
    pub attestation_type: String,

    /// Makoto level.
    pub makoto_level: MakotoLevel,

    /// Input dataset names.
    pub inputs: Vec<String>,

    /// Output dataset names.
    pub outputs: Vec<String>,

    /// Transform type URI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_type: Option<String>,
}

impl Transformation {
    /// Create a new transformation.
    pub fn new(
        order: u32,
        name: impl Into<String>,
        attestation_type: impl Into<String>,
        makoto_level: MakotoLevel,
        inputs: Vec<String>,
        outputs: Vec<String>,
    ) -> Self {
        Self {
            order,
            name: name.into(),
            description: None,
            attestation_ref: None,
            attestation_type: attestation_type.into(),
            makoto_level,
            inputs,
            outputs,
            transform_type: None,
        }
    }
}

/// Lineage graph representation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LineageGraph {
    /// Graph format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<LineageGraphFormat>,

    /// Graph content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,

    /// External URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Lineage graph formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LineageGraphFormat {
    GraphvizDot,
    Mermaid,
    JsonLd,
    Cytoscape,
}

/// Compliance information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Compliance {
    /// Overall Makoto level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overall_makoto_level: Option<MakotoLevel>,

    /// Justification for level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_justification: Option<String>,

    /// Privacy assessment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_assessment: Option<PrivacyAssessment>,

    /// Regulatory compliance statuses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regulatory_compliance: Option<Vec<RegulatoryStatus>>,

    /// D&TA compliance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dta_compliance: Option<DbomDtaCompliance>,
}

/// Privacy assessment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrivacyAssessment {
    /// PII removed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pii_removed: Option<bool>,

    /// Anonymization verified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anonymization_verified: Option<bool>,

    /// k-anonymity level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k_anonymity: Option<u32>,

    /// l-diversity level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub l_diversity: Option<u32>,

    /// t-closeness threshold.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t_closeness: Option<f64>,

    /// Differential privacy parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub differential_privacy: Option<DifferentialPrivacy>,
}

/// Differential privacy parameters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DifferentialPrivacy {
    /// Epsilon.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub epsilon: Option<f64>,

    /// Delta.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delta: Option<f64>,
}

/// Regulatory compliance status.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegulatoryStatus {
    /// Regulation name.
    pub regulation: String,

    /// Compliance status.
    pub status: ComplianceStatus,

    /// Notes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,

    /// Assessment date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_date: Option<NaiveDate>,

    /// Assessor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessor: Option<String>,
}

/// Compliance status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ComplianceStatus {
    Compliant,
    NonCompliant,
    Partial,
    NotApplicable,
    PendingReview,
}

/// D&TA compliance for DBOM.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DbomDtaCompliance {
    /// Standards version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standards_version: Option<String>,

    /// All required fields present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_fields_present: Option<bool>,
}

/// DBOM verification results.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DbomVerification {
    /// Chain verified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_verified: Option<bool>,

    /// All signatures valid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub all_signatures_valid: Option<bool>,

    /// Number of attestations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attestation_count: Option<u32>,

    /// When verification was performed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_timestamp: Option<DateTime<Utc>>,

    /// Verifier tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verifier: Option<VerifierInfo>,

    /// Verification errors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<VerificationError>>,
}

/// Verifier tool info.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VerifierInfo {
    /// Tool name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool: Option<String>,

    /// Tool version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// Verification error.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationError {
    /// Error code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    /// Error message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    /// Related attestation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attestation_ref: Option<String>,
}

/// DBOM document metadata.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DbomMetadata {
    /// Generator tool.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generator: Option<GeneratorInfo>,

    /// Creation timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<DateTime<Utc>>,

    /// Expiration timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_until: Option<DateTime<Utc>>,

    /// Access control.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_control: Option<AccessControl>,

    /// Tags.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

/// Generator tool info.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeneratorInfo {
    /// Tool name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool: Option<String>,

    /// Tool version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

/// Access control settings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessControl {
    /// Visibility level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<ConfidentialityClassification>,

    /// Allowed consumers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_consumers: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dbom_builder() {
        let dataset = DatasetInfo::new(
            "fraud-detection-training",
            "1.0.0",
            Utc::now(),
            DbomDigest::new("a".repeat(64)),
            MakotoLevel::L2,
        );

        let source = Source::new(
            "customer_transactions",
            "https://makoto.dev/origin/v1",
            MakotoLevel::L2,
        );

        let dbom = Dbom::builder()
            .id("urn:dbom:example.com:fraud-detection-v1")
            .dataset(dataset)
            .source(source)
            .build()
            .unwrap();

        assert_eq!(dbom.dbom_version, "1.0.0");
        assert!(dbom.validate().is_ok());
    }

    #[test]
    fn test_dbom_serialization() {
        let dataset = DatasetInfo::new(
            "test-dataset",
            "1.0.0",
            Utc::now(),
            DbomDigest::new("a".repeat(64)),
            MakotoLevel::L1,
        );

        let source = Source::new(
            "source_data",
            "https://makoto.dev/origin/v1",
            MakotoLevel::L1,
        );

        let dbom = Dbom::builder()
            .id("urn:dbom:test:dataset-v1")
            .dataset(dataset)
            .source(source)
            .build()
            .unwrap();

        let json = serde_json::to_string_pretty(&dbom).unwrap();
        let parsed: Dbom = serde_json::from_str(&json).unwrap();

        assert_eq!(dbom, parsed);
    }

    #[test]
    fn test_invalid_dbom_id() {
        let dataset = DatasetInfo::new(
            "test",
            "1.0.0",
            Utc::now(),
            DbomDigest::new("a".repeat(64)),
            MakotoLevel::L1,
        );

        let source = Source::new("src", "https://makoto.dev/origin/v1", MakotoLevel::L1);

        let dbom = Dbom::builder()
            .id("invalid-id") // Missing urn:dbom: prefix
            .dataset(dataset)
            .source(source)
            .build()
            .unwrap();

        assert!(dbom.validate().is_err());
    }
}
