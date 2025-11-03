use crate::error::ParserError;
use crate::handler::Parser;
use crate::models::financial_record::FinancialRecord;
use std::io::BufRead;
use csv::{ReaderBuilder, Trim};

pub struct CsvParser;

impl<R: BufRead> Parser<R> for CsvParser {
    type Item = FinancialRecord;
    type Error = ParserError;


    fn parse(&mut self, reader: R) -> Result<Vec<Self::Item>, ParserError> {
        let mut rdr = ReaderBuilder::new()
            .has_headers(true)
            .delimiter(b',')
            .trim(Trim::All)
            .from_reader(reader);

        let mut records = Vec::new();
        for result in rdr.deserialize() {
            let rec: FinancialRecord = result
                .map_err(|e| ParserError::Format(format!("CSV parse error: {}", e)))?;
            records.push(rec);
        }
        Ok(records)
    }
}

