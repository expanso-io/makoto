# Makoto Python SDK

Python SDK for [Makoto](https://makoto.dev) data integrity attestations.

Makoto brings SLSA-style assurance levels to data pipelines, producing **DBOMs (Data Bills of Materials)** to prove chain of custody for your data.

## Installation

```bash
pip install makoto
```

For development with signing support:

```bash
pip install makoto[dev]
```

## Quick Start

### Creating an Origin Attestation

```python
from makoto import AttestationBuilder

builder = AttestationBuilder()
statement = (
    builder
    .origin(
        source="https://api.example.com/v2/transactions",
        collector_id="my-collector-01",
        source_type="api",
        collection_method="scheduled-pull",
        geography="US-WEST-2",
    )
    .with_subject_file("data.csv")
    .build()
)

# Save the unsigned attestation
print(statement.to_json())
```

### Creating a Transform Attestation

```python
from makoto import AttestationBuilder

builder = AttestationBuilder()
statement = (
    builder
    .transform(
        transform_type="https://makoto.dev/transforms/anonymization",
        transform_name="PII Anonymization",
        executor_id="pipeline-cluster-01",
        platform="expanso",
    )
    .with_input_file("raw_data.csv")  # Automatically computes hash
    .with_subject_file("anonymized_data.csv")
    .build()
)
```

### Signing Attestations (L2)

```python
from makoto import AttestationBuilder, AttestationSigner

# Create the attestation
builder = AttestationBuilder()
statement = (
    builder
    .origin(source="https://api.example.com/data", collector_id="my-collector")
    .with_subject_file("data.csv")
    .build()
)

# Sign with sigstore (keyless OIDC)
signer = AttestationSigner()
result = signer.sign(statement)

if result.success:
    print(f"Signed by: {result.identity}")
    result.signed_attestation.save("attestation.sigstore.json")
else:
    print(f"Signing failed: {result.error}")
```

### Verifying Attestations

```python
from makoto import AttestationVerifier, InTotoStatement
from pathlib import Path

# Load and verify
statement = InTotoStatement.from_json(Path("attestation.json").read_text())
verifier = AttestationVerifier()

# Structure verification
result = verifier.verify(statement)
print(f"Valid: {result.valid}, Level: {result.makoto_level}")

# Verify with file hash checking
result = verifier.verify_with_files(
    statement,
    files={"data.csv": Path("./data.csv")},
)
print(f"Subjects verified: {result.subjects_verified}/{result.subjects_total}")
```

## Makoto Levels

| Level | Guarantee | What it means |
|-------|-----------|---------------|
| **L1** | Provenance Exists | Machine-readable attestation documents data origin and processing |
| **L2** | Provenance is Authentic | Cryptographically signed, tamper-evident attestations |
| **L3** | Provenance is Unforgeable | Platform-generated attestations from isolated signing infrastructure |

## Predicate Types

The SDK supports three Makoto predicate types:

### Origin (`https://makoto.dev/origin/v1`)

Documents where data was collected:

```python
from makoto.models import OriginPredicate, Origin, Collector
from datetime import datetime, UTC

predicate = OriginPredicate(
    origin=Origin(
        source="https://api.example.com/v2/transactions",
        source_type="api",
        collection_method="scheduled-pull",
        collection_timestamp=datetime.now(UTC),
        geography="US-WEST-2",
    ),
    collector=Collector(
        id="https://expanso.io/collectors/prod-01",
        environment="production",
    ),
)
```

### Transform (`https://makoto.dev/transform/v1`)

Documents how data was processed:

```python
from makoto.models import TransformPredicate, Transform, TransformInput, Executor, DigestSet

predicate = TransformPredicate(
    inputs=[
        TransformInput(
            name="raw_transactions",
            digest=DigestSet(sha256="abc123..."),
            makoto_level="L2",
        )
    ],
    transform=Transform(
        type="https://makoto.dev/transforms/anonymization",
        name="PII Anonymization",
        version="1.0.0",
    ),
    executor=Executor(
        id="https://expanso.io/pipelines/cluster-01",
        platform="expanso",
        environment="production",
    ),
)
```

### Stream Window (`https://makoto.dev/stream-window/v1`)

Documents bounded subsets of streaming data:

```python
from makoto.models import (
    StreamWindowPredicate, Stream, Window, Integrity,
    MerkleTree, StreamCollector
)

predicate = StreamWindowPredicate(
    stream=Stream(
        id="iot_sensors",
        source="mqtt://sensors.example.com:1883",
        topic="sensors/+/readings",
    ),
    window=Window(
        type="tumbling",
        duration="PT1M",
        alignment="wall-clock",
    ),
    integrity=Integrity(
        merkle_tree=MerkleTree(
            algorithm="sha256",
            leaf_count=847293,
            root="abc123def456...",
        )
    ),
    collector=StreamCollector(id="edge-collector-01"),
)
```

## Working with DBOMs

A DBOM (Data Bill of Materials) documents complete data lineage:

```python
from makoto.models import DBOM, Dataset, DatasetDigest, Source

dbom = DBOM(
    dbom_id="urn:uuid:550e8400-e29b-41d4-a716-446655440000",
    dataset=Dataset(
        name="customer_analytics_v2",
        version="2.0.0",
        digest=DatasetDigest(sha256="abc123..."),
        makoto_level="L2",
    ),
    sources=[
        Source(
            name="crm_export",
            attestation_ref="https://registry.example.com/att/origin-123",
            makoto_level="L2",
        ),
        Source(
            name="web_analytics",
            attestation_ref="https://registry.example.com/att/origin-456",
            makoto_level="L1",
        ),
    ],
)
```

## Development

### Setup

```bash
git clone https://github.com/expanso-io/makoto
cd makoto/sdks/python
uv venv
uv pip install -e ".[dev]"
```

### Running Tests

```bash
uv run pytest
```

### Type Checking

```bash
uv run mypy src/makoto
```

### Linting

```bash
uv run ruff check src/ tests/
uv run ruff format src/ tests/
```

## API Reference

### AttestationBuilder

Fluent builder for creating attestations.

- `origin(source, collector_id, ...)` - Configure an origin attestation
- `transform(transform_type, transform_name, executor_id, ...)` - Configure a transform attestation
- `stream_window(stream_id, stream_source, ...)` - Configure a stream window attestation
- `with_input(name, sha256, ...)` - Add an input (transform only)
- `with_input_file(path, ...)` - Add an input from a file
- `with_subject(name, sha256)` - Add a subject
- `with_subject_file(path, ...)` - Add a subject from a file
- `build()` - Build the attestation statement
- `reset()` - Reset the builder for reuse

### AttestationVerifier

Verify attestation structure and integrity.

- `verify(statement)` - Verify attestation structure
- `verify_with_files(statement, files)` - Verify with file hash checking
- `verify_chain(statements)` - Verify a chain of attestations

### AttestationSigner

Sign attestations using sigstore.

- `sign(statement)` - Sign with sigstore (keyless OIDC)
- `sign_with_key(statement, private_key_pem, key_id)` - Sign with a private key
- `verify(signed_attestation)` - Verify a signed attestation

## License

Apache-2.0
