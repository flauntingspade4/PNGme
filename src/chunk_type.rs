use std::convert::{TryFrom, TryInto};
use std::{array::TryFromSliceError, str::FromStr};

use crate::Error;

#[derive(PartialEq, Debug)]
pub struct ChunkType {
    chunk_type: [u8; 4],
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.chunk_type
    }
    pub fn is_valid(&self) -> bool {
        self.chunk_type[2].is_ascii_uppercase()
    }
    pub fn is_critical(&self) -> bool {
        self.chunk_type[0].is_ascii_uppercase()
    }
    pub fn is_public(&self) -> bool {
        self.chunk_type[1].is_ascii_uppercase()
    }
    pub fn is_reserved_bit_valid(&self) -> bool {
        self.chunk_type[2].is_ascii_uppercase()
    }
    pub fn is_safe_to_copy(&self) -> bool {
        self.chunk_type[3].is_ascii_lowercase()
    }
    pub fn to_string(&self) -> &str {
        return std::str::from_utf8(&self.chunk_type).unwrap();
    }
}

impl FromStr for ChunkType {
    type Err = TryFromSliceError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chunk_type: [u8; 4] = s.as_bytes().try_into()?;
        Ok(Self { chunk_type })
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;
    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        for val in value.iter() {
            if !val.is_ascii_alphabetic() {
                return Err(Error::InvalidByte);
            }
        }
        Ok(Self { chunk_type: value })
    }
}

#[allow(unused_variables)]
#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t").unwrap();
        assert!(!chunk.is_valid());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), &"RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{:?}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
