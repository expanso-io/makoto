"""Makoto attestation models.

This package provides Pydantic models for all Makoto attestation types,
generated from the official JSON schemas at https://makoto.dev/schemas/.

Predicate Types:
    - Origin (https://makoto.dev/origin/v1): Data collection attestations
    - Transform (https://makoto.dev/transform/v1): Data processing attestations
    - StreamWindow (https://makoto.dev/stream-window/v1): Streaming data attestations
    - DBOM: Data Bill of Materials for complete lineage

Example:
    ```python
    from makoto.models import OriginPredicate, Origin, Collector
    from datetime import datetime, UTC

    predicate = OriginPredicate(
        origin=Origin(
            source="https://api.example.com/data",
            collection_timestamp=datetime.now(UTC),
        ),
        collector=Collector(id="my-collector"),
    )
    ```
"""

from .common import (
    CodeReference,
    ConsentInfo,
    DigestSet,
    DTACompliance,
    DTAProvenanceStandard,
    DTASourceStandard,
    DTAUseStandard,
    MakotoLevel,
    ResourceDescriptor,
    SubjectDigest,
    VersionInfo,
)
from .dbom import (
    DBOM,
    AccessControl,
    Compliance,
    Dataset,
    DatasetCreator,
    DatasetDigest,
    DBOMMetadata,
    DBOMVerification,
    Generator,
    LineageGraph,
    PrivacyAssessment,
    RegulatoryCompliance,
    Source,
    SourceContribution,
    SourceLicense,
    Transformation,
    Verifier,
)
from .origin import (
    CollectionMethod,
    Collector,
    Origin,
    OriginMetadata,
    OriginPredicate,
    Schema,
    SourceType,
)
from .stream_window import (
    Aggregates,
    Chain,
    HashAlgorithm,
    Integrity,
    MerkleTree,
    Statistics,
    Stream,
    StreamCollector,
    StreamMetadata,
    StreamVerification,
    StreamWindowPredicate,
    TimeAlignment,
    Window,
    WindowType,
)
from .transform import (
    Executor,
    IsolationLevel,
    Transform,
    TransformInput,
    TransformMetadata,
    TransformPredicate,
    Verification,
)

__all__ = [
    # Common types
    "CodeReference",
    "ConsentInfo",
    "DigestSet",
    "DTACompliance",
    "DTAProvenanceStandard",
    "DTASourceStandard",
    "DTAUseStandard",
    "MakotoLevel",
    "ResourceDescriptor",
    "SubjectDigest",
    "VersionInfo",
    # Origin
    "Collector",
    "CollectionMethod",
    "Origin",
    "OriginMetadata",
    "OriginPredicate",
    "Schema",
    "SourceType",
    # Transform
    "Executor",
    "IsolationLevel",
    "Transform",
    "TransformInput",
    "TransformMetadata",
    "TransformPredicate",
    "Verification",
    # Stream Window
    "Aggregates",
    "Chain",
    "HashAlgorithm",
    "Integrity",
    "MerkleTree",
    "Statistics",
    "Stream",
    "StreamCollector",
    "StreamMetadata",
    "StreamVerification",
    "StreamWindowPredicate",
    "TimeAlignment",
    "Window",
    "WindowType",
    # DBOM
    "AccessControl",
    "Compliance",
    "DBOM",
    "DBOMMetadata",
    "DBOMVerification",
    "Dataset",
    "DatasetCreator",
    "DatasetDigest",
    "Generator",
    "LineageGraph",
    "PrivacyAssessment",
    "RegulatoryCompliance",
    "Source",
    "SourceContribution",
    "SourceLicense",
    "Transformation",
    "Verifier",
]
