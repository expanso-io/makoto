/* eslint-disable */
/**
 * AUTO-GENERATED FILE - DO NOT EDIT
 * Generated from: stream-window-v1.json
 */

/**
 * JSON Schema for the makoto.dev/stream-window/v1 predicate type. This predicate captures integrity attestations for bounded windows of streaming data using Merkle trees and hash chaining.
 */
export interface MakotoStreamWindowPredicate {
  stream: StreamDescriptor;
  window: WindowDescriptor;
  integrity: IntegrityDescriptor;
  aggregates?: AggregatesDescriptor;
  collector?: CollectorDescriptor;
  metadata?: MetadataDescriptor;
  verification?: VerificationDescriptor;
}
/**
 * Identifies the data stream being attested
 */
export interface StreamDescriptor {
  /**
   * Unique identifier for the stream
   */
  id: string;
  /**
   * URI of the stream source (e.g., mqtt://, kafka://, https://)
   */
  source?: string;
  /**
   * Topic pattern or name for the stream
   */
  topic?: string;
  /**
   * List of partition identifiers included in this window
   *
   * @minItems 1
   */
  partitions?: [string, ...string[]];
}
/**
 * Defines the windowing parameters for stream aggregation
 */
export interface WindowDescriptor {
  /**
   * Window type: tumbling (fixed non-overlapping), sliding (overlapping), or session (activity-based gaps)
   */
  type: "tumbling" | "sliding" | "session";
  /**
   * Window duration (for tumbling/sliding) or session gap timeout
   */
  duration: string;
  /**
   * Slide interval for sliding windows (required when type is 'sliding')
   */
  slide?: string;
  /**
   * Time alignment strategy: wall-clock (processing time) or event-time (data timestamps)
   */
  alignment?: "wall-clock" | "event-time";
  /**
   * Watermark timestamp indicating progress of event-time processing
   */
  watermark?: string;
  /**
   * Maximum allowed lateness for late-arriving records
   */
  allowedLateness?: string;
}
/**
 * Cryptographic integrity information for the window
 */
export interface IntegrityDescriptor {
  merkleTree: MerkleTreeDescriptor;
  chain?: ChainDescriptor;
}
/**
 * Merkle tree parameters enabling efficient integrity verification of window records
 */
export interface MerkleTreeDescriptor {
  /**
   * Hash algorithm used for internal tree nodes
   */
  algorithm: "sha256" | "sha384" | "sha512" | "sha3-256" | "sha3-384" | "sha3-512" | "blake2b" | "blake3";
  /**
   * Hash algorithm used for leaf nodes (defaults to 'algorithm' if not specified)
   */
  leafHashAlgorithm?: "sha256" | "sha384" | "sha512" | "sha3-256" | "sha3-384" | "sha3-512" | "blake2b" | "blake3";
  /**
   * Number of leaf nodes (records) in the Merkle tree
   */
  leafCount: number;
  /**
   * Height of the Merkle tree (ceil(log2(leafCount)) + 1)
   */
  treeHeight?: number;
  /**
   * Root hash of the Merkle tree - this is the signed integrity value
   */
  root: string;
}
/**
 * Hash chain linking this window to previous windows for tamper-evident sequencing
 */
export interface ChainDescriptor {
  /**
   * Identifier of the immediately preceding window
   */
  previousWindowId?: string;
  /**
   * Merkle root of the previous window (enables chain verification)
   */
  previousMerkleRoot?: string;
  /**
   * Position in the chain (1 = genesis window)
   */
  chainLength?: number;
  /**
   * Identifier of the first window in this chain
   */
  genesisWindowId?: string;
}
/**
 * Aggregate values computed over the window for quick verification and analysis
 */
export interface AggregatesDescriptor {
  /**
   * Simple checksum of all records (for quick integrity check)
   */
  checksum?: string;
  /**
   * Statistical aggregates over the window data
   */
  statistics?: {
    /**
     * Earliest record timestamp in the window
     */
    minTimestamp?: string;
    /**
     * Latest record timestamp in the window
     */
    maxTimestamp?: string;
    /**
     * Average interval between records in milliseconds
     */
    avgIntervalMs?: number;
    [k: string]: unknown | undefined;
  };
}
/**
 * Information about the system that collected and attested this window
 */
export interface CollectorDescriptor {
  /**
   * Unique identifier URI for the collector
   */
  id: string;
  /**
   * Version information for collector software components
   */
  version?: {
    [k: string]: string | undefined;
  };
  /**
   * Physical or logical location identifier of the collector
   */
  location?: string;
}
/**
 * Operational metadata about window processing
 */
export interface MetadataDescriptor {
  /**
   * Time taken to process and close the window
   */
  processingLatency?: string;
  /**
   * Number of records that arrived after the watermark but within allowed lateness
   */
  lateRecords?: number;
  /**
   * Number of records dropped (arrived after allowed lateness)
   */
  droppedRecords?: number;
  /**
   * Number of backpressure events during window processing
   */
  backpressureEvents?: number;
  [k: string]: unknown | undefined;
}
/**
 * Information for verifying individual records within the window
 */
export interface VerificationDescriptor {
  /**
   * Whether Merkle proofs are available for individual record verification
   */
  merkleProofAvailable?: boolean;
  /**
   * Endpoint URL for retrieving Merkle proofs
   */
  proofEndpoint?: string;
}
