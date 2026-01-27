"""Attestation verification utilities.

This module provides tools for verifying Makoto attestations,
including hash verification and chain validation.
"""

from __future__ import annotations

import hashlib
from dataclasses import dataclass, field
from datetime import datetime, timezone
from pathlib import Path
from typing import TYPE_CHECKING

from ..models import origin, stream_window, transform
from .statement import InTotoStatement

if TYPE_CHECKING:
    from ..models.common import MakotoLevel


@dataclass
class VerificationResult:
    """Result of attestation verification.

    Attributes:
        valid: Whether the verification passed
        predicate_type: The type of predicate verified
        makoto_level: Determined Makoto level (if applicable)
        subjects_verified: Number of subjects with verified hashes
        subjects_total: Total number of subjects
        errors: List of verification errors
        warnings: List of verification warnings
        verified_at: When verification was performed
    """

    valid: bool
    predicate_type: str | None = None
    makoto_level: MakotoLevel | None = None
    subjects_verified: int = 0
    subjects_total: int = 0
    errors: list[str] = field(default_factory=list)
    warnings: list[str] = field(default_factory=list)
    verified_at: datetime = field(default_factory=lambda: datetime.now(timezone.utc))

    @property
    def all_subjects_verified(self) -> bool:
        """Check if all subjects were verified."""
        return self.subjects_verified == self.subjects_total and self.subjects_total > 0


class AttestationVerifier:
    """Verifier for Makoto attestations.

    Provides verification of attestation structure, hash integrity,
    and optionally cryptographic signatures.

    Example:
        ```python
        verifier = AttestationVerifier()

        # Verify an attestation
        result = verifier.verify(statement)
        if result.valid:
            print(f"Attestation valid at level {result.makoto_level}")

        # Verify with file hash checking
        result = verifier.verify_with_files(
            statement,
            files={"data.csv": Path("./data.csv")},
        )
        ```
    """

    # Known predicate types
    KNOWN_PREDICATES = {
        origin.PREDICATE_TYPE,
        transform.PREDICATE_TYPE,
        stream_window.PREDICATE_TYPE,
    }

    def verify(self, statement: InTotoStatement) -> VerificationResult:
        """Verify an attestation statement structure.

        This performs structural validation without verifying file hashes.

        Args:
            statement: The attestation statement to verify

        Returns:
            Verification result
        """
        errors: list[str] = []
        warnings: list[str] = []

        # Check predicate type
        predicate_type = statement.predicate_type
        if predicate_type not in self.KNOWN_PREDICATES:
            warnings.append(f"Unknown predicate type: {predicate_type}")

        # Check subjects exist
        if not statement.subjects:
            errors.append("Attestation has no subjects")

        # Check subjects have digests
        for i, subject in enumerate(statement.subjects):
            if not subject.digest.sha256:
                errors.append(f"Subject {i} ({subject.name}) missing sha256 digest")

        # Validate predicate structure based on type
        if predicate_type == origin.PREDICATE_TYPE:
            errors.extend(self._verify_origin_predicate(statement.predicate))
        elif predicate_type == transform.PREDICATE_TYPE:
            errors.extend(self._verify_transform_predicate(statement.predicate))
        elif predicate_type == stream_window.PREDICATE_TYPE:
            errors.extend(self._verify_stream_window_predicate(statement.predicate))

        return VerificationResult(
            valid=len(errors) == 0,
            predicate_type=predicate_type,
            makoto_level="L1" if len(errors) == 0 else None,
            subjects_total=len(statement.subjects),
            errors=errors,
            warnings=warnings,
        )

    def verify_with_files(
        self,
        statement: InTotoStatement,
        files: dict[str, Path | str],
    ) -> VerificationResult:
        """Verify an attestation with file hash checking.

        Args:
            statement: The attestation statement to verify
            files: Mapping of subject names to file paths

        Returns:
            Verification result with hash verification
        """
        # First do structural verification
        result = self.verify(statement)
        errors = list(result.errors)
        warnings = list(result.warnings)
        subjects_verified = 0

        # Verify file hashes
        for subject in statement.subjects:
            if subject.name in files:
                file_path = Path(files[subject.name])
                if not file_path.exists():
                    errors.append(f"File not found: {file_path}")
                    continue

                expected_hash = subject.digest.sha256
                if expected_hash:
                    actual_hash = self._compute_file_hash(file_path)
                    if actual_hash == expected_hash:
                        subjects_verified += 1
                    else:
                        errors.append(
                            f"Hash mismatch for {subject.name}: "
                            f"expected {expected_hash[:16]}..., got {actual_hash[:16]}..."
                        )
            else:
                warnings.append(f"No file provided for subject: {subject.name}")

        return VerificationResult(
            valid=len(errors) == 0,
            predicate_type=result.predicate_type,
            makoto_level="L1" if len(errors) == 0 else None,
            subjects_verified=subjects_verified,
            subjects_total=len(statement.subjects),
            errors=errors,
            warnings=warnings,
        )

    def verify_chain(
        self,
        statements: list[InTotoStatement],
    ) -> VerificationResult:
        """Verify a chain of attestations.

        Checks that transform inputs reference outputs of previous attestations.

        Args:
            statements: Ordered list of attestation statements

        Returns:
            Verification result for the chain
        """
        errors: list[str] = []
        warnings: list[str] = []
        known_outputs: dict[str, str] = {}  # name -> sha256

        for i, statement in enumerate(statements):
            # Verify individual statement
            result = self.verify(statement)
            if not result.valid:
                errors.extend(f"Statement {i}: {e}" for e in result.errors)

            # Track outputs
            for subject in statement.subjects:
                if subject.digest.sha256:
                    known_outputs[subject.name] = subject.digest.sha256

            # For transforms, verify inputs are known
            if statement.predicate_type == transform.PREDICATE_TYPE:
                inputs = statement.predicate.get("inputs", [])
                for inp in inputs:
                    inp_name = inp.get("name", "")
                    inp_hash = inp.get("digest", {}).get("sha256", "")
                    if inp_name and inp_hash:
                        if inp_name in known_outputs:
                            if known_outputs[inp_name] != inp_hash:
                                errors.append(
                                    f"Statement {i}: Input {inp_name} hash mismatch "
                                    "with known output"
                                )
                        else:
                            warnings.append(
                                f"Statement {i}: Input {inp_name} not found in previous outputs"
                            )

        return VerificationResult(
            valid=len(errors) == 0,
            predicate_type="chain",
            makoto_level="L1" if len(errors) == 0 else None,
            subjects_total=len(statements),
            subjects_verified=len(statements) if len(errors) == 0 else 0,
            errors=errors,
            warnings=warnings,
        )

    def _verify_origin_predicate(self, predicate: dict[str, object]) -> list[str]:
        """Verify origin predicate structure."""
        errors: list[str] = []

        if "origin" not in predicate:
            errors.append("Origin predicate missing 'origin' field")
        else:
            origin_obj = predicate["origin"]
            if "source" not in origin_obj:
                errors.append("Origin missing 'source' field")
            if "collectionTimestamp" not in origin_obj:
                errors.append("Origin missing 'collectionTimestamp' field")

        if "collector" not in predicate:
            errors.append("Origin predicate missing 'collector' field")
        elif "id" not in predicate["collector"]:
            errors.append("Collector missing 'id' field")

        return errors

    def _verify_transform_predicate(self, predicate: dict[str, object]) -> list[str]:
        """Verify transform predicate structure."""
        errors: list[str] = []

        if "inputs" not in predicate:
            errors.append("Transform predicate missing 'inputs' field")
        elif not predicate["inputs"]:
            errors.append("Transform predicate has empty 'inputs' array")
        else:
            for i, inp in enumerate(predicate["inputs"]):
                if "name" not in inp:
                    errors.append(f"Transform input {i} missing 'name' field")
                if "digest" not in inp:
                    errors.append(f"Transform input {i} missing 'digest' field")

        if "transform" not in predicate:
            errors.append("Transform predicate missing 'transform' field")
        else:
            transform_obj = predicate["transform"]
            if "type" not in transform_obj:
                errors.append("Transform missing 'type' field")
            if "name" not in transform_obj:
                errors.append("Transform missing 'name' field")

        if "executor" not in predicate:
            errors.append("Transform predicate missing 'executor' field")
        elif "id" not in predicate["executor"]:
            errors.append("Executor missing 'id' field")

        return errors

    def _verify_stream_window_predicate(self, predicate: dict[str, object]) -> list[str]:
        """Verify stream window predicate structure."""
        errors: list[str] = []

        if "stream" not in predicate:
            errors.append("Stream window predicate missing 'stream' field")
        else:
            stream_obj = predicate["stream"]
            if "id" not in stream_obj:
                errors.append("Stream missing 'id' field")
            if "source" not in stream_obj:
                errors.append("Stream missing 'source' field")

        if "window" not in predicate:
            errors.append("Stream window predicate missing 'window' field")
        else:
            window_obj = predicate["window"]
            if "type" not in window_obj:
                errors.append("Window missing 'type' field")
            if "duration" not in window_obj:
                errors.append("Window missing 'duration' field")

        if "integrity" not in predicate:
            errors.append("Stream window predicate missing 'integrity' field")
        elif "merkleTree" not in predicate["integrity"]:
            errors.append("Integrity missing 'merkleTree' field")
        else:
            merkle = predicate["integrity"]["merkleTree"]
            if "algorithm" not in merkle:
                errors.append("MerkleTree missing 'algorithm' field")
            if "leafCount" not in merkle:
                errors.append("MerkleTree missing 'leafCount' field")
            if "root" not in merkle:
                errors.append("MerkleTree missing 'root' field")

        if "collector" not in predicate:
            errors.append("Stream window predicate missing 'collector' field")
        elif "id" not in predicate["collector"]:
            errors.append("Collector missing 'id' field")

        return errors

    @staticmethod
    def _compute_file_hash(path: Path) -> str:
        """Compute SHA-256 hash of a file."""
        h = hashlib.sha256()
        with open(path, "rb") as f:
            for chunk in iter(lambda: f.read(8192), b""):
                h.update(chunk)
        return h.hexdigest()
