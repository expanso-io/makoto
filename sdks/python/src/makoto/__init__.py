"""Makoto Python SDK - Data integrity attestations for data pipelines.

Makoto brings SLSA-style assurance levels to data pipelines, producing
DBOMs (Data Bills of Materials) to prove chain of custody for your data.

Quick Start:
    ```python
    from makoto import AttestationBuilder
    from makoto.models import OriginPredicate

    # Create an origin attestation
    builder = AttestationBuilder()
    statement = (
        builder
        .origin(
            source="https://api.example.com/data",
            collector_id="my-collector",
            source_type="api",
        )
        .with_subject_file("data.csv")
        .build()
    )

    # Sign it (requires OIDC authentication)
    from makoto import AttestationSigner
    signer = AttestationSigner()
    result = signer.sign(statement)
    if result.success:
        result.signed_attestation.save("attestation.json")
    ```

Makoto Levels:
    - L1: Provenance Exists - Machine-readable attestation documents data origin
    - L2: Provenance is Authentic - Cryptographically signed, tamper-evident
    - L3: Provenance is Unforgeable - Platform-generated from isolated infrastructure

For more information, visit https://makoto.dev
"""

__version__ = "0.1.0"

from .attestation import (
    AttestationBuilder,
    AttestationVerifier,
    InTotoStatement,
    Subject,
    VerificationResult,
)
from .signing import AttestationSigner, SignedAttestation, SigningBundle, SigningResult

__all__ = [
    # Version
    "__version__",
    # Attestation
    "AttestationBuilder",
    "AttestationVerifier",
    "InTotoStatement",
    "Subject",
    "VerificationResult",
    # Signing
    "AttestationSigner",
    "SignedAttestation",
    "SigningBundle",
    "SigningResult",
]
