# Makoto JSON Schemas

This directory contains JSON Schema definitions for validating Makoto attestations.

## Schemas

| Schema | Predicate Type | Description |
|--------|---------------|-------------|
| [origin-v1.json](origin-v1.json) | `https://makoto.dev/origin/v1` | Data origin attestation - documents provenance at collection |

## Usage

### Python (jsonschema)

```python
import json
from jsonschema import validate

schema = json.load(open('schemas/origin-v1.json'))
attestation = json.load(open('my-attestation.json'))
validate(attestation, schema)
```

### JavaScript (ajv)

```javascript
import Ajv from 'ajv';
import schema from './schemas/origin-v1.json';

const ajv = new Ajv();
const validate = ajv.compile(schema);
const valid = validate(attestation);
```

### CLI (ajv-cli)

```bash
npx ajv validate -s schemas/origin-v1.json -d attestation.json
```

## Schema Version

These schemas use JSON Schema Draft 2020-12.
