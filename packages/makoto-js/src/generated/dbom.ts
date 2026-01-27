/* eslint-disable */
/**
 * AUTO-GENERATED FILE - DO NOT EDIT
 * Generated from: dbom-v1.json
 */

/**
 * Makoto attestation level achieved
 */
export type MakotoLevel = "L1" | "L2" | "L3";

/**
 * A comprehensive manifest documenting the provenance, lineage, and compliance status of a dataset, including all source data and transformations applied.
 */
export interface DataBillOfMaterialsDBOM {
  /**
   * Version of the DBOM specification this document conforms to
   */
  dbomVersion: string;
  /**
   * Unique identifier for this DBOM, typically a URN
   */
  dbomId: string;
  dataset: Dataset;
  /**
   * List of source datasets that contribute to the final dataset
   *
   * @minItems 1
   */
  sources: [Source, ...Source[]];
  /**
   * Ordered list of transformations applied to produce the final dataset
   */
  transformations?: Transformation[];
  lineageGraph?: LineageGraph;
  compliance?: Compliance;
  verification?: Verification;
  metadata?: Metadata;
}
/**
 * Information about the final dataset described by this DBOM
 */
export interface Dataset {
  /**
   * Human-readable name of the dataset
   */
  name: string;
  /**
   * Version of the dataset
   */
  version: string;
  /**
   * Human-readable description of the dataset's purpose and contents
   */
  description?: string;
  /**
   * ISO 8601 timestamp when the dataset was created
   */
  created: string;
  creator?: Creator;
  digest: Digest;
  makotoLevel: MakotoLevel;
}
/**
 * Entity responsible for creating the dataset
 */
export interface Creator {
  /**
   * Name of the organization
   */
  organization?: string;
  /**
   * Contact email or URI for the responsible party
   */
  contact?: string;
}
/**
 * Cryptographic digest and metadata about the dataset contents
 */
export interface Digest {
  /**
   * SHA-256 hash of the dataset contents
   */
  sha256: string;
  /**
   * Optional SHA-512 hash of the dataset contents
   */
  sha512?: string;
  /**
   * Number of records in the dataset
   */
  recordCount?: string | number;
  /**
   * File format of the dataset
   */
  format?: string;
  /**
   * Size of the dataset in bytes
   */
  sizeBytes?: number;
}
/**
 * A source dataset that contributes to the final dataset
 */
export interface Source {
  /**
   * Identifier for this source dataset
   */
  name: string;
  /**
   * Human-readable description of the source
   */
  description?: string;
  /**
   * URI reference to the attestation document for this source
   */
  attestationRef?: string;
  /**
   * URI identifying the attestation predicate type
   */
  attestationType: string;
  makotoLevel: MakotoLevel;
  /**
   * Geographic region where the data was collected or processed
   */
  geography?: string;
  consent?: Consent;
  license?: License;
  contribution?: Contribution;
}
/**
 * Information about consent for data usage
 */
export interface Consent {
  /**
   * Type of consent obtained
   */
  type?: "explicit" | "contractual" | "legitimate-interest" | "public-task";
  /**
   * URI reference to the consent documentation
   */
  reference?: string;
}
/**
 * License information for the source data
 */
export interface License {
  /**
   * Type of license
   */
  type?: string;
  /**
   * SPDX license identifier or similar standard identifier
   */
  identifier?: string;
  /**
   * URI reference to the license documentation
   */
  reference?: string;
}
/**
 * Information about how much this source contributes to the final dataset
 */
export interface Contribution {
  /**
   * Number of records contributed by this source
   */
  recordCount?: number;
  /**
   * Percentage of final dataset records from this source
   */
  recordPercentage?: number;
}
/**
 * A transformation step in the data pipeline
 */
export interface Transformation {
  /**
   * Sequence number of this transformation in the pipeline
   */
  order: number;
  /**
   * Human-readable name for the transformation
   */
  name: string;
  /**
   * Detailed description of what the transformation does
   */
  description?: string;
  /**
   * URI reference to the attestation document for this transformation
   */
  attestationRef?: string;
  /**
   * URI identifying the attestation predicate type
   */
  attestationType: string;
  makotoLevel: MakotoLevel;
  /**
   * Names of input datasets consumed by this transformation
   *
   * @minItems 1
   */
  inputs: [string, ...string[]];
  /**
   * Names of output datasets produced by this transformation
   *
   * @minItems 1
   */
  outputs: [string, ...string[]];
  /**
   * URI identifying the type of transformation
   */
  transformType?: string;
}
/**
 * Visual representation of the data lineage
 */
export interface LineageGraph {
  /**
   * Format of the graph content
   */
  format?: "graphviz-dot" | "mermaid" | "json-ld" | "cytoscape";
  /**
   * The graph content in the specified format
   */
  content?: string;
  /**
   * URL to an external lineage graph representation
   */
  url?: string;
}
/**
 * Compliance and regulatory status of the dataset
 */
export interface Compliance {
  overallMakotoLevel?: MakotoLevel;
  /**
   * Explanation of how the overall Makoto level was determined
   */
  levelJustification?: string;
  privacyAssessment?: PrivacyAssessment;
  /**
   * List of regulatory compliance statuses
   */
  regulatoryCompliance?: RegulatoryStatus[];
  dtaCompliance?: DtaCompliance;
}
/**
 * Privacy and anonymization assessment
 */
export interface PrivacyAssessment {
  /**
   * Whether personally identifiable information has been removed
   */
  piiRemoved?: boolean;
  /**
   * Whether anonymization has been verified
   */
  anonymizationVerified?: boolean;
  /**
   * k-anonymity level achieved
   */
  kAnonymity?: number;
  /**
   * l-diversity level achieved
   */
  lDiversity?: number;
  /**
   * t-closeness threshold achieved
   */
  tCloseness?: number;
  /**
   * Differential privacy parameters if applicable
   */
  differentialPrivacy?: {
    epsilon?: number;
    delta?: number;
  };
}
/**
 * Compliance status for a specific regulation
 */
export interface RegulatoryStatus {
  /**
   * Name or identifier of the regulation
   */
  regulation: string;
  /**
   * Compliance status
   */
  status: "compliant" | "non-compliant" | "partial" | "not-applicable" | "pending-review";
  /**
   * Additional notes about the compliance status
   */
  notes?: string;
  /**
   * Date of the compliance assessment
   */
  assessmentDate?: string;
  /**
   * Person or entity that performed the assessment
   */
  assessor?: string;
}
/**
 * D&TA Data Provenance Standards compliance
 */
export interface DtaCompliance {
  /**
   * Version of D&TA standards this DBOM conforms to
   */
  standardsVersion?: string;
  /**
   * Whether all required D&TA fields are present
   */
  allFieldsPresent?: boolean;
}
/**
 * Results of attestation chain verification
 */
export interface Verification {
  /**
   * Whether the complete attestation chain has been verified
   */
  chainVerified?: boolean;
  /**
   * Whether all cryptographic signatures are valid
   */
  allSignaturesValid?: boolean;
  /**
   * Number of attestations in the chain
   */
  attestationCount?: number;
  /**
   * When the verification was performed
   */
  verificationTimestamp?: string;
  /**
   * Tool that performed the verification
   */
  verifier?: {
    /**
     * Name of the verification tool
     */
    tool?: string;
    /**
     * Version of the verification tool
     */
    version?: string;
  };
  /**
   * List of verification errors if any
   */
  errors?: {
    code?: string;
    message?: string;
    attestationRef?: string;
  }[];
}
/**
 * Metadata about the DBOM document itself
 */
export interface Metadata {
  /**
   * Tool that generated this DBOM
   */
  generator?: {
    /**
     * Name of the generator tool
     */
    tool?: string;
    /**
     * Version of the generator tool
     */
    version?: string;
  };
  /**
   * When this DBOM was generated
   */
  created?: string;
  /**
   * Expiration date for this DBOM
   */
  validUntil?: string;
  /**
   * Access control settings for this DBOM
   */
  accessControl?: {
    /**
     * Visibility level
     */
    visibility?: "public" | "internal" | "confidential" | "restricted";
    /**
     * List of entities allowed to access this DBOM
     */
    allowedConsumers?: string[];
  };
  /**
   * Optional tags for categorization
   */
  tags?: string[];
}
