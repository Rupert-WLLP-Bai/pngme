# Chunk

## 实现要求

1. Copy the unit tests below and paste them at the bottom of your `chunk.rs` file.
2. Write a `Chunk` struct with your implementation of PNG chunks.
3. Implement `TryFrom<&[u8]>` for your `Chunk`.
4. Implement `Display` for your `Chunk`.
5. Required methods:
   1. `fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk`
   2. `fn length(&self) -> u32`
   3. `fn chunk_type(&self) -> &ChunkType`
   4. `fn data(&self) -> &[u8]`
   5. `fn crc(&self) -> u32`
   6. `fn data_as_string(&self) -> Result<String>`
   7. `fn as_bytes(&self) -> Vec<u8>`
6. Pass all of the unit tests.
