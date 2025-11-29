#![warn(missing_docs)]
//! Public interfaces for parsing, deserializing, and serializing financial records.
//!
//! This module exposes three core traits implemented by specific formats in
//! `models::{csv, text, bin}`:
//! - `Parser` — batch parsing from `Read` thread to `Vec` of records.
//! - `Deserializer` — readin a single record `Read`.
//! - `Serializer` — writing records to `Write` thread in specified format.

use std::io::{Read, Write};

/// Batch parser of financial records from a readable source.
/// `Parser` reads as many records as possible from the source passed to `reader`
/// and returns them as a list. A correct implementation should terminate
/// without error upon reaching EOF, returning the elements that have already been successfully parsed.
/// Errors indicate I/O problems or format mismatches.
pub trait Parser<R: Read> {
    /// The item (record) produced by the parser.
    type Item;
    /// The error type returned on failure.
    type Error;
    /// Parse all available records from `reader` and return them as a vector.
    fn parse(&mut self, reader: R) -> Result<Vec<Self::Item>, Self::Error>;
}

/// Deserializer of a single record from a readable source.
/// Use the trait when it's need to return exactly one logical record from 'Read' thread.
/// It may return error if list is empty or data isn't valid
pub trait Deserializer<R: Read> {
    /// The item (record) produced by the deserializer.
    type Item;
    /// The error type returned on failure.
    type Error;

    /// Read exactly one record from `reader`.
    fn deserialize(&self, reader: R) -> Result<Self::Item, Self::Error>;
}

/// Serializer of records to a writable destination in a specific format.
/// The implementation writes the transferred elements to the `writer` according to the format rules.
/// Unless otherwise specified, the implementation must write
/// all elements and flush buffers if necessary.

pub trait Serializer<W: Write, T> {
    /// The error type returned on failure.
    type Error;

    /// Serialize the slice of items into `writer`.
    /// The implementaion must write all records and return
    /// `Ok(())`, or return error and complete writing
    fn serialize(&self, items: &[T], writer: W) -> Result<(), Self::Error>;
}
