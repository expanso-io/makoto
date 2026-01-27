//! Makoto attestation type definitions.
//!
//! This module contains all the type definitions for Makoto attestations,
//! matching the JSON schemas defined in the specification.

pub mod common;
pub mod dbom;
pub mod origin;
pub mod stream_window;
pub mod transform;

// Re-export commonly used types at the module level
pub use common::*;
pub use dbom::Dbom;
pub use origin::{OriginAttestation, OriginPredicate, ORIGIN_PREDICATE_TYPE};
pub use stream_window::{StreamWindowAttestation, StreamWindowPredicate, STREAM_WINDOW_PREDICATE_TYPE};
pub use transform::{TransformAttestation, TransformPredicate, TRANSFORM_PREDICATE_TYPE};
