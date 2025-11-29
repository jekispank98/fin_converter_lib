//! Data models and format adapters for `fin_converter_lib`.
//!
//! This module groups the core data type and the format-specific adapters used
//! to parse, deserialize, and serialize financial records.
//!
//! Modules
//! - [`financial_record`]: defines the central [`FinancialRecord`] struct shared by all formats.
//! - [`csv`]: CSV reader/writer for `FinancialRecord`.
//! - [`text`]: YAML-like text reader/writer for `FinancialRecord`.
//! - [`bin`]: compact custom binary reader/writer for `FinancialRecord`.

pub mod csv;
pub mod financial_record;
pub mod text;
pub mod bin;