//! # Chunk
//! 作者: Norfloxaciner <1762161822@qq.com>
//! 创建/修改日期: 2023/06/22
//!
//! 该模块包含了 `Chunk` 结构体的实现。

use std::convert::{TryFrom, TryInto};
use std::error::Error;
use std::fmt;

use crate::chunk_type::ChunkType;
use crc::Crc;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

#[allow(dead_code)]
impl Chunk {
    /// Creates a new `Chunk` with the given `ChunkType` and data.
    /// The length and CRC are calculated automatically.
    /// This function will return an error if the given `ChunkType` is not valid.
    /// The length of the data must be less than or equal to `u32::MAX` bytes.
    /// The CRC is calculated using the CRC-32-Castagnoli algorithm.
    /// See [this page](https://en.wikipedia.org/wiki/Cyclic_redundancy_check) for more information.
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        // 检查数据长度是否超过 u32::MAX
        if data.len() > u32::MAX as usize {
            panic!("Data length is too long");
        }

        // 计算 CRC
        // TODO: 这一步的算法有待商榷
        let my_crc = Crc::<u32>::new(&crc::CRC_32_CKSUM).checksum(&data);

        // 创建 Chunk
        let chunk = Chunk {
            length: data.len() as u32,
            chunk_type,
            data,
            crc: my_crc,
        };

        chunk
    }

    /// The length of the data portion of this chunk.
    pub fn length(&self) -> u32 {
        // 返回数据长度
        self.length
    }

    /// The `ChunkType` of this chunk
    pub fn chunk_type(&self) -> &ChunkType {
        // 返回 ChunkType
        &self.chunk_type
    }

    /// The raw data contained in this chunk in bytes
    pub fn data(&self) -> &[u8] {
        // 返回数据
        &self.data
    }

    /// The CRC of this chunk
    pub fn crc(&self) -> u32 {
        // 返回 CRC
        self.crc
    }

    /// Returns the data stored in this chunk as a `String`. This function will return an error
    /// if the stored data is not valid UTF-8.
    pub fn data_as_string(&self) -> Result<String, Box<dyn Error>> {
        // 将数据转换为 String
        let data_string = String::from_utf8(self.data.clone())?;

        Ok(data_string)
    }

    /// Returns this chunk as a byte sequences described by the PNG spec.
    /// The following data is included in this byte sequence in order:
    /// 1. Length of the data *(4 bytes)*
    /// 2. Chunk type *(4 bytes)*
    /// 3. The data itself *(`length` bytes)*
    /// 4. The CRC of the chunk type and data *(4 bytes)*
    pub fn as_bytes(&self) -> Vec<u8> {
        // 将 Chunk 转换为字节序列
        let mut bytes: Vec<u8> = Vec::new();

        // 将数据长度转换为字节序列
        bytes.extend_from_slice(&self.length.to_be_bytes());

        // 将 ChunkType 转换为字节序列
        bytes.extend_from_slice(&self.chunk_type.bytes());

        // 将数据转换为字节序列
        bytes.extend_from_slice(&self.data);

        // 将 CRC 转换为字节序列
        bytes.extend_from_slice(&self.crc.to_be_bytes());

        bytes
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Box<dyn Error>;    // 使用 Box<dyn Error> 作为错误类型，dyn 表示动态类型

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        // 检查数据长度是否小于 12
        if data.len() < 12 {
            return Err("Data length is too short".into());
        }

        // 读取数据长度
        let length_bytes: [u8; 4] = data[0..4].try_into()?;
        let length: u32 = u32::from_be_bytes(length_bytes);

        // 检查数据长度是否超过 u32::MAX
        if length > u32::MAX {
            return Err("Data length is too long".into());
        }

        // 读取 ChunkType
        let chunk_type_bytes: [u8; 4] = data[4..8].try_into()?;
        let chunk_type = ChunkType::try_from(chunk_type_bytes)?;

        // 读取数据
        let data = data[8..(8 + length as usize)].to_vec();

        // 读取 CRC
        let crc_bytes: [u8; 4] = data[(8 + length as usize)..(12 + length as usize)].try_into()?;   // 这一步会超出数组长度，导致 panic
        let crc: u32 = u32::from_be_bytes(crc_bytes);

        // 计算 CRC
        let my_crc = Crc::<u32>::new(&crc::CRC_32_CKSUM).checksum(&data);

        // 检查 CRC 是否正确
        if crc != my_crc {
            return Err("CRC does not match".into());
        }

        // 创建 Chunk
        let chunk = Chunk {
            length,
            chunk_type,
            data,
            crc,
        };

        Ok(chunk)
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} bytes", self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
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

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
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

        assert!(chunk.is_err());
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

        let _chunk_string = format!("{}", chunk);
    }
}
