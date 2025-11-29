//! Error types used by parsers and serializers in this crate.
//! The central error enum [`ParserError`] represents failures that can occur
//! while reading/writing data and interpreting it as one of the supported
//! formats (CSV, YAML-like text, or the custom binary format).
//! Notes
//! - `ParserError` implements [`std::fmt::Display`] and [`std::error::Error`].
//! - Conversions from underlying errors are provided via `From`:
//!   - [`std::io::Error`] → [`ParserError::Io`]
//!   - [`std::string::FromUtf8Error`] → [`ParserError::Utf8`]

use std::io;
use std::string::FromUtf8Error;
use thiserror::Error;

/// Error type shared by all parsers/deserializers/serializers in the crate.
/// Variants cover I/O failures, textual/structural format issues, and
/// binary-format specific problems. See the binary format documentation in
/// [`models::bin`](crate::models::bin) for details about magic headers,
/// record sizes, and enumeration codes.
#[derive(Error, Debug)]
pub enum ParserError {
    /// Underlying I/O error while reading from or writing to a stream.

    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    /// Human-readable format error description.
    #[error("Format error: {0}")]
    Format(String),
    /// Binary format: magic header does not match the expected value.
    /// The library expects the 4-byte header `0x59 0x50 0x42 0x4E` ("YPBN").
    #[error("Invalid magic: {0:02X?}")]
    InvalidMagic([u8; 4]),
    /// Binary format: the size field of a record is invalid or inconsistent.
    #[error("Invalid record size: {0}")]
    InvalidRecordSize(u32),
    /// Binary format: unknown transaction type code.
    /// Expected codes are 0 = DEPOSIT, 1 = TRANSFER, 2 = WITHDRAWAL.
    #[error("Unknown transaction type: {0}")]
    UnknownTxType(u8),
    /// Binary format: unknown status code.
    /// Expected codes are 0 = SUCCESS, 1 = FAILURE, 2 = PENDING.
    #[error("Unknown status: {0}")]
    UnknownStatus(u8),
    /// UTF-8 decoding failed (e.g., textual description contained invalid bytes).
    #[error("UTF-8 error: {0}")]
    Utf8(#[from] FromUtf8Error),
}

