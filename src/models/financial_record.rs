use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FinancialRecord {
    #[serde(rename = "TX_ID")]
    pub tx_id: i64,

    #[serde(rename = "TX_TYPE")]
    pub tx_type: String,

    #[serde(rename = "FROM_USER_ID")]
    pub from_user_id: i64,

    #[serde(rename = "TO_USER_ID")]
    pub to_user_id: i64,

    #[serde(rename = "AMOUNT")]
    pub amount: i64,

    #[serde(rename = "TIMESTAMP")]
    pub timestamp: i64,

    #[serde(rename = "STATUS")]
    pub status: String,

    #[serde(rename = "DESCRIPTION")]
    pub description: String,
}