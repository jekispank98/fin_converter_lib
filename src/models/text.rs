use std::io::{BufRead, BufReader, Read, Write};
use crate::error::ParserError;
use crate::handler::{Deserializer, Parser, Serializer};
use crate::models::financial_record::{FinancialRecord};

pub struct Txt;

impl<R: BufRead> Parser<R> for Txt {
    type Item = FinancialRecord;
    type Error = ParserError;

    fn parse(&mut self, reader: R) -> Result<Vec<Self::Item>, ParserError> {
        let rd = BufReader::new(reader);
        let mut records = Vec::new();
        let mut block = String::new();

        for line in rd.lines() {
            let line = line.map_err(ParserError::Io)?;
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

impl<R: Read> Deserializer<R> for Txt {
    type Item = FinancialRecord;
    type Error = ParserError;

    fn deserialize(&self, mut reader: R) -> Result<Self::Item, Self::Error> {
        let mut s = String::new();
        reader.read_to_string(&mut s)?;

        let yaml_only: String = s
            .lines()
            .filter(|l| !l.trim_start().starts_with('#'))
            .map(|l| format!("{}\n", l))
            .collect();

        let rec: FinancialRecord = serde_yaml::from_str(&yaml_only)
            .map_err(|e| ParserError::Format(format!("YAML parse error: {}", e)))?;
        Ok(rec)
    }
}

impl<W: Write> Serializer<W, FinancialRecord> for Txt {
    type Error = ParserError;

    fn serialize(&self, items: &[FinancialRecord], mut writer: W) -> Result<(), Self::Error> {
        for (idx, rec) in items.iter().enumerate() {
            writeln!(writer, "# Record {}", idx + 1)
                .map_err(ParserError::Io)?;
            serde_yaml::to_writer(&mut writer, rec)
                .map_err(|e| ParserError::Format(format!("YAML serialize error: {}", e)))?;
            writer.write_all(b"\n")
                .map_err(ParserError::Io)?;
        }
        Ok(())
    }
}