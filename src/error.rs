use std::io;
use bincode::error::DecodeError;

#[derive(Debug)]
pub enum ParserError {
    Io(io::Error),
    Format(String),
}

impl From<io::Error> for ParserError {
    fn from(e: io::Error) -> Self {
        ParserError::Io(e)
    }
}

impl From<DecodeError> for ParserError {
    fn from(e: DecodeError) -> Self {
        match e {
            DecodeError::Io { inner: io_err, additional: _ } => {
                ParserError::Io(io_err)
            },

            // Все остальные ошибки формата
            _ => ParserError::Format(format!("Bincode Decoding Error: {}", e)),
        }
    }
}