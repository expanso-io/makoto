/**
 * Builder for Makoto Origin Attestations.
 *
 * Origin attestations document data provenance at the point of collection.
 *
 * @example
 * ```ts
 * const attestation = new OriginAttestationBuilder()
 *   .addSubject({
 *     name: "dataset:customer_transactions_2025q4",
 *     digest: { sha256: "abc123..." }
 *   })
 *   .withOrigin({
 *     source: "https://api.partner.com/transactions",
 *     sourceType: "api",
 *     collectionMethod: "pull",
 *     collectionTimestamp: new Date().toISOString()
 *   })
 *   .withCollector({ id: "https://example.com/collectors/prod-1" })
 *   .build();
 * ```
 */

import type {
  MakotoOriginAttestation,
  Subject,
  Origin,
  Collector,
  Schema,
  Metadata,
  DtaCompliance,
  Consent,
} from "../generated/origin.js";

export type { Subject, Origin, Collector, Schema, Metadata, DtaCompliance, Consent };

/**
 * Source type options for data origin.
 */
export const SourceTypes = [
  "api",
  "database",
  "file",
  "stream",
  "manual",
  "sensor",
  "other",
] as const;

/**
 * Collection method options.
 */
export const CollectionMethods = [
  "pull",
  "push",
  "scheduled-pull",
  "event-driven",
  "batch-upload",
  "streaming",
  "manual",
] as const;

/**
 * Consent type options aligned with GDPR Article 6.
 */
export const ConsentTypes = [
  "contractual",
  "consent",
  "legitimate-interest",
  "legal-obligation",
  "public-interest",
  "vital-interest",
] as const;

/**
 * Builder class for creating Makoto Origin Attestations.
 */
export class OriginAttestationBuilder {
  private subjects: Subject[] = [];
  private origin?: Origin;
  private collector?: Collector;
  private schema?: Schema;
  private metadata?: Metadata;
  private dtaCompliance?: DtaCompliance;

  /**
   * Add a subject (dataset) to this attestation.
   * At least one subject is required.
   */
  addSubject(subject: Subject): this {
    this.subjects.push(subject);
    return this;
  }

  /**
   * Add multiple subjects at once.
   */
  addSubjects(subjects: Subject[]): this {
    this.subjects.push(...subjects);
    return this;
  }

  /**
   * Set the origin information (required).
   * Describes where and how the data was collected.
   */
  withOrigin(origin: Origin): this {
    this.origin = origin;
    return this;
  }

  /**
   * Set the collector information (required).
   * Describes the system that collected the data.
   */
  withCollector(collector: Collector): this {
    this.collector = collector;
    return this;
  }

  /**
   * Set schema information for the collected data (optional).
   */
  withSchema(schema: Schema): this {
    this.schema = schema;
    return this;
  }

  /**
   * Set collection metadata (optional).
   * Includes statistics like bytes collected, records collected, etc.
   */
  withMetadata(metadata: Metadata): this {
    this.metadata = metadata;
    return this;
  }

  /**
   * Set D&TA Data Provenance Standards compliance information (optional).
   */
  withDtaCompliance(dtaCompliance: DtaCompliance): this {
    this.dtaCompliance = dtaCompliance;
    return this;
  }

  /**
   * Build the origin attestation.
   * @throws Error if required fields are missing
   */
  build(): MakotoOriginAttestation {
    if (this.subjects.length === 0) {
      throw new Error("At least one subject is required");
    }
    if (!this.origin) {
      throw new Error("Origin information is required");
    }
    if (!this.collector) {
      throw new Error("Collector information is required");
    }

    const attestation: MakotoOriginAttestation = {
      _type: "https://in-toto.io/Statement/v1",
      subject: this.subjects as [Subject, ...Subject[]],
      predicateType: "https://makoto.dev/origin/v1",
      predicate: {
        origin: this.origin,
        collector: this.collector,
        ...(this.schema && { schema: this.schema }),
        ...(this.metadata && { metadata: this.metadata }),
        ...(this.dtaCompliance && { dtaCompliance: this.dtaCompliance }),
      },
    };

    return attestation;
  }

  /**
   * Reset the builder to its initial state.
   */
  reset(): this {
    this.subjects = [];
    this.origin = undefined;
    this.collector = undefined;
    this.schema = undefined;
    this.metadata = undefined;
    this.dtaCompliance = undefined;
    return this;
  }
}

/**
 * Helper function to create a subject with minimal required fields.
 */
export function createSubject(
  name: string,
  sha256: string,
  options?: {
    sha512?: string;
    recordCount?: string;
    merkleRoot?: string;
  }
): Subject {
  return {
    name,
    digest: {
      sha256,
      ...options,
    },
  };
}

/**
 * Helper function to create origin info with minimal required fields.
 */
export function createOrigin(
  source: string,
  sourceType: Origin["sourceType"],
  collectionMethod: Origin["collectionMethod"],
  collectionTimestamp?: string,
  options?: {
    geography?: string;
    consent?: Consent;
  }
): Origin {
  return {
    source,
    sourceType,
    collectionMethod,
    collectionTimestamp: collectionTimestamp ?? new Date().toISOString(),
    ...options,
  };
}

/**
 * Helper function to create collector info with minimal required fields.
 */
export function createCollector(
  id: string,
  options?: {
    version?: Record<string, string>;
    environment?: Collector["environment"];
    platform?: string;
  }
): Collector {
  return {
    id,
    ...options,
  };
}
