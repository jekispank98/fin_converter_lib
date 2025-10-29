use std::io::BufRead;
use crate::handler::Parser;
use crate::models::financial_record::FinancialRecord;
use crate::error::ParserError;
pub struct CsvParser;

impl <R: BufRead> Parser<R> for CsvParser {
    type Item = FinancialRecord;
    type Error = ParserError;


    fn parse(&mut self, mut reader: R) -> Result<Vec<Self::Item>, ParserError> {
        let mut records = Vec::new();
        let mut line = String::new();
        let mut is_first_line = true;

        while reader.read_line(&mut line).map_err(ParserError::Io)? > 0 {
            let mut s = line.trim_end().to_string();

            if is_first_line {
                if s.starts_with('\u{feff}') {
                    s = s.trim_start_matches('\u{feff}').to_string();
                }
                is_first_line = false;
            }

            if s.is_empty() || s.chars().all(|c| c == ',') {
                line.clear();
                continue;
            }

            let parts: Vec<&str> = s.split(',').collect();

            if parts.len() != 3 {
                line.clear();
                return Err(ParserError::Format(format!(
                    "Неправильное число полей в CSV: `{}`",
                    s
                )));
            }

            let date = parts[0].trim().to_string();
            let amount_str = parts[1].trim();

            if amount_str.eq_ignore_ascii_case("amount") {
                line.clear();
                continue;
            }

            let amount = amount_str.parse().map_err(|e| {
                ParserError::Format(format!("Не могу распарсить сумму `{}`: {}", amount_str, e))
            })?;

            let description = parts[2].trim().to_string();

            let record = FinancialRecord {
                date,
                amount,
                description,
            };
            records.push(record);

            line.clear();
        }

        Ok(records)
    }
}