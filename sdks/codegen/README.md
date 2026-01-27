# Makoto SDK Code Generation

Generate type-safe Makoto SDK models from JSON Schemas for any language.

## Prerequisites

The generation tools are language-specific. Install the ones you need:

### Python
```bash
pip install datamodel-code-generator
```

### TypeScript
```bash
npm install -g json-schema-to-typescript
```

### Go
```bash
go install github.com/atombender/go-jsonschema/cmd/gojsonschema@latest
```

### Rust
```bash
cargo install typify-cli
```

### C#
```bash
dotnet tool install -g NJsonSchema.CodeGeneration.CSharp
```

### Java
```bash
# Use jsonschema2pojo Maven/Gradle plugin or CLI
```

## Quick Generation

```bash
# Generate all languages
make all

# Generate specific language
make python
make typescript
make go
make rust

# Clean generated files
make clean
```

## Manual Generation

### Python

```bash
datamodel-codegen \
    --input ../../schemas/makoto.dev/origin/v1.json \
    --input-file-type jsonschema \
    --output ../python/src/makoto/models/origin.py \
    --output-model-type pydantic_v2.BaseModel \
    --use-annotated \
    --field-constraints \
    --use-double-quotes \
    --target-python-version 3.11

# Repeat for transform, stream-window, dbom
```

### TypeScript

```bash
json2ts \
    --input ../../schemas/makoto.dev/origin/v1.json \
    --output ../typescript/src/models/origin.ts \
    --bannerComment "" \
    --style.singleQuote

# Or generate all at once
json2ts \
    --input '../../schemas/makoto.dev/**/v1.json' \
    --output ../typescript/src/models/
```

### Go

```bash
gojsonschema \
    --package models \
    --output ../go/models/origin.go \
    ../../schemas/makoto.dev/origin/v1.json

# Repeat for other schemas
```

### Rust

```bash
typify ../../schemas/makoto.dev/origin/v1.json \
    --output ../rust/src/models/origin.rs

# Or use build.rs for compile-time generation
```

## Schema Structure

The schemas are organized as:

```
schemas/makoto.dev/
├── common/
│   └── definitions.json    # Shared types (DigestSet, MakotoLevel, etc.)
├── origin/
│   └── v1.json             # Origin attestation predicate
├── transform/
│   └── v1.json             # Transform attestation predicate
├── stream-window/
│   └── v1.json             # Stream window attestation predicate
└── dbom/
    └── v1.json             # Data Bill of Materials
```

### Key Types

| Schema | Generated Type | Description |
|--------|---------------|-------------|
| `common/definitions.json` | `DigestSet`, `MakotoLevel`, etc. | Shared types |
| `origin/v1.json` | `OriginPredicate` | Data collection metadata |
| `transform/v1.json` | `TransformPredicate` | Data transformation metadata |
| `stream-window/v1.json` | `StreamWindowPredicate` | Streaming window metadata |
| `dbom/v1.json` | `DBOM` | Complete lineage document |

## Post-Generation Steps

Generated code provides data models only. You'll need to add:

### 1. Attestation Wrapper

Wrap predicates in in-toto Statement format:

```python
# Python example
class Attestation:
    _type: str = "https://in-toto.io/Statement/v1"
    subject: List[Subject]
    predicateType: str
    predicate: Union[OriginPredicate, TransformPredicate, ...]
```

### 2. Signing

Add Sigstore or custom signing:

```python
# Python with sigstore
from sigstore.sign import Signer
from sigstore.oidc import Issuer

def sign_attestation(attestation: Attestation) -> SignedAttestation:
    signer = Signer.production()
    # ... signing logic
```

### 3. Verification

Add signature and hash verification:

```python
def verify_attestation(signed: SignedAttestation) -> bool:
    # Verify signature
    # Verify subject digest matches data
    # Verify input hashes for transforms
    pass
```

### 4. Merkle Trees (for streaming)

Implement Merkle tree operations for stream windows:

```python
def compute_merkle_root(records: List[bytes]) -> str:
    # Build tree and return root hash
    pass

def generate_merkle_proof(records: List[bytes], index: int) -> MerkleProof:
    # Generate inclusion proof for record at index
    pass
```

## Validation

Test generated code against example attestations:

```bash
# Python
python -c "
from makoto.models import OriginPredicate
import json
with open('../../docs/attestations/origin-example.json') as f:
    data = json.load(f)
    pred = OriginPredicate(**data['predicate'])
    print('Validation passed!')
"
```

## Customization

### Adding Custom Fields

The schemas use `additionalProperties: true`, so you can extend types:

```python
class ExtendedOrigin(OriginPredicate):
    custom_field: str
    internal_id: Optional[int]
```

### Custom Validation

Add business logic validation beyond schema constraints:

```python
def validate_origin(origin: OriginPredicate) -> List[str]:
    errors = []
    if origin.metadata and origin.metadata.error_rate > 0.1:
        errors.append("Error rate exceeds 10% threshold")
    return errors
```

## Troubleshooting

### Schema References Not Resolving

Some generators need all schemas in one directory:

```bash
# Copy schemas to temp directory
cp -r ../../schemas/makoto.dev/* /tmp/makoto-schemas/
# Generate from flat structure
datamodel-codegen --input /tmp/makoto-schemas/ ...
```

### Circular References

Use `--use-generic-container-types` for Python or equivalent for other languages.

### Missing Optional Fields

Ensure your generator supports `"required": []` correctly. Some need explicit nullable annotations.

## Language-Specific Notes

### Python
- Use `pydantic_v2.BaseModel` for best validation
- Add `model_config = {"extra": "allow"}` for extension support

### TypeScript
- Generated interfaces don't include runtime validation
- Consider using `zod` or `io-ts` for runtime checks

### Go
- Generated structs use `json` tags
- Add custom `MarshalJSON`/`UnmarshalJSON` for complex types

### Rust
- Use `serde` derives for serialization
- Consider `validator` crate for additional validation
