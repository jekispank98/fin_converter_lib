use std::io::BufRead;
use crate::handler::Parser;
use crate::models::financial_record::FinancialRecord;
use crate::error::ParserError;
struct Csv;

impl <R: BufRead> Parser<R> for Csv {
    type Item = FinancialRecord;
    type Error = ParserError;

    fn parse(&mut self, reader: R) -> Result<Vec<Self::Item>, Self::Error> {
        let records = Vec::new();
    }
}