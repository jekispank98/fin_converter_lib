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

/*impl TryFrom<RawRecord> for FinancialRecord {
    type Error = String;

    fn try_from(raw: RawRecord) -> Result<Self, Self::Error> {
        let amount = raw.amount.trim().replace(',', ".").parse::<f64>()
            .map_err(|e| format!("Не могу распарсить сумму `{}`: {}", raw.amount, e))?;
        Ok(FinancialRecord {
            tx_id: 0,
            tx_type: "".to_string(),
            from_user_id: 0,
            date: raw.date.trim().to_string(),
            amount,
            timestamp: 0,
            status: "".to_string(),
            description: raw.description.trim().to_string(),
            to_user_id: 0
        })
    }
}*/