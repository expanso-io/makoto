# Makoto JSON Schemas

This directory contains JSON Schema definitions for Makoto attestation formats.

## Schema Structure

```
schemas/
├── in-toto/
│   └── statement-v1.json       # in-toto Statement wrapper
├── makoto.dev/
│   ├── common/
│   │   └── definitions.json    # Shared type definitions
│   ├── origin/
│   │   └── v1.json             # Origin attestation predicate
│   ├── transform/
│   │   └── v1.json             # Transform attestation predicate
│   ├── stream-window/
│   │   └── v1.json             # Stream window attestation predicate
│   └── dbom/
│       └── v1.json             # Data Bill of Materials
└── README.md
```

## Predicate Types

| Predicate Type | Schema | Purpose |
|----------------|--------|---------|
| `https://makoto.dev/origin/v1` | `makoto.dev/origin/v1.json` | Documents data collection from external sources |
| `https://makoto.dev/transform/v1` | `makoto.dev/transform/v1.json` | Documents data transformation operations |
| `https://makoto.dev/stream-window/v1` | `makoto.dev/stream-window/v1.json` | Documents bounded windows of streaming data |

## Usage

### Validating Attestations

Using [ajv-cli](https://github.com/ajv-validator/ajv-cli):

```bash
# Validate an origin attestation
ajv validate -s schemas/in-toto/statement-v1.json \
    -r "schemas/makoto.dev/**/*.json" \
    -d docs/attestations/origin-example.json

# Validate a DBOM
ajv validate -s schemas/makoto.dev/dbom/v1.json \
    -r "schemas/makoto.dev/common/*.json" \
    -d docs/attestations/dbom-example.json
```

Using Python with [jsonschema](https://python-jsonschema.readthedocs.io/):

```python
import json
from jsonschema import validate, RefResolver

# Load the schema
with open("schemas/makoto.dev/origin/v1.json") as f:
    schema = json.load(f)

# Create a resolver for $ref resolution
resolver = RefResolver.from_schema(schema, store={
    "https://makoto.dev/schemas/common/definitions.json":
        json.load(open("schemas/makoto.dev/common/definitions.json"))
})

# Validate an attestation
with open("docs/attestations/origin-example.json") as f:
    attestation = json.load(f)

validate(attestation["predicate"], schema, resolver=resolver)
```

### Schema References

The schemas use JSON Schema `$ref` for shared definitions. When validating:

1. **Common definitions** (`makoto.dev/common/definitions.json`) must be loaded as a schema store
2. **Predicate schemas** reference common definitions via URI: `https://makoto.dev/schemas/common/definitions.json#/$defs/...`
3. **The in-toto wrapper** conditionally validates predicates based on `predicateType`

## Schema Design Principles

1. **in-toto compatibility**: All attestations wrap predicates in the in-toto Statement v1 format
2. **Extensibility**: `additionalProperties: true` allows vendor extensions
3. **Progressive adoption**: Most fields are optional to enable L1 (minimal) attestations
4. **Type safety**: Enums for known values, patterns for hashes
5. **Self-documenting**: Descriptions on all properties

## Versioning

Schemas follow semantic versioning:
- **v1.x.x**: Backward-compatible additions
- **v2.0.0**: Breaking changes (new predicate type URI)

Current versions:
- `origin/v1` - Stable
- `transform/v1` - Stable
- `stream-window/v1` - Stable
- `dbom/v1` - Stable

## Contributing

When modifying schemas:

1. Add/update test cases in `docs/attestations/`
2. Run validation against all examples
3. Update this README if adding new schemas
4. Follow [JSON Schema best practices](https://json-schema.org/understanding-json-schema/)

## Related Resources

- [Makoto Specification](https://usemakoto.dev/spec/)
- [in-toto Attestation Framework](https://github.com/in-toto/attestation)
- [JSON Schema Specification](https://json-schema.org/)
