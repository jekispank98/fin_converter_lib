use std::io::{BufRead, BufReader};
use crate::error::ParserError;
use crate::handler::Parser;
use crate::models::financial_record::FinancialRecord;

pub struct BinParser;

impl<R: BufRead> Parser<R> for BinParser {
    type Item = FinancialRecord;
    type Error = ParserError;

    fn parse(&mut self, mut reader: R) -> Result<Vec<Self::Item>, Self::Error> {
        let records: Vec<FinancialRecord> = bincode::decode_from_reader(&mut reader, /* config */)
            .map_err(|e| ParserError::Format(format!("BIN parse error: {}", e)))?;
        Ok(records)
    }
}