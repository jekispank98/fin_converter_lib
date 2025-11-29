//! YAML-like text format integration tests for `fin_converter_lib`.
//!
//! What is covered here:
//! - Parsing multiple records from a text stream; lines starting with `#` are treated as comments and ignored
//! - Deserializing a single record from a text buffer that may contain comments
//! - Round-trip: serialize records to text then parse back and compare field-by-field

use crate::handler::{Deserializer, Parser, Serializer};
use crate::models::financial_record::FinancialRecord;
use crate::models::text::Txt;
use std::io::Cursor;

fn sample_records() -> Vec<FinancialRecord> {
    vec![
        FinancialRecord {
            tx_id: 1,
            tx_type: "DEPOSIT".to_string(),
            from_user_id: 10,
            to_user_id: 11,
            amount: 1000,
            timestamp: 1_700_000_000,
            status: "SUCCESS".to_string(),
            description: "hello".to_string(),
        },
        FinancialRecord {
            tx_id: 2,
            tx_type: "WITHDRAWAL".to_string(),
            from_user_id: 12,
            to_user_id: 13,
            amount: -200,
            timestamp: 1_700_000_100,
            status: "FAILURE".to_string(),
            description: "bye".to_string(),
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
fn text_parse_multiple_records() {
    let data = "# Record 1\n\
TX_ID: 1\n\
TX_TYPE: DEPOSIT\n\
FROM_USER_ID: 10\n\
TO_USER_ID: 11\n\
AMOUNT: 1000\n\
TIMESTAMP: 1700000000\n\
STATUS: SUCCESS\n\
DESCRIPTION: hello\n\
# Record 2\n\
# a comment to ignore\n\
TX_ID: 2\n\
TX_TYPE: WITHDRAWAL\n\
FROM_USER_ID: 12\n\
TO_USER_ID: 13\n\
AMOUNT: -200\n\
TIMESTAMP: 1700000100\n\
STATUS: FAILURE\n\
DESCRIPTION: bye\n";

    let mut txt = Txt;
    let cursor = Cursor::new(data.as_bytes());
    let parsed = txt.parse(cursor).expect("text parse");

    assert_eq!(parsed.len(), 2);
    let exp = sample_records();
    assert_eq_record(&parsed[0], &exp[0]);
    assert_eq_record(&parsed[1], &exp[1]);
}

#[test]
fn text_deserialize_one_record_with_comments() {
    let data = "# header comment\n\
TX_ID: 1\n\
TX_TYPE: DEPOSIT\n\
FROM_USER_ID: 10\n\
TO_USER_ID: 11\n\
AMOUNT: 1000\n\
TIMESTAMP: 1700000000\n\
STATUS: SUCCESS\n\
DESCRIPTION: hello\n\
# footer comment\n";

    let txt = Txt;
    let cursor = Cursor::new(data.as_bytes());
    let rec = txt.deserialize(cursor).expect("text one record");

    assert_eq_record(&rec, &sample_records()[0]);
}

#[test]
fn text_serialize_roundtrip() {
    let records = sample_records();
    let txt = Txt;
    let mut out: Vec<u8> = Vec::new();
    txt.serialize(&records, &mut out).expect("text serialize");

    let mut txt2 = Txt;
    let cursor = Cursor::new(out);
    let parsed = txt2.parse(cursor).expect("text parse back");

    assert_eq!(parsed.len(), records.len());
    for (a, b) in parsed.iter().zip(records.iter()) {
        assert_eq_record(a, b);
    }
}
