use std::io::BufRead;
use crate::handler::Parser;
use crate::models::financial_record::FinancialRecord;
use crate::error::ParserError;
pub struct Csv;

impl <R: BufRead> Parser<R> for Csv {
    type Item = FinancialRecord;
    type Error = ParserError;

    fn parse(&mut self, mut reader: R) -> Result<Vec<FinancialRecord>, ParserError> {
        let mut records = Vec::new();
        let mut line: String = String::new();

        while reader
        .read_line(&mut line)
        .map_err(ParserError::Io)? > 0
         {
             let parts: Vec<&str> = line.trim_end().split(',').collect();

            if parts.len() != 3 {
                return Err(ParserError::Format(format!(
                    "Неправильное число полей в CSV: `{}`",
                    line.trim_end()
                )));
        }
        let record = FinancialRecord {
                date: parts[0].to_string(),
                amount: parts[1]
                    .parse()
                    .map_err(|e| ParserError::Format(format!(
                        "Не могу распарсить сумму `{}`: {}",
                        parts[1], e
                    )))?,
                description: parts[2].to_string(),
            };

            records.push(record);
            println!("LINE: {line}" );
            line.clear(); 
        }

        Ok(records)
    }

}