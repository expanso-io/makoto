"""Cryptographic signing for Makoto attestations.

This package provides signing capabilities using sigstore for
keyless signing with OIDC identity, producing L2+ attestations.
"""

from .bundle import SignedAttestation, SigningBundle
from .signer import AttestationSigner, SigningResult

__all__ = [
    "AttestationSigner",
    "SignedAttestation",
    "SigningBundle",
    "SigningResult",
]
