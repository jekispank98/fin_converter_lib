use std::io;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum ParserError {
    Io(io::Error),
    Format(String),
    InvalidMagic([u8; 4]),
    InvalidRecordSize(u32),
    UnknownTxType(u8),
    UnknownStatus(u8),
    Utf8(FromUtf8Error),
}

impl From<io::Error> for ParserError {
    fn from(e: io::Error) -> Self {
        ParserError::Io(e)
    }
}
impl From<FromUtf8Error> for ParserError {
    fn from(e: FromUtf8Error) -> Self {
        ParserError::Utf8(e)
    }
}  