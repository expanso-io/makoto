"""Makoto Stream Window Attestation Predicate v1.

This module defines the predicate schema for stream window attestations,
documenting bounded subsets of streaming data with Merkle tree integrity.
"""

from __future__ import annotations

from datetime import datetime
from typing import Annotated, Literal

from pydantic import BaseModel, ConfigDict, Field

from .common import VersionInfo

PREDICATE_TYPE = "https://makoto.dev/stream-window/v1"
"""The predicate type URI for stream window attestations."""


WindowType = Literal["tumbling", "sliding", "session", "global"]
"""Type of window."""

TimeAlignment = Literal["wall-clock", "event-time", "processing-time"]
"""Time alignment for the window."""

HashAlgorithm = Literal["sha256", "sha384", "sha512", "blake2b", "blake3"]
"""Hash algorithm for Merkle trees."""


class Stream(BaseModel):
    """Information about the data stream."""

    id: Annotated[str, Field(description="Unique identifier for the stream")]
    source: Annotated[str, Field(description="Source URI for the stream (e.g., mqtt://, kafka://)")]
    topic: Annotated[str | None, Field(default=None, description="Topic or channel name")] = None
    partitions: Annotated[
        list[str] | None,
        Field(default=None, description="Specific partitions included in this window"),
    ] = None


class Window(BaseModel):
    """Window definition and boundaries."""

    type: Annotated[WindowType, Field(description="Type of window")]
    duration: Annotated[str, Field(description="Window duration (ISO 8601 duration)")]
    slide: Annotated[
        str | None,
        Field(
            default=None,
            description="Slide interval for sliding windows (ISO 8601 duration)",
        ),
    ] = None
    alignment: Annotated[
        TimeAlignment | None,
        Field(default=None, description="Time alignment for the window"),
    ] = None
    watermark: Annotated[
        datetime | None, Field(default=None, description="Watermark timestamp for this window")
    ] = None
    allowed_lateness: Annotated[
        str | None,
        Field(
            default=None,
            alias="allowedLateness",
            description="Allowed late data tolerance (ISO 8601 duration)",
        ),
    ] = None


class MerkleTree(BaseModel):
    """Merkle tree details for the window records."""

    model_config = ConfigDict(populate_by_name=True)

    algorithm: Annotated[HashAlgorithm, Field(description="Hash algorithm for the tree")]
    leaf_count: Annotated[
        int,
        Field(alias="leafCount", ge=0, description="Number of leaf nodes (records)"),
    ]
    root: Annotated[str, Field(description="Merkle root hash")]
    leaf_hash_algorithm: Annotated[
        str | None,
        Field(
            default=None,
            alias="leafHashAlgorithm",
            description="Algorithm used for leaf hashes (if different)",
        ),
    ] = None
    tree_height: Annotated[
        int | None,
        Field(
            default=None,
            alias="treeHeight",
            ge=0,
            description="Height of the Merkle tree",
        ),
    ] = None


class Chain(BaseModel):
    """Chaining information linking to previous windows."""

    previous_window_id: Annotated[
        str | None,
        Field(
            default=None,
            alias="previousWindowId",
            description="ID of the previous window in the chain",
        ),
    ] = None
    previous_merkle_root: Annotated[
        str | None,
        Field(
            default=None,
            alias="previousMerkleRoot",
            description="Merkle root of the previous window",
        ),
    ] = None
    chain_length: Annotated[
        int | None,
        Field(
            default=None,
            alias="chainLength",
            ge=1,
            description="Position in the window chain (1-indexed)",
        ),
    ] = None
    genesis_window_id: Annotated[
        str | None,
        Field(
            default=None,
            alias="genesisWindowId",
            description="ID of the first window in this chain",
        ),
    ] = None


class Integrity(BaseModel):
    """Cryptographic integrity information for the window."""

    model_config = ConfigDict(populate_by_name=True)

    merkle_tree: Annotated[MerkleTree, Field(alias="merkleTree")]
    chain: Chain | None = None


class Statistics(BaseModel):
    """Statistical summary of window data."""

    model_config = ConfigDict(extra="allow")

    min_timestamp: Annotated[
        datetime | None,
        Field(
            default=None,
            alias="minTimestamp",
            description="Earliest record timestamp in window",
        ),
    ] = None
    max_timestamp: Annotated[
        datetime | None,
        Field(
            default=None,
            alias="maxTimestamp",
            description="Latest record timestamp in window",
        ),
    ] = None
    avg_interval_ms: Annotated[
        float | None,
        Field(
            default=None,
            alias="avgIntervalMs",
            ge=0,
            description="Average interval between records in milliseconds",
        ),
    ] = None


class Aggregates(BaseModel):
    """Aggregate statistics for the window."""

    checksum: Annotated[
        str | None, Field(default=None, description="Checksum of aggregate values")
    ] = None
    statistics: Statistics | None = None


class StreamCollector(BaseModel):
    """Information about the stream processor."""

    id: Annotated[str, Field(description="Unique identifier for the collector")]
    version: VersionInfo | None = None
    location: Annotated[
        str | None,
        Field(default=None, description="Physical or logical location of the collector"),
    ] = None


class StreamMetadata(BaseModel):
    """Processing metrics for the window."""

    processing_latency: Annotated[
        str | None,
        Field(
            default=None,
            alias="processingLatency",
            description="Processing latency (ISO 8601 duration)",
        ),
    ] = None
    late_records: Annotated[
        int | None,
        Field(
            default=None,
            alias="lateRecords",
            ge=0,
            description="Number of late-arriving records",
        ),
    ] = None
    dropped_records: Annotated[
        int | None,
        Field(
            default=None,
            alias="droppedRecords",
            ge=0,
            description="Records dropped (exceeded lateness)",
        ),
    ] = None
    backpressure_events: Annotated[
        int | None,
        Field(
            default=None,
            alias="backpressureEvents",
            ge=0,
            description="Number of backpressure events",
        ),
    ] = None


class StreamVerification(BaseModel):
    """Verification endpoint information."""

    merkle_proof_available: Annotated[
        bool | None,
        Field(
            default=None,
            alias="merkleProofAvailable",
            description="Whether Merkle proofs are available",
        ),
    ] = None
    proof_endpoint: Annotated[
        str | None,
        Field(
            default=None,
            alias="proofEndpoint",
            description="Endpoint to retrieve Merkle proofs",
        ),
    ] = None


class StreamWindowPredicate(BaseModel):
    """Makoto Stream Window Attestation Predicate v1.

    Documents bounded subsets of streaming data with cryptographic
    integrity via Merkle trees and optional window chaining.

    Example:
        ```python
        predicate = StreamWindowPredicate(
            stream=Stream(
                id="iot_sensors",
                source="mqtt://sensors.example.com:1883",
                topic="sensors/+/readings",
            ),
            window=Window(
                type="tumbling",
                duration="PT1M",
                alignment="wall-clock",
            ),
            integrity=Integrity(
                merkle_tree=MerkleTree(
                    algorithm="sha256",
                    leaf_count=847293,
                    tree_height=20,
                    root="abc123def456789...",
                )
            ),
            collector=StreamCollector(
                id="https://expanso.io/edge/factory-edge-01",
            ),
        )
        ```
    """

    model_config = ConfigDict(extra="allow", populate_by_name=True)

    stream: Stream
    window: Window
    integrity: Integrity
    collector: StreamCollector
    aggregates: Aggregates | None = None
    metadata: StreamMetadata | None = None
    verification: StreamVerification | None = None
