# Project Context

## Purpose

Makoto (誠, "sincerity/integrity" in Japanese) is a data integrity framework that brings SLSA-style assurance levels to data supply chains. While SLSA addresses "where, when, and how software artifacts were produced," Makoto addresses **"where, when, how, and through what transformations data was produced and processed."**

### Core Mission
- Provide verifiable data provenance for AI/ML training data, data marketplaces, and compliance-driven industries
- Enable Data Bills of Materials (DBOMs) analogous to SBOMs for software
- Support streaming data at scale (millions of events/second) with Merkle tree windows
- Align with existing standards (SLSA, in-toto, D&TA, W3C PROV)

### Target Users
- Data engineers building ETL/ELT pipelines
- ML engineers needing training data provenance
- Compliance officers requiring audit trails
- Organizations sharing/consuming data in marketplaces

## Tech Stack

- **Website**: Static HTML/CSS hosted on GitHub Pages at usemakoto.dev
- **Attestation Format**: in-toto Statement v1 with Makoto predicate types
- **Reference Implementation**: Expanso (Go-based data pipeline platform)
- **Schema Definitions**: JSON Schema + Protocol Buffers
- **Documentation**: Markdown specs, interactive HTML demos

## Project Conventions

### Code Style
- JSON examples use 2-space indentation
- YAML examples use 2-space indentation
- Attestation field names use snake_case
- Predicate types use URL format: `https://makoto.dev/<type>/v1`

### Architecture Patterns
- Build on in-toto attestation framework (don't reinvent)
- Three levels (L1, L2, L3) matching SLSA philosophy
- Privacy by design - attestations must not leak sensitive data
- Progressive adoption - L1 achievable with minimal tooling

### Testing Strategy
- JSON Schema validation for all attestation formats
- Example attestations for each predicate type
- Verification flow tests for each Makoto level

### Git Workflow
- Main branch for stable releases
- Feature branches for development
- Squash merge for clean history
- Conventional commits (feat:, fix:, docs:, etc.)

## Domain Context

### Key Concepts
- **Attestation**: Cryptographically signed statement about data or data operation
- **DBOM**: Data Bill of Materials - complete lineage documentation
- **Lineage**: Chain of transformations data has undergone
- **Origin**: Original source from which data was collected
- **Transform**: Any operation that modifies, filters, or combines data
- **Window**: Bounded subset of a data stream for attestation purposes

### Makoto Levels
- **L1 (Provenance Exists)**: Machine-readable attestation documents origin/processing
- **L2 (Provenance is Authentic)**: Attestations cryptographically signed, tamper-evident
- **L3 (Provenance is Unforgeable)**: Signing isolated from data processing logic

### Related Standards
- **SLSA**: Software Supply-chain Levels for Artifacts (sibling framework)
- **in-toto**: Attestation format (we build on this)
- **D&TA**: Data & Trust Alliance metadata fields
- **W3C PROV**: Provenance data model
- **SPDX**: Software bill of materials (model for DBOM)

## Important Constraints

- Attestations MUST NOT leak sensitive data (PII, business metrics)
- Solutions MUST scale to millions of events/second for streaming
- L1 MUST be achievable without cryptographic infrastructure
- Format MUST be compatible with in-toto Statement v1
- Spec MUST be independent of any single vendor implementation

## External Dependencies

- **in-toto project**: Attestation format specification
- **SLSA community**: Alignment and potential collaboration
- **Schema.org**: For publishing data provenance vocabulary
- **OpenSSF**: Potential home for standardization
- **OASIS DPS TC**: Data Provenance Standards alignment
