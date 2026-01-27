/**
 * Builder for Makoto Stream Window Predicates.
 *
 * Stream window predicates capture integrity attestations for bounded windows
 * of streaming data using Merkle trees and hash chaining.
 *
 * @example
 * ```ts
 * const predicate = new StreamWindowPredicateBuilder()
 *   .withStream({ id: "iot_sensors" })
 *   .withWindow({ type: "tumbling", duration: "PT1M" })
 *   .withIntegrity({
 *     merkleTree: {
 *       algorithm: "sha256",
 *       leafCount: 1000,
 *       root: "merkle_root_hash..."
 *     }
 *   })
 *   .build();
 * ```
 */

import type {
  MakotoStreamWindowPredicate,
  StreamDescriptor,
  WindowDescriptor,
  IntegrityDescriptor,
  MerkleTreeDescriptor,
  ChainDescriptor,
  AggregatesDescriptor,
  CollectorDescriptor,
  MetadataDescriptor,
  VerificationDescriptor,
} from "../generated/stream-window.js";

export type {
  MakotoStreamWindowPredicate,
  StreamDescriptor,
  WindowDescriptor,
  IntegrityDescriptor,
  MerkleTreeDescriptor,
  ChainDescriptor,
  AggregatesDescriptor,
  CollectorDescriptor,
  MetadataDescriptor,
  VerificationDescriptor,
};

/**
 * Window type options.
 */
export const WindowTypes = ["tumbling", "sliding", "session"] as const;
export type WindowType = (typeof WindowTypes)[number];

/**
 * Time alignment options.
 */
export const TimeAlignments = ["wall-clock", "event-time"] as const;
export type TimeAlignment = (typeof TimeAlignments)[number];

/**
 * Hash algorithm options for Merkle trees.
 */
export const HashAlgorithms = [
  "sha256",
  "sha384",
  "sha512",
  "sha3-256",
  "sha3-384",
  "sha3-512",
  "blake2b",
  "blake3",
] as const;
export type HashAlgorithm = (typeof HashAlgorithms)[number];

/**
 * Builder class for creating Makoto Stream Window Predicates.
 */
export class StreamWindowPredicateBuilder {
  private stream?: StreamDescriptor;
  private window?: WindowDescriptor;
  private integrity?: IntegrityDescriptor;
  private aggregates?: AggregatesDescriptor;
  private collector?: CollectorDescriptor;
  private metadata?: MetadataDescriptor;
  private verification?: VerificationDescriptor;

  /**
   * Set the stream descriptor (required).
   * Identifies the data stream being attested.
   */
  withStream(stream: StreamDescriptor): this {
    this.stream = stream;
    return this;
  }

  /**
   * Set the window descriptor (required).
   * Defines the windowing parameters for stream aggregation.
   */
  withWindow(window: WindowDescriptor): this {
    this.window = window;
    return this;
  }

  /**
   * Set the integrity descriptor (required).
   * Contains cryptographic integrity information for the window.
   */
  withIntegrity(integrity: IntegrityDescriptor): this {
    this.integrity = integrity;
    return this;
  }

  /**
   * Set aggregate values (optional).
   * Computed over the window for quick verification and analysis.
   */
  withAggregates(aggregates: AggregatesDescriptor): this {
    this.aggregates = aggregates;
    return this;
  }

  /**
   * Set collector information (optional).
   * Describes the system that collected and attested this window.
   */
  withCollector(collector: CollectorDescriptor): this {
    this.collector = collector;
    return this;
  }

  /**
   * Set operational metadata (optional).
   * Includes processing latency, late records, etc.
   */
  withMetadata(metadata: MetadataDescriptor): this {
    this.metadata = metadata;
    return this;
  }

  /**
   * Set verification information (optional).
   * Describes how to verify individual records within the window.
   */
  withVerification(verification: VerificationDescriptor): this {
    this.verification = verification;
    return this;
  }

  /**
   * Build the stream window predicate.
   * @throws Error if required fields are missing
   */
  build(): MakotoStreamWindowPredicate {
    if (!this.stream) {
      throw new Error("Stream descriptor is required");
    }
    if (!this.window) {
      throw new Error("Window descriptor is required");
    }
    if (!this.integrity) {
      throw new Error("Integrity descriptor is required");
    }

    // Validate sliding windows require slide parameter
    if (this.window.type === "sliding" && !this.window.slide) {
      throw new Error("Sliding windows require a slide interval");
    }

    const predicate: MakotoStreamWindowPredicate = {
      stream: this.stream,
      window: this.window,
      integrity: this.integrity,
      ...(this.aggregates && { aggregates: this.aggregates }),
      ...(this.collector && { collector: this.collector }),
      ...(this.metadata && { metadata: this.metadata }),
      ...(this.verification && { verification: this.verification }),
    };

    return predicate;
  }

  /**
   * Reset the builder to its initial state.
   */
  reset(): this {
    this.stream = undefined;
    this.window = undefined;
    this.integrity = undefined;
    this.aggregates = undefined;
    this.collector = undefined;
    this.metadata = undefined;
    this.verification = undefined;
    return this;
  }
}

/**
 * Helper function to create a stream descriptor.
 */
export function createStreamDescriptor(
  id: string,
  options?: {
    source?: string;
    topic?: string;
    partitions?: [string, ...string[]];
  }
): StreamDescriptor {
  return {
    id,
    ...options,
  };
}

/**
 * Helper function to create a tumbling window descriptor.
 */
export function createTumblingWindow(
  duration: string,
  options?: {
    alignment?: TimeAlignment;
    watermark?: string;
    allowedLateness?: string;
  }
): WindowDescriptor {
  return {
    type: "tumbling",
    duration,
    ...options,
  };
}

/**
 * Helper function to create a sliding window descriptor.
 */
export function createSlidingWindow(
  duration: string,
  slide: string,
  options?: {
    alignment?: TimeAlignment;
    watermark?: string;
    allowedLateness?: string;
  }
): WindowDescriptor {
  return {
    type: "sliding",
    duration,
    slide,
    ...options,
  };
}

/**
 * Helper function to create a session window descriptor.
 */
export function createSessionWindow(
  gap: string,
  options?: {
    alignment?: TimeAlignment;
    watermark?: string;
    allowedLateness?: string;
  }
): WindowDescriptor {
  return {
    type: "session",
    duration: gap,
    ...options,
  };
}

/**
 * Helper function to create a Merkle tree descriptor.
 */
export function createMerkleTree(
  algorithm: HashAlgorithm,
  leafCount: number,
  root: string,
  options?: {
    leafHashAlgorithm?: HashAlgorithm;
    treeHeight?: number;
  }
): MerkleTreeDescriptor {
  return {
    algorithm,
    leafCount,
    root,
    ...options,
  };
}

/**
 * Helper function to create integrity descriptor with Merkle tree.
 */
export function createIntegrity(
  merkleTree: MerkleTreeDescriptor,
  chain?: ChainDescriptor
): IntegrityDescriptor {
  return {
    merkleTree,
    ...(chain && { chain }),
  };
}

/**
 * Helper function to create chain descriptor for linking windows.
 */
export function createChain(options: {
  previousWindowId?: string;
  previousMerkleRoot?: string;
  chainLength?: number;
  genesisWindowId?: string;
}): ChainDescriptor {
  // Validate that previousWindowId and previousMerkleRoot are both present or both absent
  const hasPrevId = options.previousWindowId !== undefined;
  const hasPrevRoot = options.previousMerkleRoot !== undefined;
  if (hasPrevId !== hasPrevRoot) {
    throw new Error(
      "previousWindowId and previousMerkleRoot must be specified together"
    );
  }
  return options;
}
