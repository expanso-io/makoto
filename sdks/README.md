# Makoto SDKs

This directory contains official Makoto SDKs and the tooling to generate your own.

## Official SDKs

| Language | Package | Status |
|----------|---------|--------|
| [Python](./python/) | `makoto` | Beta |
| [TypeScript](./typescript/) | `@makoto/sdk` | Beta |
| [Go](./go/) | `github.com/expanso-io/makoto/sdks/go` | Beta |
| [Rust](./rust/) | `makoto` | Beta |

## Generate Your Own SDK

All Makoto SDKs are generated from JSON Schemas in `/schemas/makoto.dev/`. You can generate SDKs for any language that has JSON Schema code generation tooling.

### Quick Start

```bash
# Generate all SDKs
make -C sdks/codegen all

# Generate specific language
make -C sdks/codegen python
make -C sdks/codegen typescript
make -C sdks/codegen go
make -C sdks/codegen rust
```

### Generation Process

1. **Schema Source**: All types are defined in `/schemas/makoto.dev/`
2. **Code Generation**: Language-specific tools convert schemas to native types
3. **Manual Extensions**: Signing, verification, and helper functions are added manually
4. **Testing**: Generated code is validated against example attestations

### Supported Languages

See [codegen/README.md](./codegen/README.md) for detailed instructions on generating SDKs for:

- Python (datamodel-codegen)
- TypeScript (json-schema-to-typescript)
- Go (go-jsonschema)
- Rust (typify)
- C# (NJsonSchema)
- Java (jsonschema2pojo)
- And more...

## SDK Architecture

All SDKs follow the same architecture:

```
sdk/
├── models/           # Generated from JSON Schema
│   ├── origin.py     # Origin attestation predicate
│   ├── transform.py  # Transform attestation predicate
│   ├── stream_window.py
│   └── dbom.py
├── attestation.py    # Attestation creation helpers
├── signing.py        # Sigstore/DSSE signing
├── verification.py   # Signature verification
└── merkle.py         # Merkle tree for streaming
```

### Core Functionality

Every SDK provides:

1. **Type-safe models** - Generated from schemas, validated at runtime
2. **Attestation builders** - Fluent API for creating attestations
3. **Signing** - Sigstore keyless signing or bring-your-own-key
4. **Verification** - Signature and hash chain verification
5. **DBOM generation** - Create Data Bills of Materials

### Example Usage (Python)

```python
from makoto import Origin, Transform, Attestation
from makoto.signing import sign_with_sigstore

# Create an origin attestation
origin = Origin(
    source="https://api.example.com/data",
    source_type="api",
    collection_timestamp="2025-01-27T10:00:00Z",
    geography="US-WEST-2"
)

# Build the attestation
attestation = Attestation.origin(
    subject_name="dataset:transactions_2025q1",
    subject_digest={"sha256": "abc123..."},
    predicate=origin
)

# Sign it (Sigstore keyless)
signed = sign_with_sigstore(attestation)

# Write to file
signed.save("attestation.json")
```

## Contributing

See [CONTRIBUTING.md](../CONTRIBUTING.md) for guidelines on contributing to SDKs.

### Adding a New Language

1. Add generation commands to `codegen/Makefile`
2. Create SDK directory with manual extensions
3. Add tests validating against example attestations
4. Update this README and `codegen/README.md`
