use std::io;

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