//! Origin attestation types for documenting data provenance at collection.

use super::common::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Predicate type URI for origin attestations.
pub const ORIGIN_PREDICATE_TYPE: &str = "https://makoto.dev/origin/v1";

/// Origin attestation (in-toto Statement with origin predicate).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OriginAttestation {
    /// in-toto Statement type identifier.
    #[serde(rename = "_type")]
    pub statement_type: String,

    /// The dataset(s) this attestation describes.
    pub subject: Vec<Subject>,

    /// Predicate type identifier.
    pub predicate_type: String,

    /// The origin predicate.
    pub predicate: OriginPredicate,
}

impl OriginAttestation {
    /// Create a new origin attestation builder.
    pub fn builder() -> OriginAttestationBuilder {
        OriginAttestationBuilder::default()
    }

    /// Validate the attestation structure.
    pub fn validate(&self) -> crate::error::Result<()> {
        if self.statement_type != IN_TOTO_STATEMENT_TYPE {
            return Err(crate::error::MakotoError::InvalidAttestation(format!(
                "Invalid statement type: expected {}, got {}",
                IN_TOTO_STATEMENT_TYPE, self.statement_type
            )));
        }

        if self.predicate_type != ORIGIN_PREDICATE_TYPE {
            return Err(crate::error::MakotoError::InvalidPredicateType {
                expected: ORIGIN_PREDICATE_TYPE.to_string(),
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

/// Builder for creating origin attestations.
#[derive(Debug, Default)]
pub struct OriginAttestationBuilder {
    subjects: Vec<Subject>,
    origin: Option<Origin>,
    collector: Option<Collector>,
    schema: Option<DataSchema>,
    metadata: Option<CollectionMetadata>,
    dta_compliance: Option<DtaCompliance>,
}

impl OriginAttestationBuilder {
    /// Add a subject to the attestation.
    pub fn subject(mut self, subject: Subject) -> Self {
        self.subjects.push(subject);
        self
    }

    /// Set the origin information.
    pub fn origin(mut self, origin: Origin) -> Self {
        self.origin = Some(origin);
        self
    }

    /// Set the collector information.
    pub fn collector(mut self, collector: Collector) -> Self {
        self.collector = Some(collector);
        self
    }

    /// Set the schema information.
    pub fn schema(mut self, schema: DataSchema) -> Self {
        self.schema = Some(schema);
        self
    }

    /// Set the collection metadata.
    pub fn metadata(mut self, metadata: CollectionMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    /// Set D&TA compliance information.
    pub fn dta_compliance(mut self, dta: DtaCompliance) -> Self {
        self.dta_compliance = Some(dta);
        self
    }

    /// Build the attestation.
    pub fn build(self) -> crate::error::Result<OriginAttestation> {
        let origin = self
            .origin
            .ok_or_else(|| crate::error::MakotoError::MissingField("origin".to_string()))?;
        let collector = self
            .collector
            .ok_or_else(|| crate::error::MakotoError::MissingField("collector".to_string()))?;

        if self.subjects.is_empty() {
            return Err(crate::error::MakotoError::MissingField(
                "subject".to_string(),
            ));
        }

        Ok(OriginAttestation {
            statement_type: IN_TOTO_STATEMENT_TYPE.to_string(),
            subject: self.subjects,
            predicate_type: ORIGIN_PREDICATE_TYPE.to_string(),
            predicate: OriginPredicate {
                origin,
                collector,
                schema: self.schema,
                metadata: self.metadata,
                dta_compliance: self.dta_compliance,
            },
        })
    }
}

/// Origin predicate containing provenance information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OriginPredicate {
    /// Information about where and how data was collected.
    pub origin: Origin,

    /// Information about the collector system.
    pub collector: Collector,

    /// Schema information for the collected data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<DataSchema>,

    /// Collection statistics and metrics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<CollectionMetadata>,

    /// D&TA Data Provenance Standards compliance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dta_compliance: Option<DtaCompliance>,
}

/// Information about where and how data was collected.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Origin {
    /// URI identifying the data source.
    pub source: String,

    /// Category of the data source.
    pub source_type: SourceType,

    /// How the data was collected.
    pub collection_method: CollectionMethod,

    /// When data collection occurred.
    pub collection_timestamp: DateTime<Utc>,

    /// Geographic region where data was collected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub geography: Option<String>,

    /// Consent and legal basis for collection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent: Option<Consent>,
}

impl Origin {
    /// Create a new origin.
    pub fn new(
        source: impl Into<String>,
        source_type: SourceType,
        collection_method: CollectionMethod,
        collection_timestamp: DateTime<Utc>,
    ) -> Self {
        Self {
            source: source.into(),
            source_type,
            collection_method,
            collection_timestamp,
            geography: None,
            consent: None,
        }
    }

    /// Set the geography.
    pub fn with_geography(mut self, geography: impl Into<String>) -> Self {
        self.geography = Some(geography.into());
        self
    }

    /// Set consent information.
    pub fn with_consent(mut self, consent: Consent) -> Self {
        self.consent = Some(consent);
        self
    }
}

/// Information about the system that collected the data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Collector {
    /// Unique identifier for the collector instance.
    pub id: String,

    /// Version information for collector components.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<HashMap<String, String>>,

    /// Deployment environment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<Environment>,

    /// Platform or runtime environment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
}

impl Collector {
    /// Create a new collector.
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            version: None,
            environment: None,
            platform: None,
        }
    }

    /// Set version information.
    pub fn with_version(mut self, version: HashMap<String, String>) -> Self {
        self.version = Some(version);
        self
    }

    /// Set environment.
    pub fn with_environment(mut self, env: Environment) -> Self {
        self.environment = Some(env);
        self
    }

    /// Set platform.
    pub fn with_platform(mut self, platform: impl Into<String>) -> Self {
        self.platform = Some(platform.into());
        self
    }
}

/// Schema information for the collected data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataSchema {
    /// Data format (e.g., json-lines, csv, parquet).
    pub format: String,

    /// URI to the schema definition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_ref: Option<String>,

    /// Digest of the schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_digest: Option<HashMap<String, String>>,

    /// Version of the schema.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<String>,
}

impl DataSchema {
    /// Create a new schema.
    pub fn new(format: impl Into<String>) -> Self {
        Self {
            format: format.into(),
            schema_ref: None,
            schema_digest: None,
            schema_version: None,
        }
    }
}

/// Collection statistics and metrics.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionMetadata {
    /// ISO 8601 duration of the collection process.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_duration: Option<String>,

    /// Total bytes of data collected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_collected: Option<u64>,

    /// Number of records collected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records_collected: Option<u64>,

    /// Number of records dropped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records_dropped: Option<u64>,

    /// Fraction of records with errors (0.0 to 1.0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_rate: Option<f64>,

    /// When collection started.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<DateTime<Utc>>,

    /// When collection completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<DateTime<Utc>>,
}

impl Default for CollectionMetadata {
    fn default() -> Self {
        Self {
            collection_duration: None,
            bytes_collected: None,
            records_collected: None,
            records_dropped: None,
            error_rate: None,
            start_time: None,
            end_time: None,
        }
    }
}

/// D&TA Data Provenance Standards compliance.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DtaCompliance {
    /// Version of D&TA standards.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub standards_version: Option<String>,

    /// D&TA Source Standard fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_standard: Option<DtaSourceStandard>,

    /// D&TA Provenance Standard fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provenance_standard: Option<DtaProvenanceStandard>,

    /// D&TA Use Standard fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_standard: Option<DtaUseStandard>,
}

/// D&TA Source Standard fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DtaSourceStandard {
    /// Human-readable title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_title: Option<String>,

    /// Organization that issued the data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_issuer: Option<String>,

    /// Description of the dataset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// D&TA Provenance Standard fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DtaProvenanceStandard {
    /// Geographic origin.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_origin_geography: Option<String>,

    /// Collection method description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,

    /// Data format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_format: Option<String>,
}

/// D&TA Use Standard fields.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DtaUseStandard {
    /// Confidentiality level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidentiality_classification: Option<ConfidentialityClassification>,

    /// Intended use for the data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intended_data_use: Option<String>,

    /// License or usage terms.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_origin_attestation_builder() {
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

        assert_eq!(attestation.statement_type, IN_TOTO_STATEMENT_TYPE);
        assert_eq!(attestation.predicate_type, ORIGIN_PREDICATE_TYPE);
        assert!(attestation.validate().is_ok());
    }

    #[test]
    fn test_origin_serialization() {
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

        let json = serde_json::to_string_pretty(&attestation).unwrap();
        let parsed: OriginAttestation = serde_json::from_str(&json).unwrap();

        assert_eq!(attestation, parsed);
    }
}
