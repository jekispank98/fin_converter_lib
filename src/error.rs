use std::{fmt, io};
use std::error::Error;
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

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::Io(e)               => write!(f, "I/O error: {}", e),
            ParserError::Format(msg)        => write!(f, "Format error: {}", msg),
            ParserError::InvalidMagic(m)     => write!(f, "Invalid magic: {:x?}", m),
            ParserError::InvalidRecordSize(n)=> write!(f, "Invalid record size: {}", n),
            ParserError::UnknownTxType(t)    => write!(f, "Unknown transaction type: {}", t),
            ParserError::UnknownStatus(s)    => write!(f, "Unknown status: {}", s),
            ParserError::Utf8(e)             => write!(f, "UTF-8 error: {}", e),
        }
    }
}

impl Error for ParserError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ParserError::Io(e)   => Some(e),
            ParserError::Utf8(e) => Some(e),
            _                    => None,
        }
    }
}

