use crate::models::financial_record::FinancialRecord;
use crate::models::csv::Csv;
use crate::handler::{Serializer, Parser, Deserializer};
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
            tx_type: "TRANSFER".to_string(),
            from_user_id: 12,
            to_user_id: 13,
            amount: -200,
            timestamp: 1_700_000_100,
            status: "PENDING".to_string(),
            description: "world".to_string(),
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
fn csv_parse_multiple_records() {
    // Build CSV with header and two rows
    let csv_data = "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION\n\
1,DEPOSIT,10,11,1000,1700000000,SUCCESS,hello\n\
2,TRANSFER,12,13,-200,1700000100,PENDING,world\n";

    let mut csv = Csv;
    let cursor = Cursor::new(csv_data.as_bytes());

    let parsed = csv.parse(cursor).expect("csv parse should succeed");
    assert_eq!(parsed.len(), 2);
    let exp = sample_records();
    assert_eq_record(&parsed[0], &exp[0]);
    assert_eq_record(&parsed[1], &exp[1]);
}

#[test]
fn csv_deserialize_one_record() {
    let csv_one = "TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION\n\
1,DEPOSIT,10,11,1000,1700000000,SUCCESS,hello\n";
    let csv = Csv;
    let cursor = Cursor::new(csv_one.as_bytes());

    let rec = csv.deserialize(cursor).expect("csv one record");
    let exp = &sample_records()[0];
    assert_eq_record(&rec, exp);
}

#[test]
fn csv_serialize_roundtrip() {
    let records = sample_records();
    let csv = Csv;
    let mut out: Vec<u8> = Vec::new();
    csv.serialize(&records, &mut out).expect("csv serialize");

    // Now parse back
    let mut csv2 = Csv;
    let cursor = Cursor::new(out);
    let parsed = csv2.parse(cursor).expect("csv parse back");

    assert_eq!(parsed.len(), records.len());
    for (a, b) in parsed.iter().zip(records.iter()) {
        assert_eq_record(a, b);
    }
}
