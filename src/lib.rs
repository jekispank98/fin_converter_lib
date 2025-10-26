pub mod test;
pub mod error;
pub mod result;
pub mod handler;
pub mod models;

pub use models::csv::Csv;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

