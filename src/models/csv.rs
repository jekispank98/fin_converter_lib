use crate::error::ParserError;
use crate::handler::{Deserializer, Parser, Serializer};
use crate::models::financial_record::FinancialRecord;
use std::io::{BufRead, Read, Write};
use csv::{ReaderBuilder, Trim, WriterBuilder};

pub struct Csv;

impl<R: BufRead> Parser<R> for Csv {
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

impl<R: Read> Deserializer<R> for Csv {
    type Item = FinancialRecord;
    type Error = ParserError;

    fn deserialize(&self, reader: R) -> Result<Self::Item, ParserError> {
        let mut rdr = ReaderBuilder::new()
            .has_headers(true)
            .delimiter(b',')
            .trim(Trim::All)
            .from_reader(reader);

        if let Some(result) = rdr.deserialize().next() {
            let rec: FinancialRecord = result
                .map_err(|e| ParserError::Format(format!("CSV parse error: {}", e)))?;
            Ok(rec)
        } else {
            Err(ParserError::Format("CSV is empty, no record found".into()))
        }
    }
}

impl<W: Write> Serializer<W, FinancialRecord> for Csv {
    type Error = ParserError;

    fn serialize(&self, items: &[FinancialRecord], writer: W) -> Result<(), ParserError> {
        let mut wtr = WriterBuilder::new()
            .has_headers(true)
            .delimiter(b',')
            .from_writer(writer);

        for rec in items {
            wtr.serialize(rec)
                .map_err(|e| ParserError::Format(format!("CSV write error: {}", e)))?;
        }
        wtr.flush().map_err(ParserError::Io)?;
        Ok(())
    }
}