/**
 * Builder for Makoto Transform Attestations.
 *
 * Transform attestations document data transformations with input/output verification.
 *
 * @example
 * ```ts
 * const attestation = new TransformAttestationBuilder()
 *   .addSubject({
 *     name: "dataset:anonymized_transactions",
 *     digest: { sha256: "output_hash..." }
 *   })
 *   .addInput({
 *     name: "dataset:raw_transactions",
 *     digest: { sha256: "input_hash..." }
 *   })
 *   .withTransform({
 *     type: "https://makoto.dev/transforms/anonymization",
 *     name: "PII Anonymization"
 *   })
 *   .withExecutor({ id: "https://example.com/executors/prod-1" })
 *   .build();
 * ```
 */

import type {
  MakotoTransformAttestation,
  Subject,
  SubjectDigest,
  InputReference,
  InputDigest,
  TransformDefinition,
  CodeReference,
  Executor,
  ExecutionMetadata,
  VerificationInfo,
} from "../generated/transform.js";

export type {
  Subject,
  SubjectDigest,
  InputReference,
  InputDigest,
  TransformDefinition,
  CodeReference,
  Executor,
  ExecutionMetadata,
  VerificationInfo,
};

/**
 * Makoto level options for input attestations.
 */
export const MakotoLevels = ["L1", "L2", "L3"] as const;
export type MakotoLevel = (typeof MakotoLevels)[number];

/**
 * Isolation level options for executors.
 */
export const IsolationLevels = [
  "none",
  "process",
  "container",
  "vm",
  "hardware",
] as const;

/**
 * Builder class for creating Makoto Transform Attestations.
 */
export class TransformAttestationBuilder {
  private subjects: Subject[] = [];
  private inputs: InputReference[] = [];
  private transform?: TransformDefinition;
  private executor?: Executor;
  private metadata?: ExecutionMetadata;
  private verification?: VerificationInfo;

  /**
   * Add an output subject (transformed dataset) to this attestation.
   * At least one subject is required.
   */
  addSubject(subject: Subject): this {
    this.subjects.push(subject);
    return this;
  }

  /**
   * Add multiple output subjects at once.
   */
  addSubjects(subjects: Subject[]): this {
    this.subjects.push(...subjects);
    return this;
  }

  /**
   * Add an input reference (source dataset that was transformed).
   * At least one input is required.
   */
  addInput(input: InputReference): this {
    this.inputs.push(input);
    return this;
  }

  /**
   * Add multiple inputs at once.
   */
  addInputs(inputs: InputReference[]): this {
    this.inputs.push(...inputs);
    return this;
  }

  /**
   * Set the transform definition (required).
   * Describes what transformation was applied.
   */
  withTransform(transform: TransformDefinition): this {
    this.transform = transform;
    return this;
  }

  /**
   * Set the executor information (required).
   * Describes the system that executed the transformation.
   */
  withExecutor(executor: Executor): this {
    this.executor = executor;
    return this;
  }

  /**
   * Set execution metadata (optional).
   * Includes timing, record counts, etc.
   */
  withMetadata(metadata: ExecutionMetadata): this {
    this.metadata = metadata;
    return this;
  }

  /**
   * Set verification information (optional).
   * Describes verification performed during transformation.
   */
  withVerification(verification: VerificationInfo): this {
    this.verification = verification;
    return this;
  }

  /**
   * Build the transform attestation.
   * @throws Error if required fields are missing
   */
  build(): MakotoTransformAttestation {
    if (this.subjects.length === 0) {
      throw new Error("At least one subject (output) is required");
    }
    if (this.inputs.length === 0) {
      throw new Error("At least one input is required");
    }
    if (!this.transform) {
      throw new Error("Transform definition is required");
    }
    if (!this.executor) {
      throw new Error("Executor information is required");
    }

    const attestation: MakotoTransformAttestation = {
      _type: "https://in-toto.io/Statement/v1",
      subject: this.subjects as [Subject, ...Subject[]],
      predicateType: "https://makoto.dev/transform/v1",
      predicate: {
        inputs: this.inputs as [InputReference, ...InputReference[]],
        transform: this.transform,
        executor: this.executor,
        ...(this.metadata && { metadata: this.metadata }),
        ...(this.verification && { verification: this.verification }),
      },
    };

    return attestation;
  }

  /**
   * Reset the builder to its initial state.
   */
  reset(): this {
    this.subjects = [];
    this.inputs = [];
    this.transform = undefined;
    this.executor = undefined;
    this.metadata = undefined;
    this.verification = undefined;
    return this;
  }
}

/**
 * Helper function to create an output subject with minimal required fields.
 */
export function createTransformSubject(
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
 * Helper function to create an input reference with minimal required fields.
 */
export function createInputReference(
  name: string,
  sha256: string,
  options?: {
    sha512?: string;
    attestationRef?: string;
    makotoLevel?: MakotoLevel;
  }
): InputReference {
  const { attestationRef, makotoLevel, ...digestOptions } = options ?? {};
  return {
    name,
    digest: {
      sha256,
      ...digestOptions,
    },
    ...(attestationRef && { attestationRef }),
    ...(makotoLevel && { makotoLevel }),
  };
}

/**
 * Helper function to create a transform definition with minimal required fields.
 */
export function createTransformDefinition(
  type: string,
  name: string,
  options?: {
    version?: string;
    description?: string;
    parameters?: Record<string, unknown>;
    codeRef?: CodeReference;
  }
): TransformDefinition {
  return {
    type,
    name,
    ...options,
  };
}

/**
 * Helper function to create executor info with minimal required fields.
 */
export function createExecutor(
  id: string,
  options?: {
    platform?: string;
    version?: Record<string, string>;
    environment?: string;
    isolation?: Executor["isolation"];
  }
): Executor {
  return {
    id,
    ...options,
  };
}

/**
 * Helper function to create execution metadata.
 */
export function createExecutionMetadata(options: {
  invocationId?: string;
  startedOn?: string;
  finishedOn?: string;
  durationSeconds?: number;
  recordsInput?: number;
  recordsOutput?: number;
  recordsDropped?: number;
  recordsModified?: number;
  bytesInput?: number;
  bytesOutput?: number;
}): ExecutionMetadata {
  return options;
}
