//! Structure of the financial record that used by parsers and serializers

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FinancialRecord {
    /// Transaction identifier (unique), 64-bit integer.
    #[serde(rename = "TX_ID")]
    pub tx_id: i64,
    /// Transaction type: “DEPOSIT” | ‘TRANSFER’ | “WITHDRAWAL”.
    #[serde(rename = "TX_TYPE")]
    pub tx_type: String,
    /// Sender user ID.
    #[serde(rename = "FROM_USER_ID")]
    pub from_user_id: i64,
    /// Recipient user ID.
    #[serde(rename = "TO_USER_ID")]
    pub to_user_id: i64,
    /// Amount in minimum units (example: cents). Can be negative
    /// for refunds/debits.
    #[serde(rename = "AMOUNT")]
    pub amount: i64,
    /// Unix time in seconds.
    #[serde(rename = "TIMESTAMP")]
    pub timestamp: i64,
    /// Status: “SUCCESS” | ‘FAILURE’ | “PENDING”.
    #[serde(rename = "STATUS")]
    pub status: String,
    /// Arbitrary description, UTF‑8.
    #[serde(rename = "DESCRIPTION")]
    pub description: String,
}