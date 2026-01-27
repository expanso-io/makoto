# Change: Comprehensive Makoto Roadmap

## Why

Makoto has extensive research (RESEARCH.md) and a detailed TODO.md, but lacks:
1. A structured implementation roadmap with clear milestones
2. Specification-driven development artifacts
3. Comparison documentation (vs Open Lineage, etc.)
4. SDK implementation guides (Python, Go, JavaScript)
5. The formal Open Schema for Schema.org submission
6. Signature/verification documentation

This proposal consolidates all planned work into a prioritized, trackable roadmap.

## What Changes

### Phase 1: Specification Foundation (Immediate)
- Formalize Makoto Level requirements (L1, L2, L3)
- Create JSON Schema for all predicate types
- Document comparison with Open Lineage
- Create signature/verification guide

### Phase 2: SDK & Developer Experience
- Python SDK with attestation generation/verification
- Go SDK (reference implementation)
- JavaScript/TypeScript SDK for browser verification
- CLI tool for attestation operations

### Phase 3: Open Schema & Standards
- Publish Schema.org vocabulary extension
- Submit predicate types to in-toto project
- Create OASIS DPS TC alignment document
- Develop formal specification website

### Phase 4: Platform Integration
- Expanso integration (native attestation support)
- Kafka Connect plugin
- Apache Beam transforms
- Cloud provider integrations (AWS, GCP, Azure)

### Phase 5: Ecosystem & Adoption
- Verification tooling (CLI, web explorer)
- CI/CD integration examples
- Conformance test suite
- Community governance model

## Impact

- Affected specs: New `makoto-framework` capability
- Affected code: docs/, examples/, schemas/, sdks/
- New directories: `schemas/`, `sdks/`, `tools/`
