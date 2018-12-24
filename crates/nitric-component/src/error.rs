use std::fmt::Debug;

/// Error returned when the ID is invalid.
#[derive(Debug, Error)]
#[error(display = "ID {:?} is invalid", _0)]
pub struct InvalidIdError<I: Debug>(I);

/// Error returned when the operation failed because we ran out of the resources.
#[derive(Debug, Error)]
#[error(display = "ran out of memory / resources")]
pub struct OomError;
