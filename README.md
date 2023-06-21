# PNGME

Reference: https://picklenerd.github.io/pngme_book/introduction.html

# 第一部分 - ChunkType

> 参考: http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html
> 参考: https://www.w3.org/TR/PNG/#5Chunk-layout

## 实现要求

1. Copy the unit tests below and paste them at the bottom of your chunk_type.rs file.
2. Write a ChunkType struct with your implementation of PNG chunk types.
3. Implement TryFrom`<[u8; 4]>`for your ChunkType.
4. Implement FromStr for your ChunkType.
5. Implement Display for your ChunkType.
6. Implement or derive PartialEq and Eq for your ChunkType
7. Required methods:
    - fn bytes(&self) -> [u8; 4]
    - fn is_valid(&self) -> bool
    - fn is_critical(&self) -> bool
    - fn is_public(&self) -> bool
    - fn is_reserved_bit_valid(&self) -> bool
    - fn is_safe_to_copy(&self) -> bool
8. Pass all of the unit tests.

## 实现思路

