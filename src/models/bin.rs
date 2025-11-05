use crate::error::ParserError;
use crate::handler::Parser;
use crate::models::financial_record::FinancialRecord;
use bincode::{config, decode_from_std_read};
use std::io::BufRead;

pub struct BinParser;

impl<R: BufRead> Parser<R> for BinParser {
    type Item = FinancialRecord;
    type Error = ParserError;

    fn parse(&mut self, mut reader: R) -> Result<Vec<Self::Item>, Self::Error> {
        let config = config::standard();

        let mut records = Vec::new();
        loop {
            match decode_from_std_read::<FinancialRecord, _, R>(&mut reader, config) {
                Ok(record) => records.push(record),
                Err(e) => {
                    eprintln!(
                        "\nКритическая ошибка декодирования (прочитано {}): {}",
                        records.len(),
                        e
                    );
                    return Err(e.into());
                }
            }
        }
    }
}