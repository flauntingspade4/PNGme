#![allow(non_snake_case)]
pub mod chunk;
pub mod chunk_type;
pub mod png;

#[derive(Debug)]
pub enum Error {
	InvalidByte,
	FailedConversion,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidByte => write!(f, "The byte given was invalid ascii"),
            Error::FailedConversion => write!(f, "A conversion failed"),
        }
    }
}