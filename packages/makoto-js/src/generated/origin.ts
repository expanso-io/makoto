/* eslint-disable */
/**
 * AUTO-GENERATED FILE - DO NOT EDIT
 * Generated from: origin-v1.json
 */

/**
 * JSON Schema for Makoto Data Origin Attestation (origin/v1). Documents the provenance of data at its point of collection or creation.
 */
export interface MakotoOriginAttestation {
  /**
   * in-toto Statement type identifier
   */
  _type: "https://in-toto.io/Statement/v1";
  /**
   * The dataset(s) this attestation describes
   *
   * @minItems 1
   */
  subject: [Subject, ...Subject[]];
  /**
   * Makoto origin predicate type identifier
   */
  predicateType: "https://makoto.dev/origin/v1";
  predicate: OriginPredicate;
}
export interface Subject {
  /**
   * Identifier for the dataset, typically in format 'dataset:<name>'
   */
  name: string;
  /**
   * Cryptographic digests identifying the dataset contents
   */
  digest: {
    /**
     * SHA-256 hash of the dataset contents
     */
    sha256: string;
    /**
     * SHA-512 hash of the dataset contents
     */
    sha512?: string;
    /**
     * Number of records in the dataset (as string for large numbers)
     */
    recordCount?: string;
    /**
     * Root hash of Merkle tree over dataset records
     */
    merkleRoot?: string;
    /**
     * Additional digest algorithms
     */
    [k: string]: string | undefined;
  };
}
/**
 * The origin predicate containing provenance information
 */
export interface OriginPredicate {
  origin: Origin;
  collector: Collector;
  schema?: Schema;
  metadata?: Metadata;
  dtaCompliance?: DtaCompliance;
}
/**
 * Information about where and how the data was collected
 */
export interface Origin {
  /**
   * URI identifying the data source (API endpoint, database, file location, etc.)
   */
  source: string;
  /**
   * Category of the data source
   */
  sourceType: "api" | "database" | "file" | "stream" | "manual" | "sensor" | "other";
  /**
   * How the data was collected from the source
   */
  collectionMethod: "pull" | "push" | "scheduled-pull" | "event-driven" | "batch-upload" | "streaming" | "manual";
  /**
   * ISO 8601 timestamp when data collection occurred
   */
  collectionTimestamp: string;
  /**
   * Geographic region where data was collected (e.g., 'US-WEST-2', 'EU', 'APAC')
   */
  geography?: string;
  consent?: Consent;
}
/**
 * Consent and legal basis for data collection
 */
export interface Consent {
  /**
   * Legal basis for data collection (aligned with GDPR Article 6)
   */
  type: "contractual" | "consent" | "legitimate-interest" | "legal-obligation" | "public-interest" | "vital-interest";
  /**
   * URI to consent documentation, DPA, or legal agreement
   */
  reference?: string;
  /**
   * When consent/agreement was obtained
   */
  obtained?: string;
  /**
   * When consent/agreement expires (if applicable)
   */
  expires?: string;
}
/**
 * Information about the system that collected the data
 */
export interface Collector {
  /**
   * Unique identifier for the collector instance
   */
  id: string;
  /**
   * Version information for collector software components
   */
  version?: {
    [k: string]: string | undefined;
  };
  /**
   * Deployment environment of the collector
   */
  environment?: "production" | "staging" | "development" | "test";
  /**
   * Platform or runtime environment (e.g., 'expanso', 'kubernetes', 'aws-lambda')
   */
  platform?: string;
}
/**
 * Schema information for the collected data
 */
export interface Schema {
  /**
   * Data format (e.g., 'json-lines', 'csv', 'parquet', 'avro')
   */
  format: string;
  /**
   * URI to the schema definition
   */
  schemaRef?: string;
  /**
   * Digest of the schema for integrity verification
   */
  schemaDigest?: {
    sha256?: string;
  };
  /**
   * Version of the schema
   */
  schemaVersion?: string;
}
/**
 * Collection statistics and metrics
 */
export interface Metadata {
  /**
   * ISO 8601 duration of the collection process
   */
  collectionDuration?: string;
  /**
   * Total bytes of data collected
   */
  bytesCollected?: number;
  /**
   * Number of records successfully collected
   */
  recordsCollected?: number;
  /**
   * Number of records dropped during collection
   */
  recordsDropped?: number;
  /**
   * Fraction of records that encountered errors (0.0 to 1.0)
   */
  errorRate?: number;
  /**
   * When collection started
   */
  startTime?: string;
  /**
   * When collection completed
   */
  endTime?: string;
}
/**
 * D&TA Data Provenance Standards v1.0.0 compliance information
 */
export interface DtaCompliance {
  /**
   * Version of D&TA standards being followed
   */
  standardsVersion?: string;
  /**
   * D&TA Source Standard fields
   */
  sourceStandard?: {
    /**
     * Human-readable title for the dataset
     */
    datasetTitle?: string;
    /**
     * Organization that issued/provided the data
     */
    datasetIssuer?: string;
    /**
     * Description of the dataset contents and purpose
     */
    description?: string;
  };
  /**
   * D&TA Provenance Standard fields
   */
  provenanceStandard?: {
    /**
     * Geographic origin of the data
     */
    dataOriginGeography?: string;
    /**
     * Description of collection method
     */
    method?: string;
    /**
     * Format of the data
     */
    dataFormat?: string;
  };
  /**
   * D&TA Use Standard fields
   */
  useStandard?: {
    /**
     * Data confidentiality level
     */
    confidentialityClassification?: "public" | "internal" | "confidential" | "restricted";
    /**
     * Intended use for this data
     */
    intendedDataUse?: string;
    /**
     * License or usage terms
     */
    license?: string;
  };
}
