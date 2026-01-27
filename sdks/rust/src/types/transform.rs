//! Transform attestation types for documenting data transformations.

use super::common::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Predicate type URI for transform attestations.
pub const TRANSFORM_PREDICATE_TYPE: &str = "https://makoto.dev/transform/v1";

/// Transform attestation (in-toto Statement with transform predicate).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransformAttestation {
    /// in-toto Statement type identifier.
    #[serde(rename = "_type")]
    pub statement_type: String,

    /// The output data artifacts produced by this transformation.
    pub subject: Vec<Subject>,

    /// Predicate type identifier.
    pub predicate_type: String,

    /// The transform predicate.
    pub predicate: TransformPredicate,
}

impl TransformAttestation {
    /// Create a new transform attestation builder.
    pub fn builder() -> TransformAttestationBuilder {
        TransformAttestationBuilder::default()
    }

    /// Validate the attestation structure.
    pub fn validate(&self) -> crate::error::Result<()> {
        if self.statement_type != IN_TOTO_STATEMENT_TYPE {
            return Err(crate::error::MakotoError::InvalidAttestation(format!(
                "Invalid statement type: expected {}, got {}",
                IN_TOTO_STATEMENT_TYPE, self.statement_type
            )));
        }

        if self.predicate_type != TRANSFORM_PREDICATE_TYPE {
            return Err(crate::error::MakotoError::InvalidPredicateType {
                expected: TRANSFORM_PREDICATE_TYPE.to_string(),
                actual: self.predicate_type.clone(),
            });
        }

        if self.subject.is_empty() {
            return Err(crate::error::MakotoError::MissingField(
                "subject".to_string(),
            ));
        }

        if self.predicate.inputs.is_empty() {
            return Err(crate::error::MakotoError::MissingField(
                "inputs".to_string(),
            ));
        }

        Ok(())
    }
}

/// Builder for creating transform attestations.
#[derive(Debug, Default)]
pub struct TransformAttestationBuilder {
    subjects: Vec<Subject>,
    inputs: Vec<InputReference>,
    transform: Option<TransformDefinition>,
    executor: Option<Executor>,
    metadata: Option<ExecutionMetadata>,
    verification: Option<VerificationInfo>,
}

impl TransformAttestationBuilder {
    /// Add an output subject.
    pub fn subject(mut self, subject: Subject) -> Self {
        self.subjects.push(subject);
        self
    }

    /// Add an input reference.
    pub fn input(mut self, input: InputReference) -> Self {
        self.inputs.push(input);
        self
    }

    /// Set the transform definition.
    pub fn transform(mut self, transform: TransformDefinition) -> Self {
        self.transform = Some(transform);
        self
    }

    /// Set the executor.
    pub fn executor(mut self, executor: Executor) -> Self {
        self.executor = Some(executor);
        self
    }

    /// Set execution metadata.
    pub fn metadata(mut self, metadata: ExecutionMetadata) -> Self {
        self.metadata = Some(metadata);
        self
    }

    /// Set verification info.
    pub fn verification(mut self, verification: VerificationInfo) -> Self {
        self.verification = Some(verification);
        self
    }

    /// Build the attestation.
    pub fn build(self) -> crate::error::Result<TransformAttestation> {
        let transform = self
            .transform
            .ok_or_else(|| crate::error::MakotoError::MissingField("transform".to_string()))?;
        let executor = self
            .executor
            .ok_or_else(|| crate::error::MakotoError::MissingField("executor".to_string()))?;

        if self.subjects.is_empty() {
            return Err(crate::error::MakotoError::MissingField(
                "subject".to_string(),
            ));
        }

        if self.inputs.is_empty() {
            return Err(crate::error::MakotoError::MissingField(
                "inputs".to_string(),
            ));
        }

        Ok(TransformAttestation {
            statement_type: IN_TOTO_STATEMENT_TYPE.to_string(),
            subject: self.subjects,
            predicate_type: TRANSFORM_PREDICATE_TYPE.to_string(),
            predicate: TransformPredicate {
                inputs: self.inputs,
                transform,
                executor,
                metadata: self.metadata,
                verification: self.verification,
            },
        })
    }
}

/// Transform predicate describing what transformation was performed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransformPredicate {
    /// Input data artifacts that were transformed.
    pub inputs: Vec<InputReference>,

    /// Definition of the transformation.
    pub transform: TransformDefinition,

    /// Information about the execution environment.
    pub executor: Executor,

    /// Execution metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ExecutionMetadata>,

    /// Verification information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<VerificationInfo>,
}

/// Reference to an input data artifact.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InputReference {
    /// Identifier for the input dataset.
    pub name: String,

    /// Cryptographic digest of the input.
    pub digest: Digest,

    /// URI to the attestation for this input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attestation_ref: Option<String>,

    /// Makoto level of the input attestation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub makoto_level: Option<MakotoLevel>,
}

impl InputReference {
    /// Create a new input reference.
    pub fn new(name: impl Into<String>, digest: Digest) -> Self {
        Self {
            name: name.into(),
            digest,
            attestation_ref: None,
            makoto_level: None,
        }
    }

    /// Set the attestation reference.
    pub fn with_attestation_ref(mut self, uri: impl Into<String>) -> Self {
        self.attestation_ref = Some(uri.into());
        self
    }

    /// Set the Makoto level.
    pub fn with_makoto_level(mut self, level: MakotoLevel) -> Self {
        self.makoto_level = Some(level);
        self
    }
}

/// Definition of the transformation that was applied.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransformDefinition {
    /// URI identifying the transform type.
    #[serde(rename = "type")]
    pub transform_type: String,

    /// Human-readable name.
    pub name: String,

    /// Version of the transformation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,

    /// Human-readable description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Configuration parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<HashMap<String, serde_json::Value>>,

    /// Reference to the transformation code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_ref: Option<CodeReference>,
}

impl TransformDefinition {
    /// Create a new transform definition.
    pub fn new(transform_type: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            transform_type: transform_type.into(),
            name: name.into(),
            version: None,
            description: None,
            parameters: None,
            code_ref: None,
        }
    }

    /// Set version.
    pub fn with_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    /// Set description.
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = Some(desc.into());
        self
    }

    /// Set parameters.
    pub fn with_parameters(mut self, params: HashMap<String, serde_json::Value>) -> Self {
        self.parameters = Some(params);
        self
    }

    /// Set code reference.
    pub fn with_code_ref(mut self, code_ref: CodeReference) -> Self {
        self.code_ref = Some(code_ref);
        self
    }
}

/// Reference to the transformation code/configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeReference {
    /// URI to the code repository.
    pub uri: String,

    /// Git commit hash or version identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit: Option<String>,

    /// Path to specific file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// Digest of the code file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<HashMap<String, String>>,
}

impl CodeReference {
    /// Create a new code reference.
    pub fn new(uri: impl Into<String>) -> Self {
        Self {
            uri: uri.into(),
            commit: None,
            path: None,
            digest: None,
        }
    }

    /// Set commit hash.
    pub fn with_commit(mut self, commit: impl Into<String>) -> Self {
        self.commit = Some(commit.into());
        self
    }

    /// Set path.
    pub fn with_path(mut self, path: impl Into<String>) -> Self {
        self.path = Some(path.into());
        self
    }
}

/// Information about the system that executed the transformation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Executor {
    /// Unique identifier for the execution environment.
    pub id: String,

    /// Platform name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,

    /// Version information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<HashMap<String, String>>,

    /// Execution environment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<String>,

    /// Isolation level.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isolation: Option<IsolationLevel>,
}

impl Executor {
    /// Create a new executor.
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            platform: None,
            version: None,
            environment: None,
            isolation: None,
        }
    }

    /// Set platform.
    pub fn with_platform(mut self, platform: impl Into<String>) -> Self {
        self.platform = Some(platform.into());
        self
    }

    /// Set environment.
    pub fn with_environment(mut self, env: impl Into<String>) -> Self {
        self.environment = Some(env.into());
        self
    }

    /// Set isolation level.
    pub fn with_isolation(mut self, isolation: IsolationLevel) -> Self {
        self.isolation = Some(isolation);
        self
    }
}

/// Metadata about the transformation execution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionMetadata {
    /// Unique identifier for this execution.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invocation_id: Option<String>,

    /// When execution started.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_on: Option<DateTime<Utc>>,

    /// When execution completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_on: Option<DateTime<Utc>>,

    /// Duration in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<f64>,

    /// Records read from inputs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records_input: Option<u64>,

    /// Records written to output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records_output: Option<u64>,

    /// Records dropped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records_dropped: Option<u64>,

    /// Records modified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records_modified: Option<u64>,

    /// Bytes read.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_input: Option<u64>,

    /// Bytes written.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes_output: Option<u64>,
}

impl Default for ExecutionMetadata {
    fn default() -> Self {
        Self {
            invocation_id: None,
            started_on: None,
            finished_on: None,
            duration_seconds: None,
            records_input: None,
            records_output: None,
            records_dropped: None,
            records_modified: None,
            bytes_input: None,
            bytes_output: None,
        }
    }
}

/// Information about verification performed during transformation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VerificationInfo {
    /// Whether input hashes were verified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_hash_verified: Option<bool>,

    /// Whether the transformation is deterministic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_deterministic: Option<bool>,

    /// Whether output is reproducible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_reproducible: Option<bool>,
}

impl Default for VerificationInfo {
    fn default() -> Self {
        Self {
            input_hash_verified: None,
            transform_deterministic: None,
            output_reproducible: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_attestation_builder() {
        let input = InputReference::new("dataset:input", Digest::new("a".repeat(64)));

        let transform =
            TransformDefinition::new("https://makoto.dev/transforms/filter", "Filter Records");

        let executor = Executor::new("https://example.com/executor/001");

        let attestation = TransformAttestation::builder()
            .subject(Subject::new("dataset:output", Digest::new("b".repeat(64))))
            .input(input)
            .transform(transform)
            .executor(executor)
            .build()
            .unwrap();

        assert_eq!(attestation.statement_type, IN_TOTO_STATEMENT_TYPE);
        assert_eq!(attestation.predicate_type, TRANSFORM_PREDICATE_TYPE);
        assert!(attestation.validate().is_ok());
    }

    #[test]
    fn test_transform_serialization() {
        let input = InputReference::new("dataset:input", Digest::new("a".repeat(64)));

        let transform =
            TransformDefinition::new("https://makoto.dev/transforms/filter", "Filter Records");

        let executor = Executor::new("https://example.com/executor/001");

        let attestation = TransformAttestation::builder()
            .subject(Subject::new("dataset:output", Digest::new("b".repeat(64))))
            .input(input)
            .transform(transform)
            .executor(executor)
            .build()
            .unwrap();

        let json = serde_json::to_string_pretty(&attestation).unwrap();
        let parsed: TransformAttestation = serde_json::from_str(&json).unwrap();

        assert_eq!(attestation, parsed);
    }
}
