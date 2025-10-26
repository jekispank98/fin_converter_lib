use std::io::{BufRead, Read, Write};

/** Parse (read) financial record */
pub trait Parser<R: BufRead> {
    type Item;
    type Error;
    fn parse(&mut self, reader: R) -> std::error::Result<Vec<Self::Item>, Self::Error>;
}

/** Deserialize a record from thread */
pub trait Deserializer<R: Read> {
    type Item;
    type Error;
    fn deserialize(&self, reader: R) -> Result<Self::Item, Self::Error>;
}

/** Serialize items to writer in given format */
pub trait Serializer<W: Write, T> {
    type Error;

    fn serialize(&self, items: &[T], writer: W) -> Result<(), Self::Error>;
}