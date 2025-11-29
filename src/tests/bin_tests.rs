//! Binary format integration tests for `fin_converter_lib`.
//!
//! What is covered here:
//! - Serialize a list of `FinancialRecord` values to the custom binary format and parse them back
//! - Deserialize a single record from a binary buffer
//!
//! Invariants validated indirectly by round-trips:
//! - Magic header presence and consistency
//! - Record size encoding is stable
//! - Enumeration codes for `tx_type` and `status` map correctly during (de)serialization

use crate::handler::{Deserializer, Parser, Serializer};
use crate::models::bin::Bin;
use crate::models::financial_record::FinancialRecord;
use std::io::Cursor;
fn sample_records() -> Vec<FinancialRecord> {
    vec![
        FinancialRecord {
            tx_id: 100,
            tx_type: "DEPOSIT".to_string(),
            from_user_id: 1,
            to_user_id: 2,
            amount: 555,
            timestamp: 1_700_000_000,
            status: "SUCCESS".to_string(),
            description: "bin-one".to_string(),
        },
        FinancialRecord {
            tx_id: 101,
            tx_type: "TRANSFER".to_string(),
            from_user_id: 3,
            to_user_id: 4,
            amount: -42,
            timestamp: 1_700_000_123,
            status: "PENDING".to_string(),
            description: "bin-two".to_string(),
        },
    ]
}

fn assert_eq_record(a: &FinancialRecord, b: &FinancialRecord) {
    assert_eq!(a.tx_id, b.tx_id);
    assert_eq!(a.tx_type, b.tx_type);
    assert_eq!(a.from_user_id, b.from_user_id);
    assert_eq!(a.to_user_id, b.to_user_id);
    assert_eq!(a.amount, b.amount);
    assert_eq!(a.timestamp, b.timestamp);
    assert_eq!(a.status, b.status);
    assert_eq!(a.description, b.description);
}

#[test]
fn bin_serialize_and_parse_multiple() {
    let items = sample_records();
    let bin = Bin;
    let mut bytes: Vec<u8> = Vec::new();
    bin.serialize(&items, &mut bytes).expect("bin serialize");

    let mut bin2 = Bin;
    let cursor = Cursor::new(bytes);
    let parsed = bin2.parse(cursor).expect("bin parse");

    assert_eq!(parsed.len(), items.len());
    for (a, b) in parsed.iter().zip(items.iter()) {
        assert_eq_record(a, b);
    }
}

#[test]
fn bin_deserialize_single() {
    let item = sample_records().remove(0);
    let bin = Bin;
    let mut bytes: Vec<u8> = Vec::new();
    bin.serialize(&[item.clone()], &mut bytes)
        .expect("bin serialize one");

    let bin2 = Bin;
    let cursor = Cursor::new(bytes);
    let rec = bin2.deserialize(cursor).expect("bin deserialize");

    assert_eq_record(&rec, &item);
}
