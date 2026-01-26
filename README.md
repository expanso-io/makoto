# Makoto (誠) - Data Integrity Framework

> 誠 (makoto) — Japanese for "sincerity, truth, fidelity"

Makoto brings SLSA-style assurance levels to data pipelines, producing **DBOMs (Data Bills of Materials)** to prove chain of custody for your data.

## What is Makoto?

Just as [SLSA](https://slsa.dev) proves software artifacts weren't tampered with, Makoto proves **data** wasn't tampered with AND explains what happened to it. The output—a DBOM—is to datasets what an SBOM is to software packages.

## Makoto Levels

| Level | Guarantee | Description |
|-------|-----------|-------------|
| **L1** | Provenance Exists | Machine-readable attestation documents data origin and processing |
| **L2** | Provenance is Authentic | Cryptographically signed, tamper-evident attestations |
| **L3** | Provenance is Unforgeable | Platform-generated attestations from isolated signing infrastructure |

## Key Concepts

- **Makoto Levels** — Incremental security levels (L1/L2/L3) for data supply chain assurance
- **DBOM** — Data Bill of Materials documenting all data sources and transformations
- **Attestations** — in-toto compatible signed statements about data operations

## Predicate Types

Makoto defines three predicate types for [in-toto](https://in-toto.io) attestations:

- `https://makoto.dev/origin/v1` — Data origin and collection metadata
- `https://makoto.dev/transform/v1` — Data transformation with input/output hashing
- `https://makoto.dev/stream-window/v1` — Streaming data with Merkle tree verification

## Documentation

- [Specification](docs/spec/) — Full Makoto Levels specification
- [Examples](docs/examples/) — Sample attestation JSON
- [Threat Model](docs/threats/) — Data-specific threats and mitigations
- [Privacy Techniques](docs/privacy/) — Privacy-preserving attestation methods
- [Research](RESEARCH.md) — Framework analysis and design rationale
- [TODO](TODO.md) — Implementation roadmap

## Why This Matters

1. **AI/ML Governance** — EU AI Act and emerging regulations require training data provenance
2. **Data Marketplace Trust** — Verifiable claims about data origin and processing
3. **Compliance & Audit** — Immutable audit trails for regulated industries
4. **Supply Chain Security** — Protection against data pipeline attacks

## Quick Example

```json
{
  "_type": "https://in-toto.io/Statement/v1",
  "subject": [{
    "name": "dataset:transactions_2025q4",
    "digest": {
      "sha256": "abc123...",
      "recordCount": "1000000"
    }
  }],
  "predicateType": "https://makoto.dev/origin/v1",
  "predicate": {
    "origin": {
      "source": "https://api.partner.com/transactions",
      "collectionTimestamp": "2025-12-20T10:00:00Z",
      "geography": "US-WEST"
    },
    "collector": {
      "id": "https://example.com/collectors/prod-1"
    }
  }
}
```

## Status

**v0.1-draft** — Initial specification and research. Feedback welcome!

## Related Work

- [SLSA](https://slsa.dev) — Supply-chain Levels for Software Artifacts
- [in-toto](https://in-toto.io) — Attestation framework
- [D&TA Data Provenance Standards](https://www.dtaalliance.org/work/data-provenance-standards) — Metadata standards

## License

[Apache 2.0](LICENSE)

---

<p align="center">
  <strong>誠</strong> — Sincerity in data provenance
</p>
