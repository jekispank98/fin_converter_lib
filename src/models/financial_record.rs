use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct FinancialRecord {
    pub date: String,
    pub amount: f64,
    pub description: String,
}

// Входные данные из CSV: поддержим разные названия колонок через alias
#[derive(Debug, Clone, Deserialize)]
struct RawRecord {
    #[serde(alias = "Date", alias = "Дата")]
    date: String,

    // Сумму читаем как String, затем нормализуем запятую -> точку
    #[serde(alias = "Amount", alias = "Сумма")]
    amount: String,

    #[serde(alias = "Description", alias = "Назначение платежа", alias = "Комментарий")]
    description: String,
}

impl TryFrom<RawRecord> for FinancialRecord {
    type Error = String;

    fn try_from(raw: RawRecord) -> Result<Self, Self::Error> {
        let amount = raw.amount.trim().replace(',', ".").parse::<f64>()
            .map_err(|e| format!("Не могу распарсить сумму `{}`: {}", raw.amount, e))?;
        Ok(FinancialRecord {
            date: raw.date.trim().to_string(),
            amount,
            description: raw.description.trim().to_string(),
        })
    }
}