"""Makoto Origin Attestation Predicate v1.

This module defines the predicate schema for origin attestations,
documenting where and how data was collected.
"""

from __future__ import annotations

from datetime import datetime
from typing import Annotated, Literal

from pydantic import BaseModel, ConfigDict, Field

from .common import ConsentInfo, DTACompliance, VersionInfo

PREDICATE_TYPE = "https://makoto.dev/origin/v1"
"""The predicate type URI for origin attestations."""


SourceType = Literal["api", "database", "file", "stream", "manual", "sensor", "web-scrape", "other"]
"""Type of data source."""

CollectionMethod = Literal[
    "scheduled-pull",
    "event-driven",
    "manual-upload",
    "continuous-stream",
    "batch-export",
    "webhook",
    "other",
]
"""How data was collected from the source."""

Environment = Literal["production", "staging", "development", "testing"]
"""Deployment environment."""


class Origin(BaseModel):
    """Information about the data source and collection."""

    model_config = ConfigDict(populate_by_name=True)

    source: Annotated[str, Field(description="URI or identifier of the data source")]
    collection_timestamp: Annotated[
        datetime,
        Field(
            alias="collectionTimestamp",
            description="When data collection occurred (RFC 3339)",
        ),
    ]
    source_type: Annotated[
        SourceType | None,
        Field(default=None, alias="sourceType", description="Type of data source"),
    ] = None
    collection_method: Annotated[
        CollectionMethod | None,
        Field(
            default=None,
            alias="collectionMethod",
            description="How data was collected from the source",
        ),
    ] = None
    geography: Annotated[
        str | None,
        Field(
            default=None,
            description="Geographic region where data was collected (ISO 3166-1 or cloud region)",
        ),
    ] = None
    consent: ConsentInfo | None = None


class Collector(BaseModel):
    """Information about the system that collected the data."""

    id: Annotated[str, Field(description="Unique identifier for the collector")]
    version: VersionInfo | None = None
    environment: Annotated[
        Environment | None, Field(default=None, description="Deployment environment")
    ] = None


class Schema(BaseModel):
    """Information about the data schema/format."""

    format: Annotated[
        str | None,
        Field(
            default=None,
            description="Data format (e.g., json-lines, csv, parquet, avro)",
        ),
    ] = None
    schema_ref: Annotated[
        str | None,
        Field(default=None, alias="schemaRef", description="URI to the schema definition"),
    ] = None
    schema_digest: Annotated[
        dict[str, str] | None,
        Field(default=None, alias="schemaDigest"),
    ] = None


class OriginMetadata(BaseModel):
    """Collection statistics and metrics."""

    collection_duration: Annotated[
        str | None,
        Field(
            default=None,
            alias="collectionDuration",
            description="ISO 8601 duration of the collection process",
        ),
    ] = None
    bytes_collected: Annotated[
        int | None,
        Field(default=None, alias="bytesCollected", ge=0, description="Total bytes collected"),
    ] = None
    records_collected: Annotated[
        int | None,
        Field(
            default=None,
            alias="recordsCollected",
            ge=0,
            description="Number of records collected",
        ),
    ] = None
    records_dropped: Annotated[
        int | None,
        Field(
            default=None,
            alias="recordsDropped",
            ge=0,
            description="Records dropped due to errors or filtering",
        ),
    ] = None
    error_rate: Annotated[
        float | None,
        Field(
            default=None,
            alias="errorRate",
            ge=0.0,
            le=1.0,
            description="Error rate as a decimal (0.0 to 1.0)",
        ),
    ] = None


class OriginPredicate(BaseModel):
    """Makoto Origin Attestation Predicate v1.

    Documents where data was collected, including source information,
    collection metadata, and compliance details.

    Example:
        ```python
        predicate = OriginPredicate(
            origin=Origin(
                source="https://api.example.com/v2/transactions",
                source_type="api",
                collection_method="scheduled-pull",
                collection_timestamp=datetime.now(UTC),
                geography="US-WEST-2",
            ),
            collector=Collector(
                id="https://expanso.io/collectors/prod-collector-01",
                environment="production",
            ),
        )
        ```
    """

    model_config = ConfigDict(extra="allow", populate_by_name=True)

    origin: Origin
    collector: Collector
    schema_: Annotated[Schema | None, Field(default=None, alias="schema")] = None
    metadata: OriginMetadata | None = None
    dta_compliance: Annotated[DTACompliance | None, Field(default=None, alias="dtaCompliance")] = (
        None
    )
