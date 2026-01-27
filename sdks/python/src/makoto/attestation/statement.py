"""In-toto statement structures for Makoto attestations.

This module provides the in-toto Statement wrapper that contains
Makoto predicates, following the in-toto attestation framework.
"""

from __future__ import annotations

from typing import Annotated, Any, Literal

from pydantic import BaseModel, ConfigDict, Field

from ..models.common import DigestSet

IN_TOTO_STATEMENT_TYPE = "https://in-toto.io/Statement/v1"
"""The type URI for in-toto Statement v1."""


class Subject(BaseModel):
    """An in-toto attestation subject.

    Identifies the artifact(s) this attestation is about.
    """

    name: Annotated[str, Field(description="Identifier for the subject")]
    digest: DigestSet

    @classmethod
    def from_file(cls, name: str, sha256: str) -> Subject:
        """Create a subject from a file hash.

        Args:
            name: Name or path of the file
            sha256: SHA-256 hash of the file contents

        Returns:
            A new Subject instance
        """
        return cls(name=name, digest=DigestSet(sha256=sha256))


class InTotoStatement(BaseModel):
    """In-toto Statement v1 wrapper for Makoto predicates.

    An in-toto statement binds a predicate (the attestation content)
    to one or more subjects (the artifacts being attested).

    Example:
        ```python
        from makoto.attestation import InTotoStatement, Subject
        from makoto.models import OriginPredicate, Origin, Collector, origin

        predicate = OriginPredicate(...)
        statement = InTotoStatement(
            subjects=[Subject.from_file("data.csv", "abc123...")],
            predicate_type=origin.PREDICATE_TYPE,
            predicate=predicate.model_dump(by_alias=True),
        )
        ```
    """

    model_config = ConfigDict(populate_by_name=True)

    type_: Annotated[
        Literal["https://in-toto.io/Statement/v1"],
        Field(alias="_type", default=IN_TOTO_STATEMENT_TYPE),
    ] = IN_TOTO_STATEMENT_TYPE
    subjects: Annotated[
        list[Subject],
        Field(alias="subject", min_length=1, description="The subjects of this attestation"),
    ]
    predicate_type: Annotated[
        str,
        Field(alias="predicateType", description="URI identifying the predicate schema"),
    ]
    predicate: Annotated[
        dict[str, Any],
        Field(description="The attestation predicate content"),
    ]

    def to_json(self, *, indent: int | None = 2) -> str:
        """Serialize the statement to JSON.

        Args:
            indent: JSON indentation level (None for compact)

        Returns:
            JSON string representation
        """
        return self.model_dump_json(by_alias=True, indent=indent)

    @classmethod
    def from_json(cls, data: str | bytes) -> InTotoStatement:
        """Parse a statement from JSON.

        Args:
            data: JSON string or bytes

        Returns:
            Parsed InTotoStatement
        """
        return cls.model_validate_json(data)
