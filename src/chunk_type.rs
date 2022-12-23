use std::{str::{self, FromStr}, fmt::Display};

#[derive(PartialEq, Eq, Debug)]
struct ChunkType {
    type_code: [u8; 4]
}

const BIT_MASK: u8 = 0b0010_0000; // testing bit 5 of each byte
const INVALID_BYTES_MSG: &'static str = "Bytes must represent valid uppercase or lowercase ASCII letters!";

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        if !bytes_are_valid(value) {
            return Err(INVALID_BYTES_MSG);
        }

        if let Ok(_) = str::from_utf8(&value) {
            return Ok(ChunkType {type_code: value});
        }

        Err("Failure in parsing UTF-8!")
    }
}

fn bytes_are_valid(bytes: [u8; 4]) -> bool {
    bytes.iter().all(|x| x.is_ascii_alphabetic())
}

impl FromStr for ChunkType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(type_cd_val) = s.as_bytes().try_into() {
            if !bytes_are_valid(type_cd_val) {
                return Err(INVALID_BYTES_MSG);
            }

            return Ok (
                ChunkType { 
                    type_code: type_cd_val
                }
            );
        }
        Err("Unable to convert string! Ensure string is 4 bytes!")
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", str::from_utf8(&self.type_code).unwrap())
    }
}

impl ChunkType {
    fn bytes(&self) -> [u8; 4] {
        self.type_code
    }

    fn is_critical(&self) -> bool {
        self.type_code[0] & BIT_MASK == 0u8
    }

    fn is_public(&self) -> bool {
        self.type_code[1] & BIT_MASK == 0u8
    }

    fn is_reserved_bit_valid(&self) -> bool {
        self.type_code[2] & BIT_MASK == 0u8
    }

    fn is_safe_to_copy(&self) -> bool {
        self.type_code[3] & BIT_MASK == BIT_MASK
    }

    fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }
}

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
    pub fn test_chunk_type_from_bytes_invalid_utf8_err() {
        let bad_utf8: [u8; 4] = [0, 159, 146, 150];
        let actual = ChunkType::try_from(bad_utf8);

        assert!(actual.is_err());
        assert_eq!(Some(INVALID_BYTES_MSG), actual.err());
    }

    #[test]
    pub fn test_chunk_type_from_bytes_invalid_ascii_err() {
        let bad_ascii: [u8; 4] = [240, 159, 153, 136];
        let actual = ChunkType::try_from(bad_ascii);

        assert!(actual.is_err());
        assert_eq!(Some(INVALID_BYTES_MSG), actual.err());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_from_str_invalid_ascii_err() {
        let bad_ascii: &str = str::from_utf8(&[240, 159, 153, 136]).unwrap();
        let actual = ChunkType::from_str(bad_ascii);

        assert!(actual.is_err());
        assert_eq!(Some(INVALID_BYTES_MSG), actual.err());
    }

    #[test]
    pub fn test_chunk_type_from_str_invalid_not_four_bytes_err() {
        let bad_ascii: &str = str::from_utf8(&[82, 117, 83, 116, 127, 22]).unwrap();
        let actual = ChunkType::from_str(bad_ascii);
        let expected_msg = "Unable to convert string! Ensure string is 4 bytes!";

        assert!(actual.is_err());
        assert_eq!(Some(expected_msg), actual.err());
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
    pub fn test_chunk_type_is_not_critical_pngspec() {
        let chunk = ChunkType::from_str("bLOb").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_public_pngspec() {
        let chunk = ChunkType::from_str("bLOb").unwrap();
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
    pub fn test_chunk_type_is_reserved_bit_valid_pngspec() {
        let chunk = ChunkType::from_str("bLOb").unwrap();
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
    pub fn test_chunk_type_is_safe_to_copy_pngspec() {
        let chunk = ChunkType::from_str("bLOb").unwrap();
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
    pub fn test_invalid_chunk_is_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid()); 

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
        assert_eq!(Some(INVALID_BYTES_MSG), chunk.err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}

