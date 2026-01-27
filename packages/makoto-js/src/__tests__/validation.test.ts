/**
 * Tests for Makoto SDK validation.
 */

import { describe, it, expect } from "vitest";
import {
  validateOriginAttestation,
  validateTransformAttestation,
  validateStreamWindowPredicate,
  validateDBOM,
  validateAuto,
  detectAttestationType,
  isOriginAttestation,
  isTransformAttestation,
  isStreamWindowPredicate,
  isDBOM,
} from "../validation/index.js";
import {
  OriginAttestationBuilder,
  TransformAttestationBuilder,
  StreamWindowPredicateBuilder,
  DBOMBuilder,
  createSubject,
  createOrigin,
  createCollector,
  createTransformSubject,
  createInputReference,
  createTransformDefinition,
  createExecutor,
  createStreamDescriptor,
  createTumblingWindow,
  createMerkleTree,
  createIntegrity,
  createDataset,
  createSource,
  createDigest,
} from "../builders/index.js";

describe("validateOriginAttestation", () => {
  it("should validate a valid origin attestation", () => {
    const attestation = new OriginAttestationBuilder()
      .addSubject(createSubject("dataset:test", "a".repeat(64)))
      .withOrigin(
        createOrigin("https://api.example.com", "api", "pull")
      )
      .withCollector(createCollector("https://example.com/collectors/1"))
      .build();

    const result = validateOriginAttestation(attestation);

    expect(result.valid).toBe(true);
    expect(result.data).toEqual(attestation);
    expect(result.errors).toBeUndefined();
  });

  it("should reject an invalid origin attestation", () => {
    const invalid = {
      _type: "https://in-toto.io/Statement/v1",
      predicateType: "https://makoto.dev/origin/v1",
      // Missing subject and predicate
    };

    const result = validateOriginAttestation(invalid);

    expect(result.valid).toBe(false);
    expect(result.errors).toBeDefined();
    expect(result.errors!.length).toBeGreaterThan(0);
  });

  it("should reject invalid subject digest format", () => {
    const invalid = {
      _type: "https://in-toto.io/Statement/v1",
      subject: [
        {
          name: "dataset:test",
          digest: {
            sha256: "invalid-not-64-hex", // Should be 64 hex chars
          },
        },
      ],
      predicateType: "https://makoto.dev/origin/v1",
      predicate: {
        origin: {
          source: "https://api.example.com",
          sourceType: "api",
          collectionMethod: "pull",
          collectionTimestamp: "2025-01-01T00:00:00Z",
        },
        collector: {
          id: "https://example.com/collectors/1",
        },
      },
    };

    const result = validateOriginAttestation(invalid);

    expect(result.valid).toBe(false);
  });
});

describe("validateTransformAttestation", () => {
  it("should validate a valid transform attestation", () => {
    const attestation = new TransformAttestationBuilder()
      .addSubject(createTransformSubject("dataset:output", "a".repeat(64)))
      .addInput(createInputReference("dataset:input", "b".repeat(64)))
      .withTransform(
        createTransformDefinition("https://example.com/t", "Test Transform")
      )
      .withExecutor(createExecutor("https://example.com/executors/1"))
      .build();

    const result = validateTransformAttestation(attestation);

    expect(result.valid).toBe(true);
    expect(result.data).toEqual(attestation);
  });

  it("should reject missing required fields", () => {
    const invalid = {
      _type: "https://in-toto.io/Statement/v1",
      predicateType: "https://makoto.dev/transform/v1",
      subject: [],
      predicate: {},
    };

    const result = validateTransformAttestation(invalid);

    expect(result.valid).toBe(false);
    expect(result.errors).toBeDefined();
  });
});

describe("validateStreamWindowPredicate", () => {
  it("should validate a valid stream window predicate", () => {
    const predicate = new StreamWindowPredicateBuilder()
      .withStream(createStreamDescriptor("test_stream"))
      .withWindow(createTumblingWindow("PT5M"))
      .withIntegrity(
        createIntegrity(createMerkleTree("sha256", 500, "c".repeat(64)))
      )
      .build();

    const result = validateStreamWindowPredicate(predicate);

    expect(result.valid).toBe(true);
    expect(result.data).toEqual(predicate);
  });

  it("should reject invalid window type", () => {
    const invalid = {
      stream: { id: "test" },
      window: {
        type: "invalid-type", // Not tumbling, sliding, or session
        duration: "PT1M",
      },
      integrity: {
        merkleTree: {
          algorithm: "sha256",
          leafCount: 100,
          root: "a".repeat(64),
        },
      },
    };

    const result = validateStreamWindowPredicate(invalid);

    expect(result.valid).toBe(false);
  });
});

describe("validateDBOM", () => {
  it("should validate a valid DBOM", () => {
    const dbom = new DBOMBuilder()
      .withId("urn:dbom:example.com:test-dataset")
      .withDataset(
        createDataset("Test Dataset", "1.0.0", createDigest("a".repeat(64)), "L1")
      )
      .addSource(createSource("source1", "https://makoto.dev/origin/v1", "L1"))
      .build();

    const result = validateDBOM(dbom);

    expect(result.valid).toBe(true);
    expect(result.data).toEqual(dbom);
  });

  it("should reject invalid DBOM ID format", () => {
    const invalid = {
      dbomVersion: "1.0.0",
      dbomId: "invalid-not-urn", // Should start with urn:dbom:
      dataset: {
        name: "Test",
        version: "1.0.0",
        created: "2025-01-01T00:00:00Z",
        digest: { sha256: "a".repeat(64) },
        makotoLevel: "L1",
      },
      sources: [
        {
          name: "source1",
          attestationType: "https://makoto.dev/origin/v1",
          makotoLevel: "L1",
        },
      ],
    };

    const result = validateDBOM(invalid);

    expect(result.valid).toBe(false);
  });
});

describe("detectAttestationType", () => {
  it("should detect origin attestation", () => {
    const attestation = new OriginAttestationBuilder()
      .addSubject(createSubject("dataset:test", "a".repeat(64)))
      .withOrigin(createOrigin("https://example.com", "api", "pull"))
      .withCollector(createCollector("https://example.com/c/1"))
      .build();

    expect(detectAttestationType(attestation)).toBe("origin");
  });

  it("should detect transform attestation", () => {
    const attestation = new TransformAttestationBuilder()
      .addSubject(createTransformSubject("dataset:out", "a".repeat(64)))
      .addInput(createInputReference("dataset:in", "b".repeat(64)))
      .withTransform(createTransformDefinition("https://t.com", "T"))
      .withExecutor(createExecutor("https://example.com/e/1"))
      .build();

    expect(detectAttestationType(attestation)).toBe("transform");
  });

  it("should detect stream window predicate", () => {
    const predicate = new StreamWindowPredicateBuilder()
      .withStream(createStreamDescriptor("test"))
      .withWindow(createTumblingWindow("PT1M"))
      .withIntegrity(
        createIntegrity(createMerkleTree("sha256", 100, "a".repeat(64)))
      )
      .build();

    expect(detectAttestationType(predicate)).toBe("stream-window");
  });

  it("should detect DBOM", () => {
    const dbom = new DBOMBuilder()
      .withId("urn:dbom:test:test")
      .withDataset(
        createDataset("Test", "1.0.0", createDigest("a".repeat(64)), "L1")
      )
      .addSource(createSource("src", "https://makoto.dev/origin/v1", "L1"))
      .build();

    expect(detectAttestationType(dbom)).toBe("dbom");
  });

  it("should return unknown for unrecognized objects", () => {
    expect(detectAttestationType({ foo: "bar" })).toBe("unknown");
    expect(detectAttestationType(null)).toBe("unknown");
    expect(detectAttestationType("string")).toBe("unknown");
  });
});

describe("validateAuto", () => {
  it("should auto-detect and validate origin attestation", () => {
    const attestation = new OriginAttestationBuilder()
      .addSubject(createSubject("dataset:test", "a".repeat(64)))
      .withOrigin(createOrigin("https://example.com", "api", "pull"))
      .withCollector(createCollector("https://example.com/c/1"))
      .build();

    const result = validateAuto(attestation);

    expect(result.valid).toBe(true);
    expect(result.type).toBe("origin");
  });

  it("should return unknown for unrecognized types", () => {
    const result = validateAuto({ random: "data" });

    expect(result.valid).toBe(false);
    expect(result.type).toBe("unknown");
  });
});

describe("Type guards", () => {
  it("isOriginAttestation should work as type guard", () => {
    const attestation = new OriginAttestationBuilder()
      .addSubject(createSubject("dataset:test", "a".repeat(64)))
      .withOrigin(createOrigin("https://example.com", "api", "pull"))
      .withCollector(createCollector("https://example.com/c/1"))
      .build();

    const data: unknown = attestation;

    if (isOriginAttestation(data)) {
      // TypeScript should allow accessing origin-specific properties
      expect(data.predicate.origin.sourceType).toBe("api");
    }
  });

  it("isTransformAttestation should work as type guard", () => {
    const attestation = new TransformAttestationBuilder()
      .addSubject(createTransformSubject("dataset:out", "a".repeat(64)))
      .addInput(createInputReference("dataset:in", "b".repeat(64)))
      .withTransform(createTransformDefinition("https://t.com", "T"))
      .withExecutor(createExecutor("https://example.com/e/1"))
      .build();

    expect(isTransformAttestation(attestation)).toBe(true);
    expect(isTransformAttestation({ random: "data" })).toBe(false);
  });

  it("isStreamWindowPredicate should work as type guard", () => {
    const predicate = new StreamWindowPredicateBuilder()
      .withStream(createStreamDescriptor("test"))
      .withWindow(createTumblingWindow("PT1M"))
      .withIntegrity(
        createIntegrity(createMerkleTree("sha256", 100, "a".repeat(64)))
      )
      .build();

    expect(isStreamWindowPredicate(predicate)).toBe(true);
    expect(isStreamWindowPredicate({ random: "data" })).toBe(false);
  });

  it("isDBOM should work as type guard", () => {
    const dbom = new DBOMBuilder()
      .withId("urn:dbom:test:test")
      .withDataset(
        createDataset("Test", "1.0.0", createDigest("a".repeat(64)), "L1")
      )
      .addSource(createSource("src", "https://makoto.dev/origin/v1", "L1"))
      .build();

    expect(isDBOM(dbom)).toBe(true);
    expect(isDBOM({ random: "data" })).toBe(false);
  });
});
