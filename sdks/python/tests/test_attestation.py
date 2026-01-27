"""Tests for attestation builder and verifier."""

import tempfile
from pathlib import Path

import pytest

from makoto import AttestationBuilder, AttestationVerifier, InTotoStatement, Subject
from makoto.models import origin, stream_window, transform


class TestSubject:
    """Tests for Subject class."""

    def test_from_file(self) -> None:
        subject = Subject.from_file("test.csv", "a" * 64)
        assert subject.name == "test.csv"
        assert subject.digest.sha256 == "a" * 64

    def test_serialization(self) -> None:
        subject = Subject.from_file("test.csv", "a" * 64)
        data = subject.model_dump(by_alias=True)
        restored = Subject.model_validate(data)
        assert restored.name == subject.name
        assert restored.digest.sha256 == subject.digest.sha256


class TestInTotoStatement:
    """Tests for InTotoStatement class."""

    def test_create_statement(self) -> None:
        statement = InTotoStatement(
            subjects=[Subject.from_file("test.csv", "a" * 64)],
            predicate_type=origin.PREDICATE_TYPE,
            predicate={"origin": {"source": "test"}, "collector": {"id": "test"}},
        )
        assert statement.type_ == "https://in-toto.io/Statement/v1"
        assert statement.predicate_type == origin.PREDICATE_TYPE

    def test_json_round_trip(self) -> None:
        statement = InTotoStatement(
            subjects=[Subject.from_file("test.csv", "a" * 64)],
            predicate_type=origin.PREDICATE_TYPE,
            predicate={"test": "value"},
        )
        json_str = statement.to_json()
        restored = InTotoStatement.from_json(json_str)
        assert restored.predicate_type == statement.predicate_type
        assert restored.predicate == statement.predicate

    def test_json_uses_aliases(self) -> None:
        statement = InTotoStatement(
            subjects=[Subject.from_file("test.csv", "a" * 64)],
            predicate_type="test",
            predicate={},
        )
        json_str = statement.to_json()
        assert '"_type"' in json_str
        assert '"predicateType"' in json_str
        assert '"subject"' in json_str


class TestAttestationBuilder:
    """Tests for AttestationBuilder class."""

    def test_origin_attestation(self) -> None:
        builder = AttestationBuilder()
        statement = (
            builder.origin(
                source="https://api.example.com/data",
                collector_id="test-collector",
                source_type="api",
            )
            .with_subject("output.csv", "a" * 64)
            .build()
        )
        assert statement.predicate_type == origin.PREDICATE_TYPE
        assert len(statement.subjects) == 1
        assert statement.predicate["origin"]["source"] == "https://api.example.com/data"

    def test_transform_attestation(self) -> None:
        builder = AttestationBuilder()
        statement = (
            builder.transform(
                transform_type="https://makoto.dev/transforms/filter",
                transform_name="Filter PII",
                executor_id="test-executor",
            )
            .with_input("input.csv", "a" * 64)
            .with_subject("output.csv", "b" * 64)
            .build()
        )
        assert statement.predicate_type == transform.PREDICATE_TYPE
        assert len(statement.predicate["inputs"]) == 1
        assert statement.predicate["transform"]["name"] == "Filter PII"

    def test_stream_window_attestation(self) -> None:
        builder = AttestationBuilder()
        statement = (
            builder.stream_window(
                stream_id="test-stream",
                stream_source="mqtt://localhost:1883",
                window_type="tumbling",
                window_duration="PT1M",
                merkle_algorithm="sha256",
                merkle_root="c" * 64,
                leaf_count=1000,
                collector_id="test-collector",
            )
            .with_subject("window-001", "d" * 64)
            .build()
        )
        assert statement.predicate_type == stream_window.PREDICATE_TYPE
        assert statement.predicate["integrity"]["merkleTree"]["leafCount"] == 1000

    def test_with_subject_file(self) -> None:
        with tempfile.NamedTemporaryFile(delete=False, suffix=".csv") as f:
            f.write(b"test data")
            temp_path = f.name

        try:
            builder = AttestationBuilder()
            statement = (
                builder.origin(
                    source="test",
                    collector_id="test",
                )
                .with_subject_file(temp_path)
                .build()
            )
            assert len(statement.subjects) == 1
            # Verify the hash is correct
            import hashlib

            expected_hash = hashlib.sha256(b"test data").hexdigest()
            assert statement.subjects[0].digest.sha256 == expected_hash
        finally:
            Path(temp_path).unlink()

    def test_build_without_predicate_raises(self) -> None:
        builder = AttestationBuilder()
        builder.with_subject("test", "a" * 64)
        with pytest.raises(ValueError, match="No predicate configured"):
            builder.build()

    def test_build_without_subjects_raises(self) -> None:
        builder = AttestationBuilder()
        builder.origin(source="test", collector_id="test")
        with pytest.raises(ValueError, match="No subjects configured"):
            builder.build()

    def test_transform_without_inputs_raises(self) -> None:
        builder = AttestationBuilder()
        builder.transform(
            transform_type="test",
            transform_name="Test",
            executor_id="test",
        )
        builder.with_subject("output", "a" * 64)
        with pytest.raises(ValueError, match="requires at least one input"):
            builder.build()

    def test_with_input_on_non_transform_raises(self) -> None:
        builder = AttestationBuilder()
        builder.origin(source="test", collector_id="test")
        with pytest.raises(ValueError, match="can only be used with transform"):
            builder.with_input("test", "a" * 64)

    def test_reset(self) -> None:
        builder = AttestationBuilder()
        builder.origin(source="test", collector_id="test")
        builder.with_subject("test", "a" * 64)
        builder.reset()

        # Should fail because state was reset
        with pytest.raises(ValueError, match="No predicate"):
            builder.build()


class TestAttestationVerifier:
    """Tests for AttestationVerifier class."""

    def test_verify_valid_origin(self) -> None:
        statement = InTotoStatement(
            subjects=[Subject.from_file("test.csv", "a" * 64)],
            predicate_type=origin.PREDICATE_TYPE,
            predicate={
                "origin": {
                    "source": "test",
                    "collectionTimestamp": "2025-01-01T00:00:00Z",
                },
                "collector": {"id": "test"},
            },
        )
        verifier = AttestationVerifier()
        result = verifier.verify(statement)
        assert result.valid
        assert result.makoto_level == "L1"
        assert len(result.errors) == 0

    def test_verify_invalid_origin_missing_fields(self) -> None:
        statement = InTotoStatement(
            subjects=[Subject.from_file("test.csv", "a" * 64)],
            predicate_type=origin.PREDICATE_TYPE,
            predicate={
                "origin": {},  # Missing required fields
                "collector": {},  # Missing id
            },
        )
        verifier = AttestationVerifier()
        result = verifier.verify(statement)
        assert not result.valid
        assert len(result.errors) > 0
        assert any("source" in e for e in result.errors)

    def test_verify_valid_transform(self) -> None:
        statement = InTotoStatement(
            subjects=[Subject.from_file("output.csv", "b" * 64)],
            predicate_type=transform.PREDICATE_TYPE,
            predicate={
                "inputs": [{"name": "input.csv", "digest": {"sha256": "a" * 64}}],
                "transform": {"type": "test", "name": "Test Transform"},
                "executor": {"id": "test"},
            },
        )
        verifier = AttestationVerifier()
        result = verifier.verify(statement)
        assert result.valid

    def test_verify_with_files(self) -> None:
        with tempfile.NamedTemporaryFile(delete=False, suffix=".csv") as f:
            f.write(b"test data")
            temp_path = f.name

        try:
            import hashlib

            expected_hash = hashlib.sha256(b"test data").hexdigest()

            statement = InTotoStatement(
                subjects=[Subject.from_file(Path(temp_path).name, expected_hash)],
                predicate_type=origin.PREDICATE_TYPE,
                predicate={
                    "origin": {"source": "test", "collectionTimestamp": "2025-01-01T00:00:00Z"},
                    "collector": {"id": "test"},
                },
            )

            verifier = AttestationVerifier()
            result = verifier.verify_with_files(
                statement,
                files={Path(temp_path).name: temp_path},
            )
            assert result.valid
            assert result.subjects_verified == 1
            assert result.all_subjects_verified
        finally:
            Path(temp_path).unlink()

    def test_verify_with_files_hash_mismatch(self) -> None:
        with tempfile.NamedTemporaryFile(delete=False, suffix=".csv") as f:
            f.write(b"test data")
            temp_path = f.name

        try:
            statement = InTotoStatement(
                subjects=[Subject.from_file(Path(temp_path).name, "b" * 64)],  # Wrong hash
                predicate_type=origin.PREDICATE_TYPE,
                predicate={
                    "origin": {"source": "test", "collectionTimestamp": "2025-01-01T00:00:00Z"},
                    "collector": {"id": "test"},
                },
            )

            verifier = AttestationVerifier()
            result = verifier.verify_with_files(
                statement,
                files={Path(temp_path).name: temp_path},
            )
            assert not result.valid
            assert result.subjects_verified == 0
            assert any("Hash mismatch" in e for e in result.errors)
        finally:
            Path(temp_path).unlink()

    def test_verify_unknown_predicate_type_warns(self) -> None:
        statement = InTotoStatement(
            subjects=[Subject.from_file("test.csv", "a" * 64)],
            predicate_type="https://example.com/unknown/v1",
            predicate={},
        )
        verifier = AttestationVerifier()
        result = verifier.verify(statement)
        assert any("Unknown predicate type" in w for w in result.warnings)
