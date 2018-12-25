//! Error types
//!
//! The general strategy of this crate is to avoid sum error types, or if necessary to only include
//! the variants that are actually possible for that function.
//!

use std::fmt::Debug;

/// Error returned when the ID is invalid.
#[derive(Debug, Error)]
#[error(display = "ID {:?} is invalid", _0)]
pub struct InvalidIdError<I: Debug>(I);

/// Error returned when the operation failed because we ran out of the resources.
#[derive(Debug, Error)]
#[error(display = "ran out of memory / resources")]
pub struct OomError;
