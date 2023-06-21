// file: chunk_type.rs

use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ChunkType([u8; 4]);

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.0
    }

    pub fn is_critical(&self) -> bool {
        self.0[0].is_ascii_uppercase() // 判断首字节是否为大写字母，用来判断是否为关键块类型
    }

    pub fn is_public(&self) -> bool {
        self.0[1].is_ascii_uppercase() // 判断第二个字节是否为大写字母，用来判断是否为公共块类型
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        self.0[2].is_ascii_uppercase() // 判断第三个字节是否为大写字母，用来判断保留位是否有效
    }

    pub fn is_safe_to_copy(&self) -> bool {
        self.0[3].is_ascii_lowercase() // 判断第四个字节是否为小写字母，用来判断是否可安全复制
    }

    pub fn is_valid(&self) -> bool {
        let has_uppercase = self.0.iter().any(|&byte| byte.is_ascii_uppercase()); // 判断是否有大写字母
        let has_lowercase = self.0.iter().any(|&byte| byte.is_ascii_lowercase()); // 判断是否有小写字母
        self.0.iter().all(|&byte| byte.is_ascii_alphabetic()) && has_uppercase && has_lowercase
        // 判断是否为字母
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = ();

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        Ok(ChunkType(value))
    }
}

impl FromStr for ChunkType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err(());
        }

        let bytes = s.as_bytes();
        let mut chunk_type = [0; 4];

        for (i, byte) in bytes.iter().enumerate() {
            // 将 ASCII 字符转换为对应的字节值
            chunk_type[i] = *byte;
        }

        Ok(ChunkType(chunk_type))
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 将字节值转换为对应的 ASCII 字符，并写入到格式化器中
        let s = String::from_utf8_lossy(&self.0);
        write!(f, "{}", s)
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
        assert!(!chunk.is_valid()); // TODO: 这个地方会 panic

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
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
