//! # Chunk Type
//! 作者: Norfloxaciner <1762161822@qq.com>
//! 创建/修改日期: 2023/06/22
//! 
//! 该模块包含了 `ChunkType` 结构体的实现。

use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ChunkType([u8; 4]);

#[allow(dead_code)]
impl ChunkType {
    /// 获取 ChunkType 的字节数组表示
    pub fn bytes(&self) -> [u8; 4] {
        self.0
    }

    /// 检查 ChunkType 是否为关键类型
    pub fn is_critical(&self) -> bool {
        self.0[0] & 32 == 0
    }

    /// 检查 ChunkType 是否为公共类型
    pub fn is_public(&self) -> bool {
        self.0[1] & 32 == 0
    }

    /// 检查 ChunkType 的保留位是否有效
    pub fn is_reserved_bit_valid(&self) -> bool {
        self.0[2] & 32 == 0
    }

    /// 检查 ChunkType 是否可以安全复制
    pub fn is_safe_to_copy(&self) -> bool {
        self.0[3] & 32 == 32 // 判断第四个字节的第5位是否为小写
    }

    /// 检查 ChunkType 是否为有效类型
    pub fn is_valid(&self) -> bool {
        let third = self.0[2];
        let fourth = self.0[3];

        // 第三个字节的第5位必须为大写，第四个字节必须为字母
        (third & 32 == 0) && fourth.is_ascii_alphabetic()
    }

    /// 检查 ChunkType 是否为辅助类型
    pub fn is_ancillary(&self) -> bool {
        self.0[3] & 32 == 32 // 判断第四个字节的第5位是否为小写
    }

    /// 检查 ChunkType 是否为私有类型
    pub fn is_private(&self) -> bool {
        self.0[1] & 32 == 32 // 判断第二个字节的第5位是否为小写
    }

    /// from_str 方法
    pub fn from_str(s: &str) -> Result<Self, &'static str> {
        if s.len() != 4 {
            return Err("Invalid chunk type");
        }
        let mut bytes = [0u8; 4];
        for (i, c) in s.chars().enumerate() {
            bytes[i] = c as u8;
        }
        ChunkType::try_from(bytes)
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = &'static str;

    /// 尝试将字节数组转换为 ChunkType 类型
    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        if ChunkType::is_valid_type(value) {
            Ok(ChunkType(value))
        } else {
            Err("Invalid chunk type")
        }
    }
}

impl FromStr for ChunkType {
    type Err = &'static str;

    /// 将字符串解析为 ChunkType 类型
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err("Invalid chunk type");
        }
        let mut bytes = [0u8; 4];
        for (i, c) in s.chars().enumerate() {
            bytes[i] = c as u8;
        }
        ChunkType::try_from(bytes)
    }
}

impl fmt::Display for ChunkType {
    /// 将 ChunkType 类型格式化为字符串
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = String::from_utf8_lossy(&self.0);
        write!(f, "{}", s)
    }
}

impl ChunkType {
    /// 检查是否为有效的 ChunkType 类型
    fn is_valid_type(value: [u8; 4]) -> bool {
        value.iter().all(u8::is_ascii_alphabetic)
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
        assert!(!chunk.is_valid());

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
