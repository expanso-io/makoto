# Makoto Roadmap Implementation Tasks

## 1. Specification Foundation

### 1.1 Formal Level Requirements
- [x] 1.1.1 Write L1 formal requirements document (docs/levels/index.html)
- [x] 1.1.2 Write L2 formal requirements document (docs/levels/index.html)
- [x] 1.1.3 Write L3 formal requirements document (docs/levels/index.html)
- [x] 1.1.4 Create level comparison matrix (docs/levels/index.html#adoption)

### 1.2 JSON Schema Definitions
- [x] 1.2.1 Create schema for `makoto.dev/origin/v1` predicate
- [x] 1.2.2 Create schema for `makoto.dev/transform/v1` predicate
- [x] 1.2.3 Create schema for `makoto.dev/stream-window/v1` predicate
- [x] 1.2.4 Create schema for DBOM format
- [x] 1.2.5 Add validation examples and test cases

### 1.3 Comparison Documentation
- [x] 1.3.1 Write "Makoto vs Open Lineage" comparison (docs/comparison/open-lineage.html)
- [ ] 1.3.2 Write "Makoto vs W3C PROV" comparison
- [x] 1.3.3 Write "Makoto vs SLSA" relationship document (docs/comparison/index.html)
- [ ] 1.3.4 Create decision guide: "Which standard should I use?"

### 1.4 Signature & Verification Guide
- [ ] 1.4.1 Document signing key management
- [ ] 1.4.2 Write signature generation guide
- [ ] 1.4.3 Write signature verification guide
- [ ] 1.4.4 Document timestamp authority integration
- [ ] 1.4.5 Create security best practices guide

## 2. SDK Development

### 2.1 Python SDK (`makoto-py`)
- [ ] 2.1.1 Create package structure and pyproject.toml
- [ ] 2.1.2 Implement attestation data models
- [ ] 2.1.3 Implement attestation generation
- [ ] 2.1.4 Implement signature operations (sign/verify)
- [ ] 2.1.5 Implement DBOM generation
- [ ] 2.1.6 Add streaming window support
- [ ] 2.1.7 Write comprehensive tests
- [ ] 2.1.8 Write SDK documentation
- [ ] 2.1.9 Publish to PyPI

### 2.2 Go SDK (`makoto-go`)
- [ ] 2.2.1 Create module structure
- [ ] 2.2.2 Implement attestation types
- [ ] 2.2.3 Implement generation and signing
- [ ] 2.2.4 Implement verification
- [ ] 2.2.5 Add Merkle tree implementation
- [ ] 2.2.6 Write tests and benchmarks
- [ ] 2.2.7 Write SDK documentation

### 2.3 JavaScript SDK (`makoto-js`)
- [ ] 2.3.1 Create package structure (TypeScript)
- [ ] 2.3.2 Implement browser-compatible verification
- [ ] 2.3.3 Implement Node.js attestation generation
- [ ] 2.3.4 Add WebCrypto signing support
- [ ] 2.3.5 Write tests (Jest)
- [ ] 2.3.6 Write SDK documentation
- [ ] 2.3.7 Publish to npm

## 3. Open Schema & Standards

### 3.1 Schema.org Extension
- [ ] 3.1.1 Design DataProvenance vocabulary
- [ ] 3.1.2 Define DataAttestation type
- [ ] 3.1.3 Define DataTransformation type
- [ ] 3.1.4 Create JSON-LD context file
- [ ] 3.1.5 Submit proposal to Schema.org
- [ ] 3.1.6 Host vocabulary at makoto.dev/schema/

### 3.2 in-toto Predicate Types
- [ ] 3.2.1 Draft predicate type proposal
- [ ] 3.2.2 Create reference implementations
- [ ] 3.2.3 Submit to in-toto project
- [ ] 3.2.4 Address review feedback

### 3.3 Standards Alignment
- [ ] 3.3.1 Map D&TA fields to Makoto predicates
- [ ] 3.3.2 Create OASIS DPS TC alignment document
- [ ] 3.3.3 Engage with OpenSSF community

## 4. Platform Integration

### 4.1 Expanso Integration
- [ ] 4.1.1 Design `makoto_attest` processor
- [ ] 4.1.2 Design `makoto_attestation` output
- [ ] 4.1.3 Implement L1 attestation generation
- [ ] 4.1.4 Implement L2 signing integration
- [ ] 4.1.5 Implement stream window attestation
- [ ] 4.1.6 Write integration documentation

### 4.2 Ecosystem Integrations
- [ ] 4.2.1 Create Kafka Connect sink connector
- [ ] 4.2.2 Create Apache Beam DoFn
- [ ] 4.2.3 Create AWS Lambda layer
- [ ] 4.2.4 Create GCP Cloud Function template

## 5. Tooling & Verification

### 5.1 CLI Tool (`makoto`)
- [ ] 5.1.1 Design CLI interface
- [ ] 5.1.2 Implement `makoto verify` command
- [ ] 5.1.3 Implement `makoto verify-chain` command
- [ ] 5.1.4 Implement `makoto generate` command
- [ ] 5.1.5 Implement `makoto sign` command
- [ ] 5.1.6 Add human-readable and JSON output
- [ ] 5.1.7 Write CLI documentation

### 5.2 Web Explorer
- [ ] 5.2.1 Create attestation upload interface
- [ ] 5.2.2 Implement client-side verification
- [ ] 5.2.3 Create lineage graph visualization
- [ ] 5.2.4 Add level compliance indicator
- [ ] 5.2.5 Deploy to usemakoto.dev/explorer/

## 6. Documentation & Website

### 6.1 Specification Website
- [ ] 6.1.1 Create specification landing page
- [ ] 6.1.2 Create level requirements pages
- [ ] 6.1.3 Create attestation format documentation
- [ ] 6.1.4 Create getting started guide
- [ ] 6.1.5 Create FAQ page

### 6.2 Integration Guides
- [ ] 6.2.1 Write Python integration guide
- [ ] 6.2.2 Write Go integration guide
- [ ] 6.2.3 Write JavaScript integration guide
- [ ] 6.2.4 Write Expanso integration guide
- [ ] 6.2.5 Write CI/CD integration examples

## 7. Community & Governance

### 7.1 Open Source Setup
- [ ] 7.1.1 Create CONTRIBUTING.md
- [ ] 7.1.2 Create CODE_OF_CONDUCT.md
- [ ] 7.1.3 Set up GitHub discussions
- [ ] 7.1.4 Create issue templates

### 7.2 Adoption & Outreach
- [ ] 7.2.1 Write launch blog post
- [ ] 7.2.2 Create conference presentation
- [ ] 7.2.3 Engage with SLSA community
- [ ] 7.2.4 Create adoption case studies
