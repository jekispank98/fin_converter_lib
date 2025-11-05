use std::io::{BufRead, BufReader};
use crate::error::ParserError;
use crate::handler::Parser;
use crate::models::financial_record::{FinancialRecord};

pub struct TxtParser;

impl<R: BufRead> Parser<R> for TxtParser {
    type Item = FinancialRecord;
    type Error = ParserError;

    fn parse(&mut self, reader: R) -> Result<Vec<Self::Item>, ParserError> {
        let rd = BufReader::new(reader);
        let mut records = Vec::new();
        let mut block = String::new();

        for line in rd.lines() {
            let line = line.map_err(ParserError::Io)?;
            // Начало нового блока
            if line.starts_with("# Record") {
                if !block.trim().is_empty() {
                    let rec: FinancialRecord = serde_yaml::from_str(&block)
                        .map_err(|e| ParserError::Format(format!("YAML parse error: {}", e)))?;
                    records.push(rec);
                    block.clear();
                }
                continue;
            }
            if line.starts_with('#') {
                continue;
            }
            block.push_str(&line);
            block.push('\n');
        }
        if !block.trim().is_empty() {
            let rec: FinancialRecord = serde_yaml::from_str(&block)
                .map_err(|e| ParserError::Format(format!("YAML parse error: {}", e)))?;
            records.push(rec);
        }

        Ok(records)
    }
}