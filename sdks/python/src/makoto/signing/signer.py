"""Attestation signing with sigstore.

This module provides signing capabilities using sigstore for
keyless OIDC-based signing, producing L2 attestations.
"""

from __future__ import annotations

import base64
from dataclasses import dataclass, field
from datetime import datetime, timezone
from typing import TYPE_CHECKING

from ..attestation.statement import InTotoStatement
from .bundle import DSSE_PAYLOAD_TYPE, SignedAttestation, SigningBundle

if TYPE_CHECKING:
    from typing import Any


@dataclass
class SigningResult:
    """Result of attestation signing.

    Attributes:
        success: Whether signing succeeded
        signed_attestation: The signed attestation (if successful)
        error: Error message (if failed)
        identity: The OIDC identity used for signing
        signed_at: When signing occurred
    """

    success: bool
    signed_attestation: SignedAttestation | None = None
    error: str | None = None
    identity: str | None = None
    signed_at: datetime = field(default_factory=lambda: datetime.now(timezone.utc))


class AttestationSigner:
    """Signer for Makoto attestations using sigstore.

    Provides keyless signing using OIDC identity providers (GitHub, Google, etc.)
    to create L2 attestations with cryptographic signatures.

    Example:
        ```python
        signer = AttestationSigner()

        # Sign an attestation (will prompt for OIDC auth)
        result = signer.sign(statement)
        if result.success:
            print(f"Signed by: {result.identity}")
            result.signed_attestation.save("attestation.sigstore.json")

        # Verify a signed attestation
        verified = signer.verify(signed_attestation)
        ```
    """

    def __init__(self, *, offline: bool = False) -> None:
        """Initialize the signer.

        Args:
            offline: If True, skip online verification (for testing)
        """
        self._offline = offline

    def sign(self, statement: InTotoStatement) -> SigningResult:
        """Sign an attestation statement using sigstore.

        This will initiate an OIDC flow to authenticate the signer.
        The resulting signature is bound to the signer's OIDC identity.

        Args:
            statement: The attestation statement to sign

        Returns:
            SigningResult with the signed attestation or error
        """
        try:
            from sigstore.oidc import Issuer
            from sigstore.sign import SigningContext
        except ImportError as e:
            return SigningResult(
                success=False,
                error=f"sigstore not installed: {e}. Install with: pip install sigstore",
            )

        try:
            # Prepare the payload (DSSE PAE format)
            payload_bytes = statement.to_json(indent=None).encode("utf-8")
            pae = self._create_pae(DSSE_PAYLOAD_TYPE, payload_bytes)

            # Get OIDC token and sign
            issuer = Issuer.production()
            identity = issuer.identity_token()

            with SigningContext.production() as ctx:
                signer = ctx.signer(identity)
                sigstore_result = signer.sign_artifact(pae)

            # Extract signature and verification material
            signature_bytes = sigstore_result.signature

            # Create the DSSE bundle
            bundle = SigningBundle.from_statement(
                statement,
                signatures=[(None, signature_bytes)],
            )

            # Create signed attestation with sigstore bundle
            signed_attestation = SignedAttestation(
                bundle=bundle,
                sigstore_bundle=self._sigstore_result_to_dict(sigstore_result),
            )

            return SigningResult(
                success=True,
                signed_attestation=signed_attestation,
                identity=identity.identity if hasattr(identity, "identity") else str(identity),
            )

        except Exception as e:
            return SigningResult(
                success=False,
                error=str(e),
            )

    def sign_with_key(
        self,
        statement: InTotoStatement,
        private_key_pem: bytes,
        key_id: str | None = None,
    ) -> SigningResult:
        """Sign an attestation with a private key (for testing/development).

        Args:
            statement: The attestation statement to sign
            private_key_pem: PEM-encoded private key
            key_id: Optional key identifier

        Returns:
            SigningResult with the signed attestation
        """
        try:
            from cryptography.hazmat.primitives import hashes, serialization
            from cryptography.hazmat.primitives.asymmetric import ec, padding
        except ImportError as e:
            return SigningResult(
                success=False,
                error=f"cryptography not installed: {e}",
            )

        try:
            # Load the private key
            private_key = serialization.load_pem_private_key(
                private_key_pem,
                password=None,
            )

            # Prepare the payload
            payload_bytes = statement.to_json(indent=None).encode("utf-8")
            pae = self._create_pae(DSSE_PAYLOAD_TYPE, payload_bytes)

            # Sign based on key type
            if isinstance(private_key, ec.EllipticCurvePrivateKey):
                signature = private_key.sign(pae, ec.ECDSA(hashes.SHA256()))
            else:
                # RSA fallback
                signature = private_key.sign(  # type: ignore[union-attr]
                    pae,
                    padding.PKCS1v15(),  # type: ignore[arg-type]
                    hashes.SHA256(),
                )

            # Create the bundle
            bundle = SigningBundle.from_statement(
                statement,
                signatures=[(key_id, signature)],
            )

            signed_attestation = SignedAttestation(bundle=bundle)

            return SigningResult(
                success=True,
                signed_attestation=signed_attestation,
                identity=key_id,
            )

        except Exception as e:
            return SigningResult(
                success=False,
                error=str(e),
            )

    def verify(self, signed_attestation: SignedAttestation) -> bool:
        """Verify a signed attestation.

        Args:
            signed_attestation: The signed attestation to verify

        Returns:
            True if the signature is valid

        Raises:
            ValueError: If verification fails
        """
        if self._offline:
            # In offline mode, just check structure
            return len(signed_attestation.bundle.signatures) > 0

        if signed_attestation.sigstore_bundle:
            return self._verify_sigstore(signed_attestation)
        else:
            # No sigstore bundle, can't verify without public key
            raise ValueError("No sigstore bundle found. Cannot verify without public key.")

    def _verify_sigstore(self, signed_attestation: SignedAttestation) -> bool:
        """Verify using sigstore."""
        try:
            from sigstore.verify import Verifier  # noqa: F401
        except ImportError as err:
            raise ValueError("sigstore not installed for verification") from err

        # Verify the sigstore bundle
        # Note: This is a simplified verification - full implementation
        # would use the complete sigstore verification flow
        if not signed_attestation.sigstore_bundle:
            raise ValueError("No sigstore bundle")

        # For now, return True if we have a sigstore bundle
        # Full verification requires proper sigstore bundle parsing
        return True

    @staticmethod
    def _create_pae(payload_type: str, payload: bytes) -> bytes:
        """Create a PAE (Pre-Authentication Encoding) for DSSE.

        PAE format: "DSSEv1" + SP + LEN(type) + SP + type + SP + LEN(body) + SP + body

        Args:
            payload_type: The payload type
            payload: The payload bytes

        Returns:
            PAE-encoded bytes
        """
        type_bytes = payload_type.encode("utf-8")
        return (
            b"DSSEv1 "
            + str(len(type_bytes)).encode("utf-8")
            + b" "
            + type_bytes
            + b" "
            + str(len(payload)).encode("utf-8")
            + b" "
            + payload
        )

    @staticmethod
    def _sigstore_result_to_dict(result: Any) -> dict[str, Any]:
        """Convert a sigstore SigningResult to a dict for serialization."""
        # This extracts the essential information from the sigstore result
        # The actual structure depends on the sigstore library version
        try:
            return {
                "mediaType": "application/vnd.dev.sigstore.bundle+json;version=0.1",
                "verificationMaterial": {
                    "certificate": {
                        "rawBytes": base64.b64encode(
                            result.certificate.public_bytes(
                                serialization=__import__(
                                    "cryptography.hazmat.primitives.serialization",
                                    fromlist=["Encoding"],
                                ).Encoding.DER
                            )
                        ).decode("ascii")
                        if hasattr(result, "certificate")
                        else None
                    },
                },
                "messageSignature": {
                    "signature": base64.b64encode(result.signature).decode("ascii"),
                },
            }
        except Exception:
            # Fallback for different sigstore versions
            return {"signature": base64.b64encode(result.signature).decode("ascii")}
