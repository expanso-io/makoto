"""Makoto Data Bill of Materials (DBOM) v1.

This module defines the schema for a complete DBOM, documenting
data lineage from sources through all transformations.
"""

from __future__ import annotations

from datetime import datetime
from typing import Annotated, Literal

from pydantic import BaseModel, ConfigDict, Field

from .common import ConsentInfo, MakotoLevel

SCHEMA_VERSION = "1.0.0"
"""Current DBOM schema version."""


class DatasetCreator(BaseModel):
    """Creator information for a dataset."""

    organization: Annotated[
        str | None,
        Field(default=None, description="Organization that created the dataset"),
    ] = None
    contact: Annotated[str | None, Field(default=None, description="Contact email or URI")] = None


class DatasetDigest(BaseModel):
    """Digest information for a dataset."""

    sha256: Annotated[str, Field(description="SHA-256 hash of the dataset")]
    record_count: Annotated[
        str | None, Field(default=None, alias="recordCount", description="Number of records")
    ] = None
    format: Annotated[
        str | None,
        Field(default=None, description="Data format (e.g., parquet, csv, json)"),
    ] = None


class Dataset(BaseModel):
    """The final dataset this DBOM describes."""

    name: Annotated[str, Field(description="Name of the dataset")]
    digest: DatasetDigest
    version: Annotated[str | None, Field(default=None, description="Version of the dataset")] = None
    description: Annotated[
        str | None, Field(default=None, description="Human-readable description")
    ] = None
    created: Annotated[
        datetime | None, Field(default=None, description="When the dataset was created")
    ] = None
    creator: DatasetCreator | None = None
    makoto_level: Annotated[MakotoLevel | None, Field(default=None, alias="makotoLevel")] = None


class SourceLicense(BaseModel):
    """License information for a data source."""

    type: Annotated[str | None, Field(default=None, description="License type")] = None
    identifier: Annotated[
        str | None, Field(default=None, description="SPDX license identifier")
    ] = None
    reference: Annotated[str | None, Field(default=None, description="Link to license text")] = None


class SourceContribution(BaseModel):
    """How much a source contributes to the final dataset."""

    record_count: Annotated[int | None, Field(default=None, alias="recordCount", ge=0)] = None
    record_percentage: Annotated[
        float | None, Field(default=None, alias="recordPercentage", ge=0, le=100)
    ] = None


class Source(BaseModel):
    """A source dataset contributing to the final dataset."""

    model_config = ConfigDict(populate_by_name=True)

    name: Annotated[str, Field(description="Source dataset name")]
    attestation_ref: Annotated[
        str, Field(alias="attestationRef", description="Reference to the origin attestation")
    ]
    description: Annotated[
        str | None, Field(default=None, description="Description of the source")
    ] = None
    attestation_type: Annotated[
        Literal["https://makoto.dev/origin/v1"] | None,
        Field(default=None, alias="attestationType", description="Type of attestation"),
    ] = None
    makoto_level: Annotated[MakotoLevel | None, Field(default=None, alias="makotoLevel")] = None
    geography: Annotated[
        str | None, Field(default=None, description="Geographic origin of the data")
    ] = None
    consent: ConsentInfo | None = None
    license: SourceLicense | None = None
    contribution: SourceContribution | None = None


class Transformation(BaseModel):
    """A transformation in the processing chain."""

    order: Annotated[int, Field(ge=1, description="Order in the transformation chain")]
    name: Annotated[str, Field(description="Name of the transformation")]
    attestation_ref: Annotated[
        str,
        Field(alias="attestationRef", description="Reference to the transform attestation"),
    ]
    inputs: Annotated[list[str], Field(description="Input dataset names")]
    outputs: Annotated[list[str], Field(description="Output dataset names")]
    description: Annotated[
        str | None, Field(default=None, description="What this transformation does")
    ] = None
    attestation_type: Annotated[
        Literal["https://makoto.dev/transform/v1"] | None,
        Field(default=None, alias="attestationType", description="Type of attestation"),
    ] = None
    makoto_level: Annotated[MakotoLevel | None, Field(default=None, alias="makotoLevel")] = None
    transform_type: Annotated[
        str | None,
        Field(default=None, alias="transformType", description="Type URI for the transformation"),
    ] = None


LineageGraphFormat = Literal["graphviz-dot", "mermaid", "json-graph"]
"""Format for lineage graph content."""


class LineageGraph(BaseModel):
    """Visual representation of the data lineage."""

    format: Annotated[LineageGraphFormat, Field(description="Format of the graph content")]
    content: Annotated[str, Field(description="Graph content in the specified format")]


class PrivacyAssessment(BaseModel):
    """Privacy-related assessments."""

    pii_removed: Annotated[bool | None, Field(default=None, alias="piiRemoved")] = None
    anonymization_verified: Annotated[
        bool | None, Field(default=None, alias="anonymizationVerified")
    ] = None
    k_anonymity: Annotated[int | None, Field(default=None, alias="kAnonymity", ge=1)] = None
    l_diversity: Annotated[int | None, Field(default=None, alias="lDiversity", ge=1)] = None


ComplianceStatus = Literal["compliant", "non-compliant", "not-applicable", "pending-review"]
"""Regulatory compliance status."""


class RegulatoryCompliance(BaseModel):
    """Regulatory compliance status entry."""

    regulation: Annotated[str, Field(description="Regulation name/article")]
    status: ComplianceStatus
    notes: str | None = None


class DTAComplianceInfo(BaseModel):
    """D&TA compliance information for DBOM."""

    standards_version: Annotated[str | None, Field(default=None, alias="standardsVersion")] = None
    all_fields_present: Annotated[bool | None, Field(default=None, alias="allFieldsPresent")] = None


class Compliance(BaseModel):
    """Compliance and governance information."""

    overall_makoto_level: Annotated[
        MakotoLevel | None, Field(default=None, alias="overallMakotoLevel")
    ] = None
    level_justification: Annotated[
        str | None,
        Field(
            default=None,
            alias="levelJustification",
            description="Explanation of how the level was determined",
        ),
    ] = None
    privacy_assessment: Annotated[
        PrivacyAssessment | None, Field(default=None, alias="privacyAssessment")
    ] = None
    regulatory_compliance: Annotated[
        list[RegulatoryCompliance] | None,
        Field(default=None, alias="regulatoryCompliance"),
    ] = None
    dta_compliance: Annotated[
        DTAComplianceInfo | None, Field(default=None, alias="dtaCompliance")
    ] = None


class Verifier(BaseModel):
    """Verification tool information."""

    tool: str | None = None
    version: str | None = None


class DBOMVerification(BaseModel):
    """Results of verification checks."""

    chain_verified: Annotated[
        bool | None,
        Field(
            default=None,
            alias="chainVerified",
            description="Whether the full attestation chain was verified",
        ),
    ] = None
    all_signatures_valid: Annotated[
        bool | None,
        Field(
            default=None,
            alias="allSignaturesValid",
            description="Whether all signatures are valid",
        ),
    ] = None
    attestation_count: Annotated[
        int | None,
        Field(
            default=None,
            alias="attestationCount",
            ge=0,
            description="Total number of attestations in the chain",
        ),
    ] = None
    verification_timestamp: Annotated[
        datetime | None,
        Field(
            default=None,
            alias="verificationTimestamp",
            description="When verification was performed",
        ),
    ] = None
    verifier: Verifier | None = None


class Generator(BaseModel):
    """DBOM generator information."""

    tool: str | None = None
    version: str | None = None


Visibility = Literal["public", "internal", "restricted", "confidential"]
"""Access control visibility level."""


class AccessControl(BaseModel):
    """Access control information."""

    visibility: Visibility | None = None
    allowed_consumers: Annotated[
        list[str] | None, Field(default=None, alias="allowedConsumers")
    ] = None


class DBOMMetadata(BaseModel):
    """DBOM metadata."""

    generator: Generator | None = None
    created: datetime | None = None
    valid_until: Annotated[datetime | None, Field(default=None, alias="validUntil")] = None
    access_control: Annotated[AccessControl | None, Field(default=None, alias="accessControl")] = (
        None
    )


class DBOM(BaseModel):
    """Makoto Data Bill of Materials (DBOM) v1.

    A complete record of data lineage from original sources through
    all transformations to the final dataset.

    Example:
        ```python
        dbom = DBOM(
            dbom_id="urn:uuid:550e8400-e29b-41d4-a716-446655440000",
            dataset=Dataset(
                name="customer_analytics_v2",
                digest=DatasetDigest(sha256="abc123..."),
            ),
            sources=[
                Source(
                    name="crm_export",
                    attestation_ref="https://registry.example.com/att/123",
                )
            ],
        )
        ```
    """

    model_config = ConfigDict(extra="allow", populate_by_name=True)

    dbom_version: Annotated[
        Literal["1.0.0"],
        Field(alias="dbomVersion", description="Version of the DBOM schema"),
    ] = "1.0.0"
    dbom_id: Annotated[
        str,
        Field(
            alias="dbomId",
            description="Unique identifier for this DBOM (URN format recommended)",
        ),
    ]
    dataset: Dataset
    sources: Annotated[
        list[Source],
        Field(min_length=1, description="All source datasets contributing to the final dataset"),
    ]
    transformations: list[Transformation] | None = None
    lineage_graph: Annotated[LineageGraph | None, Field(default=None, alias="lineageGraph")] = None
    compliance: Compliance | None = None
    verification: DBOMVerification | None = None
    metadata: DBOMMetadata | None = None
