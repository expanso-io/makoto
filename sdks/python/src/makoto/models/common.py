"""Common type definitions for Makoto attestations.

This module contains shared types used across all Makoto predicate schemas,
including digest sets, resource descriptors, and compliance information.
"""

from __future__ import annotations

from datetime import datetime
from typing import Annotated, Literal

from pydantic import BaseModel, ConfigDict, Field


class DigestSet(BaseModel):
    """A set of cryptographic digests for an artifact.

    At least one digest algorithm must be provided. SHA-256 is recommended
    as the primary algorithm for interoperability.
    """

    model_config = ConfigDict(extra="allow")

    sha256: Annotated[
        str | None,
        Field(
            default=None,
            pattern=r"^[a-f0-9]{64}$",
            description="SHA-256 hash in lowercase hex",
        ),
    ] = None
    sha384: Annotated[
        str | None,
        Field(
            default=None,
            pattern=r"^[a-f0-9]{96}$",
            description="SHA-384 hash in lowercase hex",
        ),
    ] = None
    sha512: Annotated[
        str | None,
        Field(
            default=None,
            pattern=r"^[a-f0-9]{128}$",
            description="SHA-512 hash in lowercase hex",
        ),
    ] = None


class SubjectDigest(BaseModel):
    """Extended digest for Makoto subjects including data-specific fields.

    Used in in-toto statement subjects to provide additional context
    for data attestations beyond simple file hashes.
    """

    model_config = ConfigDict(extra="allow")

    sha256: Annotated[
        str | None,
        Field(default=None, pattern=r"^[a-f0-9]{64}$"),
    ] = None
    merkle_root: Annotated[
        str | None,
        Field(
            default=None,
            alias="merkleRoot",
            description="Merkle tree root hash for the data",
        ),
    ] = None
    record_count: Annotated[
        str | None,
        Field(
            default=None,
            alias="recordCount",
            pattern=r"^[0-9]+$",
            description="Number of records (as string for large values)",
        ),
    ] = None
    window_start: Annotated[
        datetime | None,
        Field(
            default=None,
            alias="windowStart",
            description="Start of window for stream subjects",
        ),
    ] = None
    window_end: Annotated[
        datetime | None,
        Field(
            default=None,
            alias="windowEnd",
            description="End of window for stream subjects",
        ),
    ] = None


MakotoLevel = Literal["L1", "L2", "L3"]
"""Makoto assurance level.

- L1: Provenance Exists - Machine-readable attestation documents data origin
- L2: Provenance is Authentic - Cryptographically signed, tamper-evident
- L3: Provenance is Unforgeable - Platform-generated from isolated infrastructure
"""


class ResourceDescriptor(BaseModel):
    """Describes a data resource (input or output).

    Used to reference datasets in transform attestations with their
    cryptographic identity and optional provenance information.
    """

    name: Annotated[str, Field(description="Identifier for the resource")]
    digest: DigestSet
    uri: Annotated[
        str | None,
        Field(default=None, description="URI locating the resource"),
    ] = None
    attestation_ref: Annotated[
        str | None,
        Field(
            default=None,
            alias="attestationRef",
            description="Reference to attestation for this resource",
        ),
    ] = None
    makoto_level: Annotated[
        MakotoLevel | None,
        Field(default=None, alias="makotoLevel"),
    ] = None


class VersionInfo(BaseModel):
    """Software version information.

    A flexible key-value mapping of component names to version strings.
    """

    model_config = ConfigDict(extra="allow")


class CodeReference(BaseModel):
    """Reference to source code.

    Provides traceability from attestations back to the code that
    produced or transformed the data.
    """

    uri: Annotated[str, Field(description="URI to the code repository")]
    commit: Annotated[
        str | None,
        Field(default=None, description="Git commit hash or equivalent"),
    ] = None
    path: Annotated[
        str | None,
        Field(default=None, description="Path within the repository"),
    ] = None
    digest: DigestSet | None = None


class ConsentInfo(BaseModel):
    """Data collection consent information.

    Documents the legal basis for data collection, aligned with
    GDPR lawful bases and similar frameworks.
    """

    type: Annotated[
        Literal[
            "explicit",
            "contractual",
            "legitimate-interest",
            "public-interest",
            "legal-obligation",
            "vital-interest",
        ],
        Field(description="GDPR lawful basis or equivalent consent type"),
    ]
    reference: Annotated[
        str | None,
        Field(default=None, description="Reference to consent documentation"),
    ] = None
    obtained: datetime | None = None


class DTASourceStandard(BaseModel):
    """D&TA Source Standard fields.

    Aligns with Data & Trust Alliance Data Provenance Standards v1.0.0.
    """

    dataset_title: Annotated[str | None, Field(default=None, alias="datasetTitle")] = None
    dataset_issuer: Annotated[str | None, Field(default=None, alias="datasetIssuer")] = None
    description: str | None = None


class DTAProvenanceStandard(BaseModel):
    """D&TA Provenance Standard fields."""

    data_origin_geography: Annotated[
        str | None, Field(default=None, alias="dataOriginGeography")
    ] = None
    method: str | None = None
    data_format: Annotated[str | None, Field(default=None, alias="dataFormat")] = None


class DTAUseStandard(BaseModel):
    """D&TA Use Standard fields."""

    confidentiality_classification: Annotated[
        str | None, Field(default=None, alias="confidentialityClassification")
    ] = None
    intended_data_use: Annotated[str | None, Field(default=None, alias="intendedDataUse")] = None
    license: str | None = None


class DTACompliance(BaseModel):
    """Data & Trust Alliance compliance information.

    Provides compatibility with D&TA Data Provenance Standards.
    """

    standards_version: Annotated[str | None, Field(default=None, alias="standardsVersion")] = None
    source_standard: Annotated[
        DTASourceStandard | None, Field(default=None, alias="sourceStandard")
    ] = None
    provenance_standard: Annotated[
        DTAProvenanceStandard | None, Field(default=None, alias="provenanceStandard")
    ] = None
    use_standard: Annotated[DTAUseStandard | None, Field(default=None, alias="useStandard")] = None
