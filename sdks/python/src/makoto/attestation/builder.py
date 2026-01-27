"""Attestation builder for creating Makoto attestations.

This module provides a fluent builder interface for creating
in-toto compatible attestations with Makoto predicates.
"""

from __future__ import annotations

import hashlib
from dataclasses import dataclass, field
from datetime import datetime, timezone
from pathlib import Path
from typing import TYPE_CHECKING

from ..models import origin as origin_module
from ..models import stream_window as stream_window_module
from ..models import transform as transform_module
from ..models.common import DigestSet
from ..models.origin import Collector, Origin, OriginPredicate
from ..models.stream_window import (
    Integrity,
    MerkleTree,
    Stream,
    StreamCollector,
    StreamWindowPredicate,
    Window,
)
from ..models.transform import Executor, Transform, TransformInput, TransformPredicate
from .statement import InTotoStatement, Subject

if TYPE_CHECKING:
    from ..models.common import MakotoLevel


def compute_sha256(data: bytes) -> str:
    """Compute SHA-256 hash of data.

    Args:
        data: Bytes to hash

    Returns:
        Lowercase hex digest
    """
    return hashlib.sha256(data).hexdigest()


def compute_file_sha256(path: Path | str) -> str:
    """Compute SHA-256 hash of a file.

    Args:
        path: Path to the file

    Returns:
        Lowercase hex digest
    """
    h = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(8192), b""):
            h.update(chunk)
    return h.hexdigest()


@dataclass
class _OriginConfig:
    """Configuration for building an origin predicate."""

    source: str
    collector_id: str
    source_type: str | None = None
    collection_method: str | None = None
    geography: str | None = None
    environment: str | None = None
    collection_timestamp: datetime | None = None


@dataclass
class _TransformConfig:
    """Configuration for building a transform predicate."""

    transform_type: str
    transform_name: str
    executor_id: str
    transform_version: str | None = None
    transform_description: str | None = None
    platform: str | None = None
    environment: str | None = None
    inputs: list[TransformInput] = field(default_factory=list)


@dataclass
class _StreamWindowConfig:
    """Configuration for building a stream window predicate."""

    stream_id: str
    stream_source: str
    window_type: str
    window_duration: str
    merkle_algorithm: str
    merkle_root: str
    leaf_count: int
    collector_id: str
    topic: str | None = None
    alignment: str | None = None


class AttestationBuilder:
    """Fluent builder for creating Makoto attestations.

    Example:
        ```python
        builder = AttestationBuilder()

        # Create an origin attestation
        statement = (
            builder
            .origin(
                source="https://api.example.com/data",
                collector_id="my-collector",
            )
            .with_subject_file("data.csv")
            .build()
        )

        # Create a transform attestation
        statement = (
            builder
            .transform(
                transform_type="https://makoto.dev/transforms/filter",
                transform_name="Remove PII",
                executor_id="pipeline-01",
            )
            .with_input("input.csv", "abc123...")
            .with_subject_file("output.csv")
            .build()
        )
        ```
    """

    def __init__(self) -> None:
        """Initialize a new attestation builder."""
        self._subjects: list[Subject] = []
        self._config: _OriginConfig | _TransformConfig | _StreamWindowConfig | None = None
        self._predicate_type: str | None = None

    def origin(
        self,
        source: str,
        collector_id: str,
        *,
        source_type: str | None = None,
        collection_method: str | None = None,
        geography: str | None = None,
        environment: str | None = None,
        collection_timestamp: datetime | None = None,
    ) -> AttestationBuilder:
        """Configure an origin attestation.

        Args:
            source: URI or identifier of the data source
            collector_id: Unique identifier for the collector
            source_type: Type of data source (api, database, file, etc.)
            collection_method: How data was collected
            geography: Geographic region (ISO 3166-1 or cloud region)
            environment: Deployment environment
            collection_timestamp: When collection occurred (defaults to now)

        Returns:
            Self for chaining
        """
        self._config = _OriginConfig(
            source=source,
            collector_id=collector_id,
            source_type=source_type,
            collection_method=collection_method,
            geography=geography,
            environment=environment,
            collection_timestamp=collection_timestamp,
        )
        self._predicate_type = origin_module.PREDICATE_TYPE
        return self

    def transform(
        self,
        transform_type: str,
        transform_name: str,
        executor_id: str,
        *,
        transform_version: str | None = None,
        transform_description: str | None = None,
        platform: str | None = None,
        environment: str | None = None,
    ) -> AttestationBuilder:
        """Configure a transform attestation.

        Args:
            transform_type: URI identifying the transform type
            transform_name: Human-readable name
            executor_id: Unique identifier for the executor
            transform_version: Version of the transformation
            transform_description: Description of what the transform does
            platform: Platform name (expanso, spark, flink, etc.)
            environment: Deployment environment

        Returns:
            Self for chaining
        """
        self._config = _TransformConfig(
            transform_type=transform_type,
            transform_name=transform_name,
            executor_id=executor_id,
            transform_version=transform_version,
            transform_description=transform_description,
            platform=platform,
            environment=environment,
        )
        self._predicate_type = transform_module.PREDICATE_TYPE
        return self

    def stream_window(
        self,
        stream_id: str,
        stream_source: str,
        window_type: str,
        window_duration: str,
        merkle_algorithm: str,
        merkle_root: str,
        leaf_count: int,
        collector_id: str,
        *,
        topic: str | None = None,
        alignment: str | None = None,
    ) -> AttestationBuilder:
        """Configure a stream window attestation.

        Args:
            stream_id: Unique identifier for the stream
            stream_source: Source URI (mqtt://, kafka://, etc.)
            window_type: Window type (tumbling, sliding, session, global)
            window_duration: Window duration (ISO 8601)
            merkle_algorithm: Hash algorithm for Merkle tree
            merkle_root: Merkle root hash
            leaf_count: Number of records in the window
            collector_id: Unique identifier for the collector
            topic: Topic or channel name
            alignment: Time alignment (wall-clock, event-time, processing-time)

        Returns:
            Self for chaining
        """
        self._config = _StreamWindowConfig(
            stream_id=stream_id,
            stream_source=stream_source,
            window_type=window_type,
            window_duration=window_duration,
            merkle_algorithm=merkle_algorithm,
            merkle_root=merkle_root,
            leaf_count=leaf_count,
            collector_id=collector_id,
            topic=topic,
            alignment=alignment,
        )
        self._predicate_type = stream_window_module.PREDICATE_TYPE
        return self

    def with_input(
        self,
        name: str,
        sha256: str,
        *,
        attestation_ref: str | None = None,
        makoto_level: MakotoLevel | None = None,
    ) -> AttestationBuilder:
        """Add an input dataset (for transform attestations).

        Args:
            name: Identifier for the input dataset
            sha256: SHA-256 hash of the input
            attestation_ref: Reference to the input's attestation
            makoto_level: Makoto level of the input

        Returns:
            Self for chaining

        Raises:
            ValueError: If not building a transform attestation
        """
        if not isinstance(self._config, _TransformConfig):
            raise ValueError("with_input() can only be used with transform attestations")

        self._config.inputs.append(
            TransformInput(
                name=name,
                digest=DigestSet(sha256=sha256),
                attestation_ref=attestation_ref,
                makoto_level=makoto_level,
            )
        )
        return self

    def with_input_file(
        self,
        path: Path | str,
        *,
        name: str | None = None,
        attestation_ref: str | None = None,
        makoto_level: MakotoLevel | None = None,
    ) -> AttestationBuilder:
        """Add an input dataset from a file (for transform attestations).

        Args:
            path: Path to the input file
            name: Optional name (defaults to filename)
            attestation_ref: Reference to the input's attestation
            makoto_level: Makoto level of the input

        Returns:
            Self for chaining
        """
        path = Path(path)
        sha256 = compute_file_sha256(path)
        return self.with_input(
            name=name or path.name,
            sha256=sha256,
            attestation_ref=attestation_ref,
            makoto_level=makoto_level,
        )

    def with_subject(self, name: str, sha256: str) -> AttestationBuilder:
        """Add a subject to the attestation.

        Args:
            name: Identifier for the subject
            sha256: SHA-256 hash of the subject

        Returns:
            Self for chaining
        """
        self._subjects.append(Subject.from_file(name, sha256))
        return self

    def with_subject_file(self, path: Path | str, *, name: str | None = None) -> AttestationBuilder:
        """Add a subject from a file.

        Args:
            path: Path to the file
            name: Optional name (defaults to filename)

        Returns:
            Self for chaining
        """
        path = Path(path)
        sha256 = compute_file_sha256(path)
        return self.with_subject(name=name or path.name, sha256=sha256)

    def with_subject_data(self, name: str, data: bytes) -> AttestationBuilder:
        """Add a subject from in-memory data.

        Args:
            name: Identifier for the subject
            data: The data to hash

        Returns:
            Self for chaining
        """
        sha256 = compute_sha256(data)
        return self.with_subject(name, sha256)

    def _build_origin_predicate(self, config: _OriginConfig) -> OriginPredicate:
        """Build an origin predicate from config."""
        timestamp = config.collection_timestamp or datetime.now(timezone.utc)

        return OriginPredicate(
            origin=Origin(
                source=config.source,
                collection_timestamp=timestamp,
                source_type=config.source_type,  # type: ignore[arg-type]
                collection_method=config.collection_method,  # type: ignore[arg-type]
                geography=config.geography,
            ),
            collector=Collector(
                id=config.collector_id,
                environment=config.environment,  # type: ignore[arg-type]
            ),
        )

    def _build_transform_predicate(self, config: _TransformConfig) -> TransformPredicate:
        """Build a transform predicate from config."""
        return TransformPredicate(
            inputs=config.inputs,
            transform=Transform(
                type=config.transform_type,
                name=config.transform_name,
                version=config.transform_version,
                description=config.transform_description,
            ),
            executor=Executor(
                id=config.executor_id,
                platform=config.platform,
                environment=config.environment,  # type: ignore[arg-type]
            ),
        )

    def _build_stream_window_predicate(self, config: _StreamWindowConfig) -> StreamWindowPredicate:
        """Build a stream window predicate from config."""
        return StreamWindowPredicate(
            stream=Stream(
                id=config.stream_id,
                source=config.stream_source,
                topic=config.topic,
            ),
            window=Window(
                type=config.window_type,  # type: ignore[arg-type]
                duration=config.window_duration,
                alignment=config.alignment,  # type: ignore[arg-type]
            ),
            integrity=Integrity(
                merkle_tree=MerkleTree(
                    algorithm=config.merkle_algorithm,  # type: ignore[arg-type]
                    leaf_count=config.leaf_count,
                    root=config.merkle_root,
                )
            ),
            collector=StreamCollector(id=config.collector_id),
        )

    def build(self) -> InTotoStatement:
        """Build the attestation statement.

        Returns:
            The complete in-toto statement

        Raises:
            ValueError: If required fields are missing
        """
        if self._config is None or self._predicate_type is None:
            raise ValueError(
                "No predicate configured. Call origin(), transform(), or stream_window() first."
            )

        if not self._subjects:
            raise ValueError(
                "No subjects configured. Call with_subject() or with_subject_file() first."
            )

        # Build the predicate based on type
        predicate: OriginPredicate | TransformPredicate | StreamWindowPredicate
        if isinstance(self._config, _OriginConfig):
            predicate = self._build_origin_predicate(self._config)
        elif isinstance(self._config, _TransformConfig):
            if not self._config.inputs:
                raise ValueError(
                    "Transform attestation requires at least one input. Call with_input() first."
                )
            predicate = self._build_transform_predicate(self._config)
        elif isinstance(self._config, _StreamWindowConfig):
            predicate = self._build_stream_window_predicate(self._config)
        else:
            raise ValueError(f"Unknown config type: {type(self._config)}")

        return InTotoStatement(
            subjects=self._subjects,
            predicate_type=self._predicate_type,
            predicate=predicate.model_dump(by_alias=True, exclude_none=True),
        )

    def reset(self) -> AttestationBuilder:
        """Reset the builder for reuse.

        Returns:
            Self for chaining
        """
        self._subjects = []
        self._config = None
        self._predicate_type = None
        return self
