use std::array::TryFromSliceError;
use std::convert::{TryFrom, TryInto};
use std::str::{from_utf8, Utf8Error};

use crate::chunk_type::ChunkType;
use crate::Error;

use crc::crc32::checksum_ieee;

#[derive(Debug)]
pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    chunk_data: Vec<u8>,
    crc: u32,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, chunk_data: Vec<u8>) -> Self {
        let crc = checksum_ieee(chunk_data.as_slice());
        Self {
            length: chunk_data.len() as u32,
            chunk_type,
            chunk_data,
            crc,
        }
    }
    pub fn length(&self) -> u32 {
        self.length
    }
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }
    pub fn data(&self) -> &Vec<u8> {
        &self.chunk_data
    }
    pub fn crc(&self) -> u32 {
        self.crc
    }
    pub fn data_as_string(&self) -> Result<&str, Utf8Error> {
        from_utf8(self.chunk_data.as_slice())
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut to_return = Vec::new();
        to_return.append(&mut self.length.to_be_bytes().to_vec());
        to_return.append(&mut self.chunk_type.bytes().to_vec());
        to_return.append(&mut self.chunk_data.clone());
        to_return.append(&mut self.crc.to_be_bytes().to_vec());
        to_return
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;
    fn try_from(mut value: &[u8]) -> Result<Self, Self::Error> {
        // Reads the input from the first 4 bytes of value to get the length of the data
        let length = read_be_u32(&mut value)?;

        // Reads the chunk_type from the next 4 bytes of value
        let chunk_type = match ChunkType::try_from([value[0], value[1], value[2], value[3]]) {
            Ok(t) => t,
            Err(_) => return Err(Error::InvalidByte),
        };

        // Removes the first 4 bytes, as they're used for the chunk_type
        value = value.split_at(std::mem::size_of::<u32>()).1;

        // Gets the last 4 bytes for the crc, and the remaining bytes for the data itself
        let (chunk_data, mut input) = value.split_at(value.len() - 4);
        let chunk_data = chunk_data.to_vec();
        let crc = read_be_u32(&mut input)?;

        Ok(Self {
            length,
            chunk_type,
            chunk_data,
            crc,
        })
    }
}

// Converts a &mut &[u8] to a Result<u32, ...>, and removes the first 4 bytes from the input
fn read_be_u32(input: &mut &[u8]) -> Result<u32, Error> {
    let (int_bytes, rest) = input.split_at(std::mem::size_of::<u32>());
    *input = rest;
    return match int_bytes.try_into() {
        Ok(t) => Ok(u32::from_be_bytes(t)),
        Err(_) => Err(Error::FailedConversion),
    }
}

#[allow(unused_variables)]
#[cfg(test)]
mod tests {
    use super::*;
    pub use crate::chunk_type::ChunkType;
    pub use std::str::FromStr;
    use crate::Error;

    fn testing_chunk() -> Result<Chunk, Error> {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref())
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk().unwrap();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk().unwrap();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk().unwrap();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk().unwrap();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(!chunk.is_err());
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{:?}", chunk);
    }
}
