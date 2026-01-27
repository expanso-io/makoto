/**
 * Makoto SDK Validation
 *
 * Runtime validation of Makoto attestations and DBOMs against JSON schemas.
 * Uses AJV for high-performance JSON Schema validation.
 */

import Ajv2020 from "ajv/dist/2020.js";
import addFormats from "ajv-formats";
import type { MakotoOriginAttestation } from "../generated/origin.js";
import type { MakotoTransformAttestation } from "../generated/transform.js";
import type { MakotoStreamWindowPredicate } from "../generated/stream-window.js";
import type { DataBillOfMaterialsDBOM } from "../generated/dbom.js";

// Import schemas from generated schema modules
import {
  originSchema,
  transformSchema,
  streamWindowSchema,
  dbomSchema,
} from "../schemas/index.js";

/**
 * Validation error details.
 */
export interface ValidationError {
  /** JSON pointer to the property that failed validation */
  path: string;
  /** Human-readable error message */
  message: string;
  /** The value that failed validation */
  value?: unknown;
  /** Additional params from the JSON Schema keyword */
  params?: Record<string, unknown>;
}

/**
 * Validation result.
 */
export interface ValidationResult<T> {
  /** Whether the data is valid */
  valid: boolean;
  /** The validated data (only if valid) */
  data?: T;
  /** Validation errors (only if invalid) */
  errors?: ValidationError[];
}

// Initialize AJV with JSON Schema 2020-12 support and formats
const ajv = new Ajv2020({
  allErrors: true,
  verbose: true,
  strict: false, // Allow additional JSON Schema features
});
addFormats(ajv);

// Compile validators
const validateOriginSchema = ajv.compile(originSchema);
const validateTransformSchema = ajv.compile(transformSchema);
const validateStreamWindowSchema = ajv.compile(streamWindowSchema);
const validateDbomSchema = ajv.compile(dbomSchema);

/**
 * Convert AJV errors to our ValidationError format.
 */
function convertErrors(
  errors: typeof validateOriginSchema.errors
): ValidationError[] {
  if (!errors) return [];

  return errors.map((err) => ({
    path: err.instancePath || "/",
    message: err.message || "Unknown validation error",
    value: err.data,
    params: err.params as Record<string, unknown>,
  }));
}

/**
 * Validate a Makoto Origin Attestation.
 *
 * @example
 * ```ts
 * const result = validateOriginAttestation(data);
 * if (result.valid) {
 *   console.log("Valid origin attestation:", result.data);
 * } else {
 *   console.error("Validation errors:", result.errors);
 * }
 * ```
 */
export function validateOriginAttestation(
  data: unknown
): ValidationResult<MakotoOriginAttestation> {
  const valid = validateOriginSchema(data);

  if (valid) {
    return { valid: true, data: data as MakotoOriginAttestation };
  }

  return {
    valid: false,
    errors: convertErrors(validateOriginSchema.errors),
  };
}

/**
 * Type guard for Origin Attestation.
 */
export function isOriginAttestation(
  data: unknown
): data is MakotoOriginAttestation {
  return validateOriginSchema(data);
}

/**
 * Validate a Makoto Transform Attestation.
 *
 * @example
 * ```ts
 * const result = validateTransformAttestation(data);
 * if (result.valid) {
 *   console.log("Valid transform attestation:", result.data);
 * } else {
 *   console.error("Validation errors:", result.errors);
 * }
 * ```
 */
export function validateTransformAttestation(
  data: unknown
): ValidationResult<MakotoTransformAttestation> {
  const valid = validateTransformSchema(data);

  if (valid) {
    return { valid: true, data: data as MakotoTransformAttestation };
  }

  return {
    valid: false,
    errors: convertErrors(validateTransformSchema.errors),
  };
}

/**
 * Type guard for Transform Attestation.
 */
export function isTransformAttestation(
  data: unknown
): data is MakotoTransformAttestation {
  return validateTransformSchema(data);
}

/**
 * Validate a Makoto Stream Window Predicate.
 *
 * @example
 * ```ts
 * const result = validateStreamWindowPredicate(data);
 * if (result.valid) {
 *   console.log("Valid stream window predicate:", result.data);
 * } else {
 *   console.error("Validation errors:", result.errors);
 * }
 * ```
 */
export function validateStreamWindowPredicate(
  data: unknown
): ValidationResult<MakotoStreamWindowPredicate> {
  const valid = validateStreamWindowSchema(data);

  if (valid) {
    return { valid: true, data: data as MakotoStreamWindowPredicate };
  }

  return {
    valid: false,
    errors: convertErrors(validateStreamWindowSchema.errors),
  };
}

/**
 * Type guard for Stream Window Predicate.
 */
export function isStreamWindowPredicate(
  data: unknown
): data is MakotoStreamWindowPredicate {
  return validateStreamWindowSchema(data);
}

/**
 * Validate a Data Bill of Materials (DBOM).
 *
 * @example
 * ```ts
 * const result = validateDBOM(data);
 * if (result.valid) {
 *   console.log("Valid DBOM:", result.data);
 * } else {
 *   console.error("Validation errors:", result.errors);
 * }
 * ```
 */
export function validateDBOM(
  data: unknown
): ValidationResult<DataBillOfMaterialsDBOM> {
  const valid = validateDbomSchema(data);

  if (valid) {
    return { valid: true, data: data as DataBillOfMaterialsDBOM };
  }

  return {
    valid: false,
    errors: convertErrors(validateDbomSchema.errors),
  };
}

/**
 * Type guard for DBOM.
 */
export function isDBOM(data: unknown): data is DataBillOfMaterialsDBOM {
  return validateDbomSchema(data);
}

/**
 * Attestation type detection result.
 */
export type AttestationType =
  | "origin"
  | "transform"
  | "stream-window"
  | "dbom"
  | "unknown";

/**
 * Detect the type of a Makoto document.
 *
 * @example
 * ```ts
 * const type = detectAttestationType(data);
 * switch (type) {
 *   case "origin":
 *     // Handle origin attestation
 *     break;
 *   case "transform":
 *     // Handle transform attestation
 *     break;
 *   // ...
 * }
 * ```
 */
export function detectAttestationType(data: unknown): AttestationType {
  if (typeof data !== "object" || data === null) {
    return "unknown";
  }

  const obj = data as Record<string, unknown>;

  // Check for DBOM (has dbomVersion and dbomId)
  if ("dbomVersion" in obj && "dbomId" in obj) {
    return "dbom";
  }

  // Check for in-toto Statement types
  if (obj._type === "https://in-toto.io/Statement/v1") {
    const predicateType = obj.predicateType as string;

    if (predicateType === "https://makoto.dev/origin/v1") {
      return "origin";
    }
    if (predicateType === "https://makoto.dev/transform/v1") {
      return "transform";
    }
  }

  // Check for stream-window predicate (has stream, window, integrity)
  if ("stream" in obj && "window" in obj && "integrity" in obj) {
    return "stream-window";
  }

  return "unknown";
}

/**
 * Auto-detect and validate a Makoto document.
 *
 * @example
 * ```ts
 * const result = validateAuto(data);
 * if (result.valid) {
 *   console.log(`Valid ${result.type}:`, result.data);
 * } else {
 *   console.error("Validation errors:", result.errors);
 * }
 * ```
 */
export function validateAuto(
  data: unknown
): ValidationResult<unknown> & { type: AttestationType } {
  const type = detectAttestationType(data);

  switch (type) {
    case "origin":
      return { ...validateOriginAttestation(data), type };
    case "transform":
      return { ...validateTransformAttestation(data), type };
    case "stream-window":
      return { ...validateStreamWindowPredicate(data), type };
    case "dbom":
      return { ...validateDBOM(data), type };
    default:
      return {
        valid: false,
        type: "unknown",
        errors: [
          {
            path: "/",
            message: "Unable to detect Makoto document type",
          },
        ],
      };
  }
}
