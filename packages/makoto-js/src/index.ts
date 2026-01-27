/**
 * Makoto SDK for TypeScript
 *
 * TypeScript SDK for creating and validating Makoto data integrity attestations.
 * Makoto brings SLSA-style assurance levels to data pipelines, producing DBOMs
 * (Data Bills of Materials) to prove chain of custody for your data.
 *
 * @packageDocumentation
 *
 * @example
 * ```ts
 * import {
 *   OriginAttestationBuilder,
 *   validateOriginAttestation,
 *   createSubject,
 *   createOrigin,
 *   createCollector,
 * } from "@makoto/sdk";
 *
 * // Build an origin attestation
 * const attestation = new OriginAttestationBuilder()
 *   .addSubject(createSubject("dataset:transactions_2025q4", "abc123..."))
 *   .withOrigin(createOrigin(
 *     "https://api.partner.com/transactions",
 *     "api",
 *     "pull"
 *   ))
 *   .withCollector(createCollector("https://example.com/collectors/prod-1"))
 *   .build();
 *
 * // Validate any attestation
 * const result = validateOriginAttestation(attestation);
 * if (result.valid) {
 *   console.log("Valid attestation:", result.data);
 * }
 * ```
 */

// Re-export all generated types
export * from "./generated/index.js";

// Re-export all builders
export * from "./builders/index.js";

// Re-export all validation functions
export * from "./validation/index.js";

// Re-export schemas for advanced use cases
export * as schemas from "./schemas/index.js";
