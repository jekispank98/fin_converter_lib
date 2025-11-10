use crate::error::ParserError;
use crate::handler::{Deserializer, Parser, Serializer};
use crate::models::financial_record::FinancialRecord;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use std::fmt::format;
use std::io::{BufRead, ErrorKind, Read, Write};
/* Header */
const MAGIC_HEADER: [u8; 4] = [0x59, 0x50, 0x42, 0x4E];

/* Field's sizes*/
const SIZE_RECORD_SIZE: usize = 4;
const SIZE_TX_ID: usize = 8;
const SIZE_TX_TYPE: usize = 1;
const SIZE_USER_ID: usize = 8;
const SIZE_AMOUNT: usize = 8;
const SIZE_TIMESTAMP: usize = 8;
const SIZE_STATUS: usize = 1;
const SIZE_DESC_LEN: usize = 4;

/* Enum's codes */
const TX_TYPE_DEPOSIT: u8 = 0;
const TX_TYPE_TRANSFER: u8 = 1;
const TX_TYPE_WITHDRAWAL: u8 = 2;

/* Result's coddes */
const STATUS_SUCCESS: u8 = 0;
const STATUS_FAILURE: u8 = 1;
const STATUS_PENDING: u8 = 2;
pub struct Bin;

fn read_one_record<R: Read>(reader: &mut R) -> Result<FinancialRecord, ParserError> {
    let mut magic = [0u8; MAGIC_HEADER.len()];
    reader.read_exact(&mut magic)?;
    if magic != MAGIC_HEADER {
        return Err(ParserError::InvalidMagic(magic));
    }

    let _size = reader.read_u32::<BigEndian>()?;

    let tx_id = reader.read_u64::<BigEndian>()? as i64;
    let tx_type = match reader.read_u8()? {
        TX_TYPE_DEPOSIT => "DEPOSIT".to_string(),
        TX_TYPE_TRANSFER => "TRANSFER".to_string(),
        TX_TYPE_WITHDRAWAL => "WITHDRAWAL".to_string(),
        other => return Err(ParserError::UnknownTxType(other)),
    };

    let from_user_id = reader.read_u64::<BigEndian>()? as i64;
    let to_user_id = reader.read_u64::<BigEndian>()? as i64;
    let amount = reader.read_i64::<BigEndian>()?;
    let timestamp = reader.read_u64::<BigEndian>()? as i64;

    let status = match reader.read_u8()? {
        STATUS_SUCCESS => "SUCCESS".into(),
        STATUS_FAILURE => "FAILURE".into(),
        STATUS_PENDING => "PENDING".into(),
        other => return Err(ParserError::UnknownStatus(other)),
    };

    let desc_len = reader.read_u32::<BigEndian>()? as usize;
    let mut buf = vec![0u8; desc_len];
    reader.read_exact(&mut buf)?;
    let description = String::from_utf8(buf).map_err(|e| ParserError::Utf8(e))?;

    let financial_record = FinancialRecord {
        tx_id,
        tx_type,
        from_user_id,
        to_user_id,
        amount,
        timestamp,
        status,
        description,
    };
    print_one_record(&financial_record);
    Ok(financial_record)
}

impl<R: BufRead> Parser<R> for Bin {
    type Item = FinancialRecord;
    type Error = ParserError;

    fn parse(&mut self, mut reader: R) -> Result<Vec<Self::Item>, Self::Error> {
        let mut out = Vec::new();
        loop {
            match read_one_record(&mut reader) {
                Ok(rec) => out.push(rec),
                Err(ParserError::Io(ref e)) if e.kind() == ErrorKind::UnexpectedEof => break,
                Err(e) => return Err(e),
            }
        }
        Ok(out)
    }
}

impl<R: Read> Deserializer<R> for Bin {
    type Item = FinancialRecord;
    type Error = ParserError;

    fn deserialize(&self, mut reader: R) -> Result<Self::Item, Self::Error> {
        read_one_record(&mut reader)
    }
}

impl<W: Write> Serializer<W, FinancialRecord> for Bin {
    type Error = ParserError;

    fn serialize(&self, items: &[FinancialRecord], mut writer: W) -> Result<(), Self::Error> {
        for rec in items {
            writer.write_all(&MAGIC_HEADER)?;

            let desc = rec.description.as_bytes();
            let size = SIZE_TX_ID
                + SIZE_TX_TYPE
                + SIZE_USER_ID
                + SIZE_USER_ID
                + SIZE_AMOUNT
                + SIZE_TIMESTAMP
                + SIZE_STATUS
                + SIZE_DESC_LEN;
            writer.write_u32::<BigEndian>(size as u32)?;

            writer.write_u64::<BigEndian>(rec.tx_id as u64)?;
            let tt = match rec.tx_type.as_str() {
                "DEPOSIT" => 0,
                "TRANSFER" => 1,
                "WITHDRAWAL" => 2,
                o => return Err(ParserError::Format(format!("Unknown TX_TYPE: {}", o))),
            };
            writer.write_u8(tt)?;

            writer.write_u64::<BigEndian>(rec.from_user_id as u64)?;
            writer.write_u64::<BigEndian>(rec.to_user_id as u64)?;
            writer.write_i64::<BigEndian>(rec.amount)?;
            writer.write_u64::<BigEndian>(rec.timestamp as u64)?;

            let st = match rec.status.as_str() {
                "SUCCESS" => 0,
                "FAILURE" => 1,
                "PENDING" => 2,
                o => return Err(ParserError::Format(format!("Unknown STATUS: {}", o))),
            };
            writer.write_u8(st)?;

            writer.write_u32::<BigEndian>(desc.len() as u32)?;
            writer.write_all(desc)?;
        }
        Ok(())
    }
}

fn print_one_record(record: &FinancialRecord) {
    let pattern = format!(
        "Id: {}\nTx_type: {}\nFrom_user: {}\nTo_user: {}\nTimestamp: {}\nAmount: {}\nStatus: {}\nDescription: {}\n",
        record.tx_id,
        record.tx_type,
        record.from_user_id,
        record.to_user_id,
        format_timestamp_millis(record.timestamp),
        record.amount,
        record.status,
        record.description
    );
    println!("{}", pattern)
}

fn format_timestamp_millis(ts_millis: i64) -> String {
    let datetime: DateTime<Local> = Local
        .timestamp_millis_opt(ts_millis)
        .single()
        .expect("invalid timestamp");

    datetime.format("%Y-%m-%d %H:%M:%S%.3f").to_string()
}
