"""Signing bundle structures for Makoto attestations.

This module defines the envelope format for signed attestations,
compatible with in-toto DSSE (Dead Simple Signing Envelope).
"""

from __future__ import annotations

import base64
from typing import Annotated, Literal

from pydantic import BaseModel, ConfigDict, Field

from ..attestation.statement import InTotoStatement

DSSE_PAYLOAD_TYPE = "application/vnd.in-toto+json"
"""The payload type for in-toto DSSE envelopes."""


class Signature(BaseModel):
    """A DSSE signature entry."""

    keyid: str | None = None
    sig: Annotated[str, Field(description="Base64-encoded signature")]


class SigningBundle(BaseModel):
    """Dead Simple Signing Envelope (DSSE) for attestations.

    This is the standard envelope format for signed in-toto attestations,
    containing the payload and one or more signatures.
    """

    model_config = ConfigDict(populate_by_name=True)

    payload_type: Annotated[
        Literal["application/vnd.in-toto+json"],
        Field(alias="payloadType", default=DSSE_PAYLOAD_TYPE),
    ] = DSSE_PAYLOAD_TYPE
    payload: Annotated[str, Field(description="Base64-encoded statement")]
    signatures: list[Signature]

    @classmethod
    def from_statement(
        cls,
        statement: InTotoStatement,
        signatures: list[tuple[str | None, bytes]],
    ) -> SigningBundle:
        """Create a bundle from a statement and signatures.

        Args:
            statement: The attestation statement
            signatures: List of (keyid, signature_bytes) tuples

        Returns:
            A new SigningBundle
        """
        payload_bytes = statement.to_json(indent=None).encode("utf-8")
        payload_b64 = base64.b64encode(payload_bytes).decode("ascii")

        return cls(
            payload=payload_b64,
            signatures=[
                Signature(keyid=keyid, sig=base64.b64encode(sig).decode("ascii"))
                for keyid, sig in signatures
            ],
        )

    def get_statement(self) -> InTotoStatement:
        """Extract the statement from the bundle.

        Returns:
            The decoded InTotoStatement
        """
        payload_bytes = base64.b64decode(self.payload)
        return InTotoStatement.from_json(payload_bytes)

    def to_json(self, *, indent: int | None = 2) -> str:
        """Serialize the bundle to JSON.

        Args:
            indent: JSON indentation level

        Returns:
            JSON string
        """
        return self.model_dump_json(by_alias=True, indent=indent)

    @classmethod
    def from_json(cls, data: str | bytes) -> SigningBundle:
        """Parse a bundle from JSON.

        Args:
            data: JSON string or bytes

        Returns:
            Parsed SigningBundle
        """
        return cls.model_validate_json(data)


class SigstoreCertificate(BaseModel):
    """Sigstore certificate information."""

    issuer: str | None = None
    subject: str | None = None
    certificate_pem: Annotated[str | None, Field(default=None, alias="certificatePem")] = None


class SigstoreVerificationMaterial(BaseModel):
    """Sigstore verification material."""

    model_config = ConfigDict(populate_by_name=True)

    certificate: SigstoreCertificate | None = None
    transparency_log_entry: Annotated[
        dict[str, object] | None, Field(default=None, alias="transparencyLogEntry")
    ] = None
    timestamp_verification_data: Annotated[
        dict[str, object] | None, Field(default=None, alias="timestampVerificationData")
    ] = None


class SignedAttestation(BaseModel):
    """A signed Makoto attestation with full verification material.

    This extends the DSSE bundle with sigstore-specific verification
    data for L2+ attestations.
    """

    model_config = ConfigDict(populate_by_name=True)

    bundle: SigningBundle
    sigstore_bundle: Annotated[
        dict[str, object] | None,
        Field(default=None, alias="sigstoreBundle", description="Raw sigstore bundle"),
    ] = None
    verification_material: Annotated[
        SigstoreVerificationMaterial | None,
        Field(default=None, alias="verificationMaterial"),
    ] = None

    def get_statement(self) -> InTotoStatement:
        """Extract the statement from the signed attestation.

        Returns:
            The decoded InTotoStatement
        """
        return self.bundle.get_statement()

    def to_json(self, *, indent: int | None = 2) -> str:
        """Serialize the signed attestation to JSON.

        Args:
            indent: JSON indentation level

        Returns:
            JSON string
        """
        return self.model_dump_json(by_alias=True, exclude_none=True, indent=indent)

    @classmethod
    def from_json(cls, data: str | bytes) -> SignedAttestation:
        """Parse a signed attestation from JSON.

        Args:
            data: JSON string or bytes

        Returns:
            Parsed SignedAttestation
        """
        return cls.model_validate_json(data)

    def save(self, path: str) -> None:
        """Save the signed attestation to a file.

        Args:
            path: Output file path
        """
        with open(path, "w") as f:
            f.write(self.to_json())

    @classmethod
    def load(cls, path: str) -> SignedAttestation:
        """Load a signed attestation from a file.

        Args:
            path: Input file path

        Returns:
            Loaded SignedAttestation
        """
        with open(path) as f:
            return cls.from_json(f.read())
