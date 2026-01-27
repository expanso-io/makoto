"""Tests for Makoto models."""

from datetime import datetime, timezone

import pytest

from makoto.models import (
    DBOM,
    Collector,
    Dataset,
    DatasetDigest,
    DigestSet,
    Executor,
    Integrity,
    MerkleTree,
    Origin,
    OriginPredicate,
    Source,
    Stream,
    StreamCollector,
    StreamWindowPredicate,
    Transform,
    TransformInput,
    TransformPredicate,
    Window,
)


class TestDigestSet:
    """Tests for DigestSet model."""

    def test_sha256_only(self) -> None:
        digest = DigestSet(sha256="a" * 64)
        assert digest.sha256 == "a" * 64
        assert digest.sha384 is None
        assert digest.sha512 is None

    def test_all_algorithms(self) -> None:
        digest = DigestSet(
            sha256="a" * 64,
            sha384="b" * 96,
            sha512="c" * 128,
        )
        assert digest.sha256 == "a" * 64
        assert digest.sha384 == "b" * 96
        assert digest.sha512 == "c" * 128

    def test_invalid_sha256_length(self) -> None:
        with pytest.raises(ValueError):
            DigestSet(sha256="too_short")

    def test_serialization(self) -> None:
        digest = DigestSet(sha256="a" * 64)
        json_str = digest.model_dump_json()
        restored = DigestSet.model_validate_json(json_str)
        assert restored.sha256 == digest.sha256


class TestOriginPredicate:
    """Tests for OriginPredicate model."""

    def test_minimal(self) -> None:
        predicate = OriginPredicate(
            origin=Origin(
                source="https://api.example.com/data",
                collection_timestamp=datetime(2025, 1, 1, tzinfo=timezone.utc),
            ),
            collector=Collector(id="test-collector"),
        )
        assert predicate.origin.source == "https://api.example.com/data"
        assert predicate.collector.id == "test-collector"

    def test_full(self) -> None:
        predicate = OriginPredicate(
            origin=Origin(
                source="https://api.example.com/data",
                source_type="api",
                collection_method="scheduled-pull",
                collection_timestamp=datetime(2025, 1, 1, tzinfo=timezone.utc),
                geography="US-WEST-2",
            ),
            collector=Collector(
                id="test-collector",
                environment="production",
            ),
        )
        assert predicate.origin.source_type == "api"
        assert predicate.origin.collection_method == "scheduled-pull"
        assert predicate.collector.environment == "production"

    def test_json_serialization_uses_aliases(self) -> None:
        predicate = OriginPredicate(
            origin=Origin(
                source="test",
                source_type="api",
                collection_timestamp=datetime(2025, 1, 1, tzinfo=timezone.utc),
            ),
            collector=Collector(id="test"),
        )
        data = predicate.model_dump(by_alias=True)
        assert "sourceType" in data["origin"]
        assert "collectionTimestamp" in data["origin"]


class TestTransformPredicate:
    """Tests for TransformPredicate model."""

    def test_minimal(self) -> None:
        predicate = TransformPredicate(
            inputs=[
                TransformInput(
                    name="input.csv",
                    digest=DigestSet(sha256="a" * 64),
                )
            ],
            transform=Transform(
                type="https://makoto.dev/transforms/filter",
                name="Filter Records",
            ),
            executor=Executor(id="test-executor"),
        )
        assert len(predicate.inputs) == 1
        assert predicate.transform.name == "Filter Records"

    def test_multiple_inputs(self) -> None:
        predicate = TransformPredicate(
            inputs=[
                TransformInput(
                    name="input1.csv",
                    digest=DigestSet(sha256="a" * 64),
                    makoto_level="L2",
                ),
                TransformInput(
                    name="input2.csv",
                    digest=DigestSet(sha256="b" * 64),
                    makoto_level="L1",
                ),
            ],
            transform=Transform(
                type="https://makoto.dev/transforms/join",
                name="Join Datasets",
            ),
            executor=Executor(id="test-executor"),
        )
        assert len(predicate.inputs) == 2
        assert predicate.inputs[0].makoto_level == "L2"

    def test_requires_inputs(self) -> None:
        with pytest.raises(ValueError):
            TransformPredicate(
                inputs=[],
                transform=Transform(
                    type="https://makoto.dev/transforms/filter",
                    name="Filter",
                ),
                executor=Executor(id="test"),
            )


class TestStreamWindowPredicate:
    """Tests for StreamWindowPredicate model."""

    def test_minimal(self) -> None:
        predicate = StreamWindowPredicate(
            stream=Stream(
                id="test-stream",
                source="mqtt://localhost:1883",
            ),
            window=Window(
                type="tumbling",
                duration="PT1M",
            ),
            integrity=Integrity(
                merkle_tree=MerkleTree(
                    algorithm="sha256",
                    leaf_count=1000,
                    root="a" * 64,
                )
            ),
            collector=StreamCollector(id="test-collector"),
        )
        assert predicate.stream.id == "test-stream"
        assert predicate.window.type == "tumbling"
        assert predicate.integrity.merkle_tree.leaf_count == 1000

    def test_sliding_window(self) -> None:
        predicate = StreamWindowPredicate(
            stream=Stream(id="test", source="kafka://localhost:9092"),
            window=Window(
                type="sliding",
                duration="PT5M",
                slide="PT1M",
                alignment="event-time",
            ),
            integrity=Integrity(
                merkle_tree=MerkleTree(
                    algorithm="sha256",
                    leaf_count=5000,
                    root="b" * 64,
                )
            ),
            collector=StreamCollector(id="test"),
        )
        assert predicate.window.type == "sliding"
        assert predicate.window.slide == "PT1M"


class TestDBOM:
    """Tests for DBOM model."""

    def test_minimal(self) -> None:
        dbom = DBOM(
            dbom_id="urn:uuid:12345678-1234-1234-1234-123456789012",
            dataset=Dataset(
                name="test-dataset",
                digest=DatasetDigest(sha256="a" * 64),
            ),
            sources=[
                Source(
                    name="source-1",
                    attestation_ref="https://registry.example.com/att/1",
                )
            ],
        )
        assert dbom.dbom_version == "1.0.0"
        assert dbom.dataset.name == "test-dataset"
        assert len(dbom.sources) == 1

    def test_requires_sources(self) -> None:
        with pytest.raises(ValueError):
            DBOM(
                dbom_id="urn:uuid:12345678-1234-1234-1234-123456789012",
                dataset=Dataset(
                    name="test",
                    digest=DatasetDigest(sha256="a" * 64),
                ),
                sources=[],
            )
