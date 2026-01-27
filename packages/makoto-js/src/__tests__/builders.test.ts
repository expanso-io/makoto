/**
 * Tests for Makoto SDK builders.
 */

import { describe, it, expect } from "vitest";
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
  generateDbomId,
} from "../builders/index.js";

describe("OriginAttestationBuilder", () => {
  it("should build a valid origin attestation", () => {
    const attestation = new OriginAttestationBuilder()
      .addSubject(
        createSubject(
          "dataset:transactions_2025q4",
          "a".repeat(64)
        )
      )
      .withOrigin(
        createOrigin(
          "https://api.partner.com/transactions",
          "api",
          "pull",
          "2025-01-01T00:00:00Z"
        )
      )
      .withCollector(createCollector("https://example.com/collectors/prod-1"))
      .build();

    expect(attestation._type).toBe("https://in-toto.io/Statement/v1");
    expect(attestation.predicateType).toBe("https://makoto.dev/origin/v1");
    expect(attestation.subject).toHaveLength(1);
    expect(attestation.subject[0].name).toBe("dataset:transactions_2025q4");
    expect(attestation.predicate.origin.sourceType).toBe("api");
    expect(attestation.predicate.collector.id).toBe(
      "https://example.com/collectors/prod-1"
    );
  });

  it("should throw if no subject is added", () => {
    const builder = new OriginAttestationBuilder()
      .withOrigin(
        createOrigin(
          "https://api.example.com",
          "api",
          "pull"
        )
      )
      .withCollector(createCollector("https://example.com/collectors/1"));

    expect(() => builder.build()).toThrow("At least one subject is required");
  });

  it("should throw if origin is missing", () => {
    const builder = new OriginAttestationBuilder()
      .addSubject(createSubject("dataset:test", "a".repeat(64)))
      .withCollector(createCollector("https://example.com/collectors/1"));

    expect(() => builder.build()).toThrow("Origin information is required");
  });

  it("should throw if collector is missing", () => {
    const builder = new OriginAttestationBuilder()
      .addSubject(createSubject("dataset:test", "a".repeat(64)))
      .withOrigin(createOrigin("https://api.example.com", "api", "pull"));

    expect(() => builder.build()).toThrow("Collector information is required");
  });

  it("should allow multiple subjects", () => {
    const attestation = new OriginAttestationBuilder()
      .addSubject(createSubject("dataset:part1", "a".repeat(64)))
      .addSubject(createSubject("dataset:part2", "b".repeat(64)))
      .withOrigin(createOrigin("https://api.example.com", "api", "pull"))
      .withCollector(createCollector("https://example.com/collectors/1"))
      .build();

    expect(attestation.subject).toHaveLength(2);
  });

  it("should support optional schema and metadata", () => {
    const attestation = new OriginAttestationBuilder()
      .addSubject(createSubject("dataset:test", "a".repeat(64)))
      .withOrigin(createOrigin("https://api.example.com", "api", "pull"))
      .withCollector(createCollector("https://example.com/collectors/1"))
      .withSchema({ format: "json-lines" })
      .withMetadata({ recordsCollected: 1000, bytesCollected: 50000 })
      .build();

    expect(attestation.predicate.schema?.format).toBe("json-lines");
    expect(attestation.predicate.metadata?.recordsCollected).toBe(1000);
  });

  it("should reset the builder", () => {
    const builder = new OriginAttestationBuilder()
      .addSubject(createSubject("dataset:test", "a".repeat(64)))
      .withOrigin(createOrigin("https://api.example.com", "api", "pull"))
      .withCollector(createCollector("https://example.com/collectors/1"));

    builder.reset();

    expect(() => builder.build()).toThrow("At least one subject is required");
  });
});

describe("TransformAttestationBuilder", () => {
  it("should build a valid transform attestation", () => {
    const attestation = new TransformAttestationBuilder()
      .addSubject(createTransformSubject("dataset:output", "a".repeat(64)))
      .addInput(createInputReference("dataset:input", "b".repeat(64)))
      .withTransform(
        createTransformDefinition(
          "https://makoto.dev/transforms/anonymization",
          "PII Anonymization"
        )
      )
      .withExecutor(createExecutor("https://example.com/executors/prod-1"))
      .build();

    expect(attestation._type).toBe("https://in-toto.io/Statement/v1");
    expect(attestation.predicateType).toBe("https://makoto.dev/transform/v1");
    expect(attestation.subject).toHaveLength(1);
    expect(attestation.predicate.inputs).toHaveLength(1);
    expect(attestation.predicate.transform.name).toBe("PII Anonymization");
  });

  it("should throw if no output subject is added", () => {
    const builder = new TransformAttestationBuilder()
      .addInput(createInputReference("dataset:input", "a".repeat(64)))
      .withTransform(
        createTransformDefinition("https://example.com/t", "Test")
      )
      .withExecutor(createExecutor("https://example.com/executors/1"));

    expect(() => builder.build()).toThrow(
      "At least one subject (output) is required"
    );
  });

  it("should support input with attestation reference", () => {
    const attestation = new TransformAttestationBuilder()
      .addSubject(createTransformSubject("dataset:output", "a".repeat(64)))
      .addInput(
        createInputReference("dataset:input", "b".repeat(64), {
          attestationRef: "https://example.com/attestations/123",
          makotoLevel: "L2",
        })
      )
      .withTransform(
        createTransformDefinition("https://example.com/t", "Test")
      )
      .withExecutor(createExecutor("https://example.com/executors/1"))
      .build();

    expect(attestation.predicate.inputs[0].attestationRef).toBe(
      "https://example.com/attestations/123"
    );
    expect(attestation.predicate.inputs[0].makotoLevel).toBe("L2");
  });
});

describe("StreamWindowPredicateBuilder", () => {
  it("should build a valid tumbling window predicate", () => {
    const predicate = new StreamWindowPredicateBuilder()
      .withStream(createStreamDescriptor("iot_sensors"))
      .withWindow(createTumblingWindow("PT1M"))
      .withIntegrity(
        createIntegrity(createMerkleTree("sha256", 1000, "a".repeat(64)))
      )
      .build();

    expect(predicate.stream.id).toBe("iot_sensors");
    expect(predicate.window.type).toBe("tumbling");
    expect(predicate.window.duration).toBe("PT1M");
    expect(predicate.integrity.merkleTree.leafCount).toBe(1000);
  });

  it("should throw if sliding window is missing slide", () => {
    const builder = new StreamWindowPredicateBuilder()
      .withStream(createStreamDescriptor("test"))
      .withWindow({ type: "sliding", duration: "PT1M" }) // Missing slide
      .withIntegrity(
        createIntegrity(createMerkleTree("sha256", 100, "a".repeat(64)))
      );

    expect(() => builder.build()).toThrow(
      "Sliding windows require a slide interval"
    );
  });

  it("should support chain linking", () => {
    const predicate = new StreamWindowPredicateBuilder()
      .withStream(createStreamDescriptor("test"))
      .withWindow(createTumblingWindow("PT1M"))
      .withIntegrity(
        createIntegrity(createMerkleTree("sha256", 100, "a".repeat(64)), {
          previousWindowId: "stream:test:window_20250101_000000",
          previousMerkleRoot: "b".repeat(64),
          chainLength: 2,
        })
      )
      .build();

    expect(predicate.integrity.chain?.chainLength).toBe(2);
    expect(predicate.integrity.chain?.previousWindowId).toBe(
      "stream:test:window_20250101_000000"
    );
  });
});

describe("DBOMBuilder", () => {
  it("should build a valid DBOM", () => {
    const dbom = new DBOMBuilder()
      .withId(generateDbomId("example.com", "fraud-detection-v3"))
      .withDataset(
        createDataset(
          "Fraud Detection Training Data",
          "3.0.0",
          createDigest("a".repeat(64), { recordCount: 1000000 }),
          "L2"
        )
      )
      .addSource(
        createSource(
          "raw_transactions",
          "https://makoto.dev/origin/v1",
          "L2"
        )
      )
      .build();

    expect(dbom.dbomVersion).toBe("1.0.0");
    expect(dbom.dbomId).toBe("urn:dbom:example.com:fraud-detection-v3");
    expect(dbom.dataset.makotoLevel).toBe("L2");
    expect(dbom.sources).toHaveLength(1);
  });

  it("should throw if no sources are added", () => {
    const builder = new DBOMBuilder()
      .withId("urn:dbom:test:test")
      .withDataset(
        createDataset("Test", "1.0.0", createDigest("a".repeat(64)), "L1")
      );

    expect(() => builder.build()).toThrow("At least one source is required");
  });

  it("should sort transformations by order", () => {
    const dbom = new DBOMBuilder()
      .withId("urn:dbom:test:test")
      .withDataset(
        createDataset("Test", "1.0.0", createDigest("a".repeat(64)), "L1")
      )
      .addSource(createSource("src", "https://makoto.dev/origin/v1", "L1"))
      .addTransformation({
        order: 3,
        name: "Third",
        attestationType: "https://makoto.dev/transform/v1",
        makotoLevel: "L1",
        inputs: ["second"],
        outputs: ["third"],
      })
      .addTransformation({
        order: 1,
        name: "First",
        attestationType: "https://makoto.dev/transform/v1",
        makotoLevel: "L1",
        inputs: ["src"],
        outputs: ["first"],
      })
      .addTransformation({
        order: 2,
        name: "Second",
        attestationType: "https://makoto.dev/transform/v1",
        makotoLevel: "L1",
        inputs: ["first"],
        outputs: ["second"],
      })
      .build();

    expect(dbom.transformations?.[0].name).toBe("First");
    expect(dbom.transformations?.[1].name).toBe("Second");
    expect(dbom.transformations?.[2].name).toBe("Third");
  });
});
