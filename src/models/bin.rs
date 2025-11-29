//! Custom binary format adapter for `FinancialRecord` (Big-Endian).

use crate::error::ParserError;
use crate::handler::{Deserializer, Parser, Serializer};
use crate::models::financial_record::FinancialRecord;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::io::{ErrorKind, Read, Write};

/// - Magic header: `59 50 42 4E` (ASCII "YPBN"), 4 bytes.
const MAGIC_HEADER: [u8; 4] = [0x59, 0x50, 0x42, 0x4E];

/// - TX_ID: `i64` (written as `u64` for non-negative, read as `u64` then cast to `i64`), 8 bytes.
const SIZE_TX_ID: usize = 8;


/// - TX_TYPE: `u8` code — 0 = DEPOSIT, 1 = TRANSFER, 2 = WITHDRAWAL, 1 byte.
const SIZE_TX_TYPE: usize = 1;

/// Size of a single user ID field (`FROM_USER_ID`/`TO_USER_ID`), 8 bytes (`u64` Big-Endian).
const SIZE_USER_ID: usize = 8;

/// Size of the `AMOUNT` field, 8 bytes (`i64` Big-Endian).
const SIZE_AMOUNT: usize = 8;

/// Size of the `TIMESTAMP` field, 8 bytes (`u64` Big-Endian).
const SIZE_TIMESTAMP: usize = 8;
/// Size of the `STATUS` enum code, 1 byte (`u8`).
const SIZE_STATUS: usize = 1;

/// Size of the `DESC_LEN` field, 4 bytes (`u32` Big-Endian). It specifies the length
/// of the following `DESCRIPTION` byte sequence and does not include itself.
const SIZE_DESC_LEN: usize = 4;

/// Transaction type code for `DEPOSIT`.
const TX_TYPE_DEPOSIT: u8 = 0;
/// Transaction type code for `TRANSFER`.
const TX_TYPE_TRANSFER: u8 = 1;
/// Transaction type code for `WITHDRAWAL`.
const TX_TYPE_WITHDRAWAL: u8 = 2;


/// Status code for `SUCCESS`.
const STATUS_SUCCESS: u8 = 0;
/// Status code for `FAILURE`.
const STATUS_FAILURE: u8 = 1;
/// Status code for `PENDING`.
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
    Ok(financial_record)
}

impl<R: Read> Parser<R> for Bin {
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
