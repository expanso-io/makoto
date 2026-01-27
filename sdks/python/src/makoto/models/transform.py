"""Makoto Transform Attestation Predicate v1.

This module defines the predicate schema for transform attestations,
documenting how data was processed and transformed.
"""

from __future__ import annotations

from datetime import datetime
from typing import Annotated, Literal

from pydantic import BaseModel, ConfigDict, Field

from .common import CodeReference, DigestSet, MakotoLevel, VersionInfo

PREDICATE_TYPE = "https://makoto.dev/transform/v1"
"""The predicate type URI for transform attestations."""


Environment = Literal["production", "staging", "development", "testing"]
"""Deployment environment."""

IsolationLevel = Literal["none", "process", "container", "vm", "tee", "hsm"]
"""Isolation level for the execution environment."""


class TransformInput(BaseModel):
    """An input dataset consumed by the transformation."""

    model_config = ConfigDict(populate_by_name=True)

    name: Annotated[str, Field(description="Identifier for the input dataset")]
    digest: DigestSet
    attestation_ref: Annotated[
        str | None,
        Field(
            default=None,
            alias="attestationRef",
            description="Reference to the attestation for this input",
        ),
    ] = None
    makoto_level: Annotated[MakotoLevel | None, Field(default=None, alias="makotoLevel")] = None


class Transform(BaseModel):
    """Details of the transformation operation."""

    type: Annotated[
        str,
        Field(
            description="URI identifying the transform type (e.g., https://makoto.dev/transforms/anonymization)"
        ),
    ]
    name: Annotated[str, Field(description="Human-readable name of the transformation")]
    version: Annotated[
        str | None, Field(default=None, description="Version of the transformation")
    ] = None
    description: Annotated[
        str | None,
        Field(default=None, description="Description of what the transformation does"),
    ] = None
    parameters: Annotated[
        dict[str, object] | None,
        Field(
            default=None,
            description="Configuration parameters used in the transformation",
        ),
    ] = None
    code_ref: Annotated[CodeReference | None, Field(default=None, alias="codeRef")] = None


class Executor(BaseModel):
    """Information about the system that executed the transformation."""

    id: Annotated[str, Field(description="Unique identifier for the executor")]
    platform: Annotated[
        str | None,
        Field(default=None, description="Platform name (e.g., expanso, spark, flink)"),
    ] = None
    version: VersionInfo | None = None
    environment: Annotated[
        Environment | None, Field(default=None, description="Deployment environment")
    ] = None
    isolation: Annotated[
        IsolationLevel | None,
        Field(default=None, description="Isolation level for the execution environment"),
    ] = None


class TransformMetadata(BaseModel):
    """Execution metrics and statistics."""

    invocation_id: Annotated[
        str | None,
        Field(
            default=None,
            alias="invocationId",
            description="Unique identifier for this invocation",
        ),
    ] = None
    started_on: Annotated[
        datetime | None,
        Field(
            default=None,
            alias="startedOn",
            description="When transformation started (RFC 3339)",
        ),
    ] = None
    finished_on: Annotated[
        datetime | None,
        Field(
            default=None,
            alias="finishedOn",
            description="When transformation completed (RFC 3339)",
        ),
    ] = None
    duration_seconds: Annotated[
        float | None,
        Field(
            default=None,
            alias="durationSeconds",
            ge=0,
            description="Total duration in seconds",
        ),
    ] = None
    records_input: Annotated[
        int | None,
        Field(
            default=None,
            alias="recordsInput",
            ge=0,
            description="Total records read from inputs",
        ),
    ] = None
    records_output: Annotated[
        int | None,
        Field(
            default=None,
            alias="recordsOutput",
            ge=0,
            description="Total records written to output",
        ),
    ] = None
    records_dropped: Annotated[
        int | None,
        Field(
            default=None,
            alias="recordsDropped",
            ge=0,
            description="Records dropped during transformation",
        ),
    ] = None
    records_modified: Annotated[
        int | None,
        Field(
            default=None,
            alias="recordsModified",
            ge=0,
            description="Records that were modified",
        ),
    ] = None
    bytes_input: Annotated[
        int | None,
        Field(default=None, alias="bytesInput", ge=0, description="Total bytes read"),
    ] = None
    bytes_output: Annotated[
        int | None,
        Field(default=None, alias="bytesOutput", ge=0, description="Total bytes written"),
    ] = None


class Verification(BaseModel):
    """Verification properties of the transformation."""

    input_hash_verified: Annotated[
        bool | None,
        Field(
            default=None,
            alias="inputHashVerified",
            description="Whether input hashes were verified before processing",
        ),
    ] = None
    transform_deterministic: Annotated[
        bool | None,
        Field(
            default=None,
            alias="transformDeterministic",
            description="Whether the transform is deterministic",
        ),
    ] = None
    output_reproducible: Annotated[
        bool | None,
        Field(
            default=None,
            alias="outputReproducible",
            description="Whether the output can be reproduced from inputs",
        ),
    ] = None


class TransformPredicate(BaseModel):
    """Makoto Transform Attestation Predicate v1.

    Documents how data was processed, including input datasets,
    transformation details, and execution metadata.

    Example:
        ```python
        predicate = TransformPredicate(
            inputs=[
                TransformInput(
                    name="dataset:raw_transactions",
                    digest=DigestSet(sha256="a1b2c3..."),
                    makoto_level="L2",
                )
            ],
            transform=Transform(
                type="https://makoto.dev/transforms/anonymization",
                name="PII Anonymization",
                version="1.0.0",
            ),
            executor=Executor(
                id="https://expanso.io/pipelines/cluster-01",
                platform="expanso",
                environment="production",
            ),
        )
        ```
    """

    model_config = ConfigDict(extra="allow", populate_by_name=True)

    inputs: Annotated[
        list[TransformInput],
        Field(min_length=1, description="Input datasets consumed by this transformation"),
    ]
    transform: Transform
    executor: Executor
    metadata: TransformMetadata | None = None
    verification: Verification | None = None
