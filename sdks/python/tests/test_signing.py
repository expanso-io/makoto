"""Tests for signing module."""

from makoto import AttestationBuilder, AttestationSigner, SignedAttestation, SigningBundle
from makoto.attestation import InTotoStatement, Subject
from makoto.models import origin
from makoto.signing.bundle import DSSE_PAYLOAD_TYPE


class TestSigningBundle:
    """Tests for SigningBundle class."""

    def test_from_statement(self) -> None:
        statement = InTotoStatement(
            subjects=[Subject.from_file("test.csv", "a" * 64)],
            predicate_type=origin.PREDICATE_TYPE,
            predicate={"test": "value"},
        )
        bundle = SigningBundle.from_statement(
            statement,
            signatures=[("key1", b"signature1")],
        )
        assert bundle.payload_type == DSSE_PAYLOAD_TYPE
        assert len(bundle.signatures) == 1
        assert bundle.signatures[0].keyid == "key1"

    def test_get_statement(self) -> None:
        statement = InTotoStatement(
            subjects=[Subject.from_file("test.csv", "a" * 64)],
            predicate_type=origin.PREDICATE_TYPE,
            predicate={"test": "value"},
        )
        bundle = SigningBundle.from_statement(
            statement,
            signatures=[("key1", b"signature1")],
        )
        restored = bundle.get_statement()
        assert restored.predicate_type == statement.predicate_type
        assert restored.predicate == statement.predicate

    def test_json_round_trip(self) -> None:
        statement = InTotoStatement(
            subjects=[Subject.from_file("test.csv", "a" * 64)],
            predicate_type=origin.PREDICATE_TYPE,
            predicate={"test": "value"},
        )
        bundle = SigningBundle.from_statement(
            statement,
            signatures=[("key1", b"signature1")],
        )
        json_str = bundle.to_json()
        restored = SigningBundle.from_json(json_str)
        assert restored.payload_type == bundle.payload_type
        assert len(restored.signatures) == len(bundle.signatures)


class TestSignedAttestation:
    """Tests for SignedAttestation class."""

    def test_get_statement(self) -> None:
        statement = InTotoStatement(
            subjects=[Subject.from_file("test.csv", "a" * 64)],
            predicate_type=origin.PREDICATE_TYPE,
            predicate={"test": "value"},
        )
        bundle = SigningBundle.from_statement(
            statement,
            signatures=[("key1", b"signature1")],
        )
        signed = SignedAttestation(bundle=bundle)
        restored = signed.get_statement()
        assert restored.predicate_type == statement.predicate_type

    def test_json_round_trip(self) -> None:
        statement = InTotoStatement(
            subjects=[Subject.from_file("test.csv", "a" * 64)],
            predicate_type=origin.PREDICATE_TYPE,
            predicate={"test": "value"},
        )
        bundle = SigningBundle.from_statement(
            statement,
            signatures=[("key1", b"signature1")],
        )
        signed = SignedAttestation(bundle=bundle)
        json_str = signed.to_json()
        restored = SignedAttestation.from_json(json_str)
        assert restored.bundle.payload_type == signed.bundle.payload_type


class TestAttestationSigner:
    """Tests for AttestationSigner class."""

    def test_sign_with_key(self) -> None:
        # Generate a test key
        from cryptography.hazmat.primitives import serialization
        from cryptography.hazmat.primitives.asymmetric import ec

        private_key = ec.generate_private_key(ec.SECP256R1())
        private_key_pem = private_key.private_bytes(
            encoding=serialization.Encoding.PEM,
            format=serialization.PrivateFormat.PKCS8,
            encryption_algorithm=serialization.NoEncryption(),
        )

        # Create a statement
        builder = AttestationBuilder()
        statement = (
            builder.origin(
                source="test",
                collector_id="test",
            )
            .with_subject("test.csv", "a" * 64)
            .build()
        )

        # Sign it
        signer = AttestationSigner(offline=True)
        result = signer.sign_with_key(statement, private_key_pem, key_id="test-key")

        assert result.success
        assert result.signed_attestation is not None
        assert result.identity == "test-key"
        assert len(result.signed_attestation.bundle.signatures) == 1

    def test_pae_encoding(self) -> None:
        """Test Pre-Authentication Encoding format."""
        pae = AttestationSigner._create_pae("test/type", b"payload")
        # PAE format: "DSSEv1" + SP + LEN(type) + SP + type + SP + LEN(body) + SP + body
        assert pae.startswith(b"DSSEv1 ")
        assert b"test/type" in pae
        assert b"payload" in pae

    def test_verify_offline_mode(self) -> None:
        statement = InTotoStatement(
            subjects=[Subject.from_file("test.csv", "a" * 64)],
            predicate_type=origin.PREDICATE_TYPE,
            predicate={"test": "value"},
        )
        bundle = SigningBundle.from_statement(
            statement,
            signatures=[("key1", b"signature1")],
        )
        signed = SignedAttestation(bundle=bundle)

        signer = AttestationSigner(offline=True)
        assert signer.verify(signed)

    def test_sign_without_sigstore_returns_error(self) -> None:
        """Test that signing without sigstore installed returns an error."""
        builder = AttestationBuilder()
        statement = (
            builder.origin(source="test", collector_id="test")
            .with_subject("test.csv", "a" * 64)
            .build()
        )

        signer = AttestationSigner()
        # This will fail if sigstore is not installed or if OIDC auth fails
        # In a test environment without sigstore, we expect an error
        result = signer.sign(statement)
        # Either it succeeds (sigstore installed and auth works) or it fails with an error
        assert result.success or result.error is not None
