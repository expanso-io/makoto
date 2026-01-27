# @makoto/sdk

TypeScript SDK for creating and validating [Makoto](https://usemakoto.dev) data integrity attestations.

Makoto brings SLSA-style assurance levels to data pipelines, producing **DBOMs (Data Bills of Materials)** to prove chain of custody for your data.

## Installation

```bash
npm install @makoto/sdk
```

## Features

- **Type-safe builders** for creating Makoto attestations
- **Runtime validation** against JSON schemas using AJV
- **Auto-generated types** from official Makoto JSON schemas
- **Type guards** for narrowing unknown data to specific attestation types

## Quick Start

```typescript
import {
  OriginAttestationBuilder,
  validateOriginAttestation,
  createSubject,
  createOrigin,
  createCollector,
} from "@makoto/sdk";

// Build an origin attestation using the fluent builder API
const attestation = new OriginAttestationBuilder()
  .addSubject(
    createSubject(
      "dataset:customer_transactions_2025q4",
      "abc123def456..." // SHA-256 hash (64 hex chars)
    )
  )
  .withOrigin(
    createOrigin(
      "https://api.partner.com/transactions",
      "api",
      "pull",
      new Date().toISOString()
    )
  )
  .withCollector(createCollector("https://example.com/collectors/prod-1"))
  .build();

// Validate the attestation
const result = validateOriginAttestation(attestation);
if (result.valid) {
  console.log("Valid attestation:", result.data);
} else {
  console.error("Validation errors:", result.errors);
}
```

## Attestation Types

### Origin Attestation

Documents data provenance at the point of collection.

```typescript
import {
  OriginAttestationBuilder,
  createSubject,
  createOrigin,
  createCollector,
} from "@makoto/sdk";

const originAttestation = new OriginAttestationBuilder()
  .addSubject(createSubject("dataset:raw_transactions", "sha256-hash..."))
  .withOrigin(
    createOrigin(
      "https://api.bank.com/v2/transactions",
      "api",
      "scheduled-pull",
      "2025-01-01T00:00:00Z",
      {
        geography: "US-WEST-2",
        consent: {
          type: "contractual",
          reference: "https://example.com/dpa/partner-bank",
        },
      }
    )
  )
  .withCollector(
    createCollector("https://example.com/collectors/prod-west-1", {
      version: { "expanso-cli": "1.4.2" },
      environment: "production",
      platform: "expanso",
    })
  )
  .withSchema({ format: "json-lines" })
  .withMetadata({
    recordsCollected: 1000000,
    bytesCollected: 50000000,
  })
  .build();
```

### Transform Attestation

Documents data transformations with input/output verification.

```typescript
import {
  TransformAttestationBuilder,
  createTransformSubject,
  createInputReference,
  createTransformDefinition,
  createExecutor,
} from "@makoto/sdk";

const transformAttestation = new TransformAttestationBuilder()
  .addSubject(createTransformSubject("dataset:anonymized_data", "output-hash..."))
  .addInput(
    createInputReference("dataset:raw_data", "input-hash...", {
      attestationRef: "https://registry.example.com/attestations/123",
      makotoLevel: "L2",
    })
  )
  .withTransform(
    createTransformDefinition(
      "https://makoto.dev/transforms/anonymization",
      "PII Anonymization",
      {
        version: "1.0.0",
        description: "Removes personally identifiable information",
        parameters: { kAnonymity: 5 },
      }
    )
  )
  .withExecutor(
    createExecutor("https://example.com/executors/prod-1", {
      platform: "expanso",
      isolation: "container",
    })
  )
  .build();
```

### Stream Window Predicate

Captures integrity attestations for bounded windows of streaming data.

```typescript
import {
  StreamWindowPredicateBuilder,
  createStreamDescriptor,
  createTumblingWindow,
  createMerkleTree,
  createIntegrity,
} from "@makoto/sdk";

const streamPredicate = new StreamWindowPredicateBuilder()
  .withStream(
    createStreamDescriptor("iot_sensors", {
      source: "kafka://broker:9092",
      topic: "sensor-readings",
      partitions: ["0", "1", "2"],
    })
  )
  .withWindow(
    createTumblingWindow("PT1M", {
      alignment: "event-time",
      allowedLateness: "PT30S",
    })
  )
  .withIntegrity(
    createIntegrity(
      createMerkleTree("sha256", 10000, "merkle-root-hash...", {
        treeHeight: 14,
      }),
      {
        previousWindowId: "stream:iot_sensors:window_20250101_120000",
        previousMerkleRoot: "previous-merkle-root...",
        chainLength: 42,
      }
    )
  )
  .withCollector({
    id: "https://example.com/collectors/edge-1",
    location: "factory-floor-a",
  })
  .build();
```

### Data Bill of Materials (DBOM)

Comprehensive manifest documenting dataset provenance, lineage, and compliance.

```typescript
import {
  DBOMBuilder,
  createDataset,
  createSource,
  createTransformation,
  createDigest,
  generateDbomId,
  calculateOverallMakotoLevel,
} from "@makoto/sdk";

const sources = [
  createSource("bank_transactions", "https://makoto.dev/origin/v1", "L2", {
    geography: "US",
  }),
  createSource("customer_profiles", "https://makoto.dev/origin/v1", "L2", {
    geography: "US",
  }),
];

const transformations = [
  createTransformation(
    1,
    "Join Datasets",
    "https://makoto.dev/transform/v1",
    "L2",
    ["bank_transactions", "customer_profiles"],
    ["joined_data"]
  ),
  createTransformation(
    2,
    "Anonymize PII",
    "https://makoto.dev/transform/v1",
    "L2",
    ["joined_data"],
    ["training_data"]
  ),
];

const dbom = new DBOMBuilder()
  .withId(generateDbomId("example.com", "fraud-detection-training-v3"))
  .withDataset(
    createDataset(
      "Fraud Detection Training Data",
      "3.0.0",
      createDigest("final-hash...", {
        recordCount: 1000000,
        format: "parquet",
        sizeBytes: 500000000,
      }),
      calculateOverallMakotoLevel(sources, transformations)
    )
  )
  .addSources(sources)
  .addTransformations(transformations)
  .withCompliance({
    overallMakotoLevel: "L2",
    levelJustification: "All sources and transformations are L2 compliant",
    privacyAssessment: {
      piiRemoved: true,
      anonymizationVerified: true,
      kAnonymity: 5,
    },
    regulatoryCompliance: [
      {
        regulation: "EU AI Act Article 10",
        status: "compliant",
        assessmentDate: "2025-01-01",
      },
    ],
  })
  .build();
```

## Validation

### Validate Specific Types

```typescript
import {
  validateOriginAttestation,
  validateTransformAttestation,
  validateStreamWindowPredicate,
  validateDBOM,
} from "@makoto/sdk";

// Each returns { valid: boolean, data?: T, errors?: ValidationError[] }
const originResult = validateOriginAttestation(unknownData);
const transformResult = validateTransformAttestation(unknownData);
const streamResult = validateStreamWindowPredicate(unknownData);
const dbomResult = validateDBOM(unknownData);
```

### Auto-Detect and Validate

```typescript
import { validateAuto, detectAttestationType } from "@makoto/sdk";

// Detect type without validation
const type = detectAttestationType(data);
// Returns: "origin" | "transform" | "stream-window" | "dbom" | "unknown"

// Auto-detect and validate
const result = validateAuto(data);
if (result.valid) {
  console.log(`Valid ${result.type}:`, result.data);
}
```

### Type Guards

```typescript
import {
  isOriginAttestation,
  isTransformAttestation,
  isStreamWindowPredicate,
  isDBOM,
} from "@makoto/sdk";

// Type guards for type narrowing
if (isOriginAttestation(data)) {
  // TypeScript knows data is MakotoOriginAttestation
  console.log(data.predicate.origin.sourceType);
}
```

## API Reference

### Builders

| Builder | Creates |
|---------|---------|
| `OriginAttestationBuilder` | Origin attestations (data collection) |
| `TransformAttestationBuilder` | Transform attestations (data transformation) |
| `StreamWindowPredicateBuilder` | Stream window predicates (streaming data) |
| `DBOMBuilder` | Data Bill of Materials |

### Helper Functions

| Function | Purpose |
|----------|---------|
| `createSubject(name, sha256, options?)` | Create attestation subject |
| `createOrigin(source, sourceType, method, timestamp?, options?)` | Create origin info |
| `createCollector(id, options?)` | Create collector info |
| `createInputReference(name, sha256, options?)` | Create transform input ref |
| `createTransformDefinition(type, name, options?)` | Create transform definition |
| `createExecutor(id, options?)` | Create executor info |
| `createStreamDescriptor(id, options?)` | Create stream descriptor |
| `createTumblingWindow(duration, options?)` | Create tumbling window |
| `createSlidingWindow(duration, slide, options?)` | Create sliding window |
| `createSessionWindow(gap, options?)` | Create session window |
| `createMerkleTree(algorithm, leafCount, root, options?)` | Create Merkle tree info |
| `createIntegrity(merkleTree, chain?)` | Create integrity descriptor |
| `createDataset(name, version, digest, level, options?)` | Create DBOM dataset |
| `createSource(name, attestationType, level, options?)` | Create DBOM source |
| `createTransformation(order, name, type, level, inputs, outputs, options?)` | Create DBOM transformation |
| `createDigest(sha256, options?)` | Create digest object |
| `generateDbomId(organization, name)` | Generate DBOM URN |
| `calculateOverallMakotoLevel(sources, transforms?)` | Calculate minimum level |

### Validation Functions

| Function | Purpose |
|----------|---------|
| `validateOriginAttestation(data)` | Validate origin attestation |
| `validateTransformAttestation(data)` | Validate transform attestation |
| `validateStreamWindowPredicate(data)` | Validate stream window predicate |
| `validateDBOM(data)` | Validate DBOM |
| `validateAuto(data)` | Auto-detect and validate |
| `detectAttestationType(data)` | Detect attestation type |
| `isOriginAttestation(data)` | Type guard for origin |
| `isTransformAttestation(data)` | Type guard for transform |
| `isStreamWindowPredicate(data)` | Type guard for stream window |
| `isDBOM(data)` | Type guard for DBOM |

## Types

All types are exported from the package:

```typescript
import type {
  // Main attestation types
  MakotoOriginAttestation,
  MakotoTransformAttestation,
  MakotoStreamWindowPredicate,
  DataBillOfMaterialsDBOM,
  MakotoLevel,

  // Validation types
  ValidationResult,
  ValidationError,
  AttestationType,
} from "@makoto/sdk";
```

For detailed type definitions, see the [generated types](./src/generated/).

## Development

```bash
# Install dependencies
npm install

# Generate types from schemas
npm run generate-types

# Run tests
npm test

# Type check
npm run typecheck

# Build
npm run build
```

## License

Apache 2.0
