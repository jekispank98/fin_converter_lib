//!Integration tests for `fin_converter_lib`.
//!
//! This module groups tests by supported formats:
//! - `csv_tests`: CSV parsing/serialization and error handling
//! - `bin_tests`: custom binary format read/write and validation
//! - `text_tests`: YAML-like text format parsing/serialization

#[cfg(test)]
pub mod csv_tests;

#[cfg(test)]
pub mod bin_tests;
#[cfg(test)]
pub mod text_tests;