//! Crate-wide `Result` alias.
//! Defaults error to [`ParserError`](crate::error::ParserError) to reduce typing.
//! Mirrors `std::result::Result`.

use crate::error::ParserError;
/// Convenience alias for results that by default use [`ParserError`].
pub type Result<T, E = ParserError> = std::result::Result<T, E>;