# fin_converter_lib

A small Rust library for parsing, deserializing, and serializing financial transaction records in multiple formats (CSV, human-readable text/YAML, and a compact binary format).

It provides three simple traits:
- `Parser<R>` — read many records from a reader
- `Deserializer<R>` — read a single record from a reader
- `Serializer<W, T>` — write records to a writer

And ready-to-use implementations for a common `FinancialRecord` model:
- CSV: `models::csv::Csv`
- Text/YAML-with-comments: `models::text::Txt`
- Binary: `models::bin::Bin`

## Table of contents
- [Installation](#installation)
- [Data model](#data-model)
- [CSV format](#csv-format)
- [Text/YAML format](#textyaml-format)
- [Binary format](#binary-format)
- [Examples](#examples)
  - [Parse all records (CSV)](#parse-all-records-csv)
  - [Deserialize a single record (Text)](#deserialize-a-single-record-text)
  - [Serialize to CSV](#serialize-to-csv)
  - [Round-trip with the Binary format](#round-trip-with-the-binary-format)
- [Errors](#errors)
- [Notes](#notes)
- [Roadmap](#roadmap)

## Installation
Add to your `Cargo.toml`:

```toml
[dependencies]
fin_converter_lib = { path = "./fin_converter_lib" }
# or once published:
# fin_converter_lib = "0.1"
```

This crate uses Rust 2024 edition (see the `Cargo.toml`).

## Data model
The common record type is `models::financial_record::FinancialRecord`:

```rust,no_run
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FinancialRecord {
    #[serde(rename = "TX_ID")]        pub tx_id: i64,
    #[serde(rename = "TX_TYPE")]      pub tx_type: String,      // DEPOSIT | TRANSFER | WITHDRAWAL
    #[serde(rename = "FROM_USER_ID")] pub from_user_id: i64,
    #[serde(rename = "TO_USER_ID")]   pub to_user_id: i64,
    #[serde(rename = "AMOUNT")]       pub amount: i64,
    #[serde(rename = "TIMESTAMP")]    pub timestamp: i64,
    #[serde(rename = "STATUS")]       pub status: String,       // SUCCESS | FAILURE | PENDING
    #[serde(rename = "DESCRIPTION")]  pub description: String,
}
```

## CSV format
- Comma-separated with headers on the first line.
- Fields match the `serde`-renamed column names shown above.
- Trimming is enabled; headers must be present.

Example header line:
```
TX_ID,TX_TYPE,FROM_USER_ID,TO_USER_ID,AMOUNT,TIMESTAMP,STATUS,DESCRIPTION
```

## Text/YAML format
A human-friendly text file where YAML blocks describe records; lines starting with `#` are treated as comments and ignored during parsing. The helper usually writes a comment header `# Record N` before each YAML block.

Example:
```
# Record 1
TX_ID: 1001
TX_TYPE: DEPOSIT
FROM_USER_ID: 1
TO_USER_ID: 1
AMOUNT: 5000
TIMESTAMP: 1730000000
STATUS: SUCCESS
DESCRIPTION: Initial deposit
```
Multiple records can be concatenated, typically separated by a `# Record` comment line.

## Binary format
A compact big-endian binary format per record with a 4-byte magic header followed by fixed-size fields and a length-prefixed description.

Per record layout:
- Magic: `0x59 0x50 0x42 0x4E` (ASCII: `Y P B N`)
- `u32` size field (currently not validated against the rest of the record by the writer)
- `u64` TX_ID
- `u8`  TX_TYPE (0=DEPOSIT, 1=TRANSFER, 2=WITHDRAWAL)
- `u64` FROM_USER_ID
- `u64` TO_USER_ID
- `i64` AMOUNT
- `u64` TIMESTAMP
- `u8`  STATUS (0=SUCCESS, 1=FAILURE, 2=PENDING)
- `u32` DESCRIPTION length (bytes)
- `bytes` DESCRIPTION (UTF-8)

Parsing reads until EOF; an unexpected EOF cleanly ends the stream.

## Examples
Import the traits and formats you want to use:

```rust,ignore
use fin_converter_lib::handler::{Parser, Deserializer, Serializer};
use fin_converter_lib::models::{self, financial_record::FinancialRecord};
use std::fs::File;
use std::io::{BufReader, BufWriter};
```

### Parse all records (CSV)
```rust,no_run
use fin_converter_lib::models::csv::Csv;

let file = File::open("./data/records.csv")?;
let mut parser = Csv;
let records: Vec<FinancialRecord> = parser.parse(BufReader::new(file))?;
println!("parsed {} records", records.len());
```

### Deserialize a single record (Text)
```rust,no_run
use fin_converter_lib::models::text::Txt;

let file = File::open("./data/one_record.txt")?;
let rec: FinancialRecord = Txt.deserialize(BufReader::new(file))?;
println!("one record: {:?}", rec);
```

### Serialize to CSV
```rust,no_run
use fin_converter_lib::models::csv::Csv;

let records: Vec<FinancialRecord> = vec![/* ... */];
let file = File::create("./out/records.csv")?;
Csv.serialize(&records, BufWriter::new(file))?;
```

### Round-trip with the Binary format
```rust,no_run
use fin_converter_lib::models::bin::Bin;

let records: Vec<FinancialRecord> = vec![/* ... */];
let bin_file = File::create("./out/records.bin")?;
Bin.serialize(&records, BufWriter::new(bin_file))?;

let bin_in = File::open("./out/records.bin")?;
let mut bin_parser = Bin;
let read_back: Vec<FinancialRecord> = bin_parser.parse(BufReader::new(bin_in))?;
assert_eq!(read_back.len(), records.len());
```

## Errors
All operations return `Result<_, error::ParserError>`. Possible variants include:
- `Io(io::Error)` — I/O failures
- `Format(String)` — formatting/validation issues (e.g., malformed CSV/YAML, unknown field values)
- `InvalidMagic([u8; 4])`, `InvalidRecordSize(u32)` — binary format errors
- `UnknownTxType(u8)`, `UnknownStatus(u8)` — binary enum mapping errors
- `Utf8(FromUtf8Error)` — description decoding in binary format

## Notes
- The parser traits are generic over any `Read`/`Write` so you can use files, in-memory buffers, network streams, etc.
- CSV reader expects headers and uses comma as a delimiter.
- Text/YAML deserializer strips comment lines (`# ...`).
- Binary writer currently writes a constant size header that reflects fixed parts; the description length is written separately and not included in the size field.

## Roadmap
- Optional field validation and normalization utilities
- Configurable CSV dialect (custom delimiter, headers on/off)
- Streaming serializers/deserializers for very large datasets
- Publish to crates.io and add examples as a workspace `examples/` crate

---

If you spot an issue or want to contribute, please open an issue or a pull request. Thank you!