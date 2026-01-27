/* eslint-disable */
/**
 * AUTO-GENERATED FILE - DO NOT EDIT
 * Generated from: transform-v1.json
 */

/**
 * JSON Schema for Makoto transform attestations, documenting data transformations with verifiable provenance. Built on in-toto Statement v1 format.
 */
export interface MakotoTransformAttestation {
  /**
   * in-toto Statement format identifier
   */
  _type: "https://in-toto.io/Statement/v1";
  /**
   * The output data artifacts produced by this transformation
   *
   * @minItems 1
   */
  subject: [Subject, ...Subject[]];
  /**
   * Makoto transform predicate type identifier
   */
  predicateType: "https://makoto.dev/transform/v1";
  predicate: TransformPredicate;
}
/**
 * Identifies an output data artifact with its cryptographic digest
 */
export interface Subject {
  /**
   * Identifier for the output dataset (e.g., 'dataset:customer_transactions_anonymized_2025q4')
   */
  name: string;
  digest: SubjectDigest;
}
/**
 * Cryptographic digest(s) of the output data
 */
export interface SubjectDigest {
  /**
   * SHA-256 hash of the output data (64 hex characters)
   */
  sha256: string;
  /**
   * SHA-512 hash of the output data (128 hex characters, optional)
   */
  sha512?: string;
  /**
   * Number of records in the output dataset
   */
  recordCount?: string;
  /**
   * Merkle tree root hash for record-level verification (64 hex characters)
   */
  merkleRoot?: string;
  [k: string]: unknown | undefined;
}
/**
 * The transform predicate describing what transformation was performed
 */
export interface TransformPredicate {
  /**
   * Input data artifacts that were transformed
   *
   * @minItems 1
   */
  inputs: [InputReference, ...InputReference[]];
  transform: TransformDefinition;
  executor: Executor;
  metadata?: ExecutionMetadata;
  verification?: VerificationInfo;
}
/**
 * Reference to an input data artifact
 */
export interface InputReference {
  /**
   * Identifier for the input dataset
   */
  name: string;
  digest: InputDigest;
  /**
   * URI to the attestation for this input (enables lineage chain verification)
   */
  attestationRef?: string;
  /**
   * Makoto level of the input attestation
   */
  makotoLevel?: "L1" | "L2" | "L3";
}
/**
 * Cryptographic digest(s) of the input data
 */
export interface InputDigest {
  /**
   * SHA-256 hash of the input data (64 hex characters)
   */
  sha256: string;
  /**
   * SHA-512 hash of the input data (128 hex characters, optional)
   */
  sha512?: string;
  [k: string]: unknown | undefined;
}
/**
 * Definition of the transformation that was applied
 */
export interface TransformDefinition {
  /**
   * URI identifying the transform type (e.g., 'https://makoto.dev/transforms/anonymization')
   */
  type: string;
  /**
   * Human-readable name of the transformation
   */
  name: string;
  /**
   * Version of the transformation definition
   */
  version?: string;
  /**
   * Human-readable description of what the transformation does
   */
  description?: string;
  /**
   * Configuration parameters used for this transformation
   */
  parameters?: {
    [k: string]: unknown | undefined;
  };
  codeRef?: CodeReference;
}
/**
 * Reference to the transformation code/configuration
 */
export interface CodeReference {
  /**
   * URI to the code repository (e.g., 'git+https://github.com/org/repo.git')
   */
  uri: string;
  /**
   * Git commit hash or other version identifier
   */
  commit?: string;
  /**
   * Path to the specific file within the repository
   */
  path?: string;
  /**
   * Cryptographic digest of the code file
   */
  digest?: {
    /**
     * SHA-256 hash of the code (64 hex characters in production)
     */
    sha256?: string;
    [k: string]: unknown | undefined;
  };
}
/**
 * Information about the system that executed the transformation
 */
export interface Executor {
  /**
   * Unique identifier for the execution environment
   */
  id: string;
  /**
   * Platform name (e.g., 'expanso', 'apache-spark', 'dbt')
   */
  platform?: string;
  /**
   * Version information for platform components
   */
  version?: {
    [k: string]: string | undefined;
  };
  /**
   * Execution environment (e.g., 'production', 'staging', 'development')
   */
  environment?: string;
  /**
   * Level of isolation for the execution environment
   */
  isolation?: "none" | "process" | "container" | "vm" | "hardware";
}
/**
 * Metadata about the transformation execution
 */
export interface ExecutionMetadata {
  /**
   * Unique identifier for this specific execution
   */
  invocationId?: string;
  /**
   * ISO 8601 timestamp when execution started
   */
  startedOn?: string;
  /**
   * ISO 8601 timestamp when execution completed
   */
  finishedOn?: string;
  /**
   * Execution duration in seconds
   */
  durationSeconds?: number;
  /**
   * Number of records read from inputs
   */
  recordsInput?: number;
  /**
   * Number of records written to output
   */
  recordsOutput?: number;
  /**
   * Number of records filtered/dropped during transformation
   */
  recordsDropped?: number;
  /**
   * Number of records that were modified
   */
  recordsModified?: number;
  /**
   * Total bytes read from inputs
   */
  bytesInput?: number;
  /**
   * Total bytes written to output
   */
  bytesOutput?: number;
}
/**
 * Information about verification performed during transformation
 */
export interface VerificationInfo {
  /**
   * Whether input data hashes were verified before processing
   */
  inputHashVerified?: boolean;
  /**
   * Whether the transformation produces deterministic output
   */
  transformDeterministic?: boolean;
  /**
   * Whether the output can be reproduced from inputs + transform definition
   */
  outputReproducible?: boolean;
}
