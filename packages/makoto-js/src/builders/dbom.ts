/**
 * Builder for Data Bill of Materials (DBOM).
 *
 * A DBOM is a comprehensive manifest documenting the provenance, lineage,
 * and compliance status of a dataset.
 *
 * @example
 * ```ts
 * const dbom = new DBOMBuilder()
 *   .withVersion("1.0.0")
 *   .withId("urn:dbom:example.com:fraud-detection-v3")
 *   .withDataset({
 *     name: "Fraud Detection Training Data",
 *     version: "3.0.0",
 *     created: new Date().toISOString(),
 *     digest: { sha256: "..." },
 *     makotoLevel: "L2"
 *   })
 *   .addSource({
 *     name: "raw_transactions",
 *     attestationType: "https://makoto.dev/origin/v1",
 *     makotoLevel: "L2"
 *   })
 *   .build();
 * ```
 */

import type {
  DataBillOfMaterialsDBOM,
  Dataset,
  Source,
  Transformation,
  LineageGraph,
  Compliance,
  Verification,
  Metadata,
  MakotoLevel,
  Digest,
  Creator,
  Consent,
  License,
  Contribution,
  PrivacyAssessment,
  RegulatoryStatus,
} from "../generated/dbom.js";

export type {
  DataBillOfMaterialsDBOM,
  Dataset,
  Source,
  Transformation,
  LineageGraph,
  Compliance,
  Verification,
  Metadata,
  MakotoLevel,
  Digest,
  Creator,
  Consent,
  License,
  Contribution,
  PrivacyAssessment,
  RegulatoryStatus,
};

/**
 * Current DBOM specification version.
 */
export const DBOM_VERSION = "1.0.0";

/**
 * Makoto level options.
 */
export const MakotoLevels = ["L1", "L2", "L3"] as const;

/**
 * Lineage graph format options.
 */
export const LineageGraphFormats = [
  "graphviz-dot",
  "mermaid",
  "json-ld",
  "cytoscape",
] as const;
export type LineageGraphFormat = (typeof LineageGraphFormats)[number];

/**
 * Regulatory compliance status options.
 */
export const ComplianceStatuses = [
  "compliant",
  "non-compliant",
  "partial",
  "not-applicable",
  "pending-review",
] as const;
export type ComplianceStatus = (typeof ComplianceStatuses)[number];

/**
 * Builder class for creating Data Bill of Materials (DBOM).
 */
export class DBOMBuilder {
  private dbomVersion: string = DBOM_VERSION;
  private dbomId?: string;
  private dataset?: Dataset;
  private sources: Source[] = [];
  private transformations: Transformation[] = [];
  private lineageGraph?: LineageGraph;
  private compliance?: Compliance;
  private verification?: Verification;
  private metadata?: Metadata;

  /**
   * Set the DBOM specification version.
   * Defaults to the current version if not specified.
   */
  withVersion(version: string): this {
    this.dbomVersion = version;
    return this;
  }

  /**
   * Set the DBOM unique identifier (required).
   * Should be a URN in format "urn:dbom:<organization>:<name>".
   */
  withId(id: string): this {
    this.dbomId = id;
    return this;
  }

  /**
   * Set the dataset information (required).
   * Describes the final dataset this DBOM documents.
   */
  withDataset(dataset: Dataset): this {
    this.dataset = dataset;
    return this;
  }

  /**
   * Add a source dataset (at least one required).
   */
  addSource(source: Source): this {
    this.sources.push(source);
    return this;
  }

  /**
   * Add multiple sources at once.
   */
  addSources(sources: Source[]): this {
    this.sources.push(...sources);
    return this;
  }

  /**
   * Add a transformation step.
   */
  addTransformation(transformation: Transformation): this {
    this.transformations.push(transformation);
    return this;
  }

  /**
   * Add multiple transformations at once.
   */
  addTransformations(transformations: Transformation[]): this {
    this.transformations.push(...transformations);
    return this;
  }

  /**
   * Set the lineage graph (optional).
   * Visual representation of the data lineage.
   */
  withLineageGraph(lineageGraph: LineageGraph): this {
    this.lineageGraph = lineageGraph;
    return this;
  }

  /**
   * Set compliance information (optional).
   * Regulatory and compliance status of the dataset.
   */
  withCompliance(compliance: Compliance): this {
    this.compliance = compliance;
    return this;
  }

  /**
   * Set verification results (optional).
   * Results of attestation chain verification.
   */
  withVerification(verification: Verification): this {
    this.verification = verification;
    return this;
  }

  /**
   * Set DBOM metadata (optional).
   * Information about the DBOM document itself.
   */
  withMetadata(metadata: Metadata): this {
    this.metadata = metadata;
    return this;
  }

  /**
   * Build the DBOM.
   * @throws Error if required fields are missing
   */
  build(): DataBillOfMaterialsDBOM {
    if (!this.dbomId) {
      throw new Error("DBOM ID is required");
    }
    if (!this.dataset) {
      throw new Error("Dataset information is required");
    }
    if (this.sources.length === 0) {
      throw new Error("At least one source is required");
    }

    // Sort transformations by order if present
    const sortedTransformations =
      this.transformations.length > 0
        ? [...this.transformations].sort((a, b) => a.order - b.order)
        : undefined;

    const dbom: DataBillOfMaterialsDBOM = {
      dbomVersion: this.dbomVersion,
      dbomId: this.dbomId,
      dataset: this.dataset,
      sources: this.sources as [Source, ...Source[]],
      ...(sortedTransformations && { transformations: sortedTransformations }),
      ...(this.lineageGraph && { lineageGraph: this.lineageGraph }),
      ...(this.compliance && { compliance: this.compliance }),
      ...(this.verification && { verification: this.verification }),
      ...(this.metadata && { metadata: this.metadata }),
    };

    return dbom;
  }

  /**
   * Reset the builder to its initial state.
   */
  reset(): this {
    this.dbomVersion = DBOM_VERSION;
    this.dbomId = undefined;
    this.dataset = undefined;
    this.sources = [];
    this.transformations = [];
    this.lineageGraph = undefined;
    this.compliance = undefined;
    this.verification = undefined;
    this.metadata = undefined;
    return this;
  }
}

/**
 * Helper function to create a dataset descriptor.
 */
export function createDataset(
  name: string,
  version: string,
  digest: Digest,
  makotoLevel: MakotoLevel,
  options?: {
    description?: string;
    created?: string;
    creator?: Creator;
  }
): Dataset {
  return {
    name,
    version,
    created: options?.created ?? new Date().toISOString(),
    digest,
    makotoLevel,
    ...(options?.description && { description: options.description }),
    ...(options?.creator && { creator: options.creator }),
  };
}

/**
 * Helper function to create a source descriptor.
 */
export function createSource(
  name: string,
  attestationType: string,
  makotoLevel: MakotoLevel,
  options?: {
    description?: string;
    attestationRef?: string;
    geography?: string;
    consent?: Consent;
    license?: License;
    contribution?: Contribution;
  }
): Source {
  return {
    name,
    attestationType,
    makotoLevel,
    ...options,
  };
}

/**
 * Helper function to create a transformation descriptor.
 */
export function createTransformation(
  order: number,
  name: string,
  attestationType: string,
  makotoLevel: MakotoLevel,
  inputs: [string, ...string[]],
  outputs: [string, ...string[]],
  options?: {
    description?: string;
    attestationRef?: string;
    transformType?: string;
  }
): Transformation {
  return {
    order,
    name,
    attestationType,
    makotoLevel,
    inputs,
    outputs,
    ...options,
  };
}

/**
 * Helper function to generate a DBOM ID.
 */
export function generateDbomId(organization: string, name: string): string {
  return `urn:dbom:${organization}:${name}`;
}

/**
 * Helper function to create a digest object.
 */
export function createDigest(
  sha256: string,
  options?: {
    sha512?: string;
    recordCount?: string | number;
    format?: string;
    sizeBytes?: number;
  }
): Digest {
  return {
    sha256,
    ...options,
  };
}

/**
 * Helper function to calculate the overall Makoto level.
 * The overall level is the minimum level across all sources and transformations.
 */
export function calculateOverallMakotoLevel(
  sources: Source[],
  transformations: Transformation[] = []
): MakotoLevel {
  const levelOrder: Record<MakotoLevel, number> = { L1: 1, L2: 2, L3: 3 };

  const allLevels = [
    ...sources.map((s) => s.makotoLevel),
    ...transformations.map((t) => t.makotoLevel),
  ];

  if (allLevels.length === 0) {
    return "L1";
  }

  const minLevel = allLevels.reduce((min, level) => {
    return levelOrder[level] < levelOrder[min] ? level : min;
  });

  return minLevel;
}
