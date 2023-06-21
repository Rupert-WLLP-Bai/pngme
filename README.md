# PNGME

Reference: https://picklenerd.github.io/pngme_book/introduction.html

# 第一部分 - ChunkType

> 参考: http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html

> 参考: https://www.w3.org/TR/PNG/#5Chunk-layout

## 实现要求

1. Copy the unit tests below and paste them at the bottom of your chunk_type.rs file.
2. Write a ChunkType struct with your implementation of PNG chunk types.
3. Implement TryFrom<[u8; 4]> for your ChunkType.
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
[chunk_type.rs](./src/chunk_type.rs)

## 解释
第一个字节（第一个字母）：
- 大写字母表示关键（critical）块。
- 小写字母表示辅助（ancillary）块。

第二个字节（第二个字母）：
- 大写字母表示公共（public）块，即PNG规范中定义的或在PNG特定用途公共块类型列表中注册的块。
- 小写字母表示私有（private）块，即应用程序为自己的目的定义的未注册块。

第三个字节（第三个字母）：
- 目前保留为0，应为大写字母。但解码器不应因为小写字母而发出警告，因为将来的PNG规范版本可能为该位定义其他含义。

第四个字节（第四个字母）：
- 大写字母表示不安全复制的块，表示它们依赖于图像数据。
- 小写字母表示安全复制的块，表示它们可以在修改PNG文件时安全复制，而不管软件是否识别块类型，无论文件修改的程度如何。

这些要求是为了使解码器能够在不识别类型代码的情况下确定块的某些属性，并允许安全、灵活地扩展PNG格式。请注意，这些属性位是块名称的固有部分，对于任何块类型都是固定的。

以假设的块类型名称bLOb为例，它的属性位如下：
```
bLOb  <-- 32位块类型代码的文本表示形式
||||
|||+- 安全复制位为1（小写字母；第5位为1）
||+-- 保留位为0（大写字母；第5位为0）
|+--- 私有位为0（大写字母；第5位为0）
+---- 辅助位为1（小写字母；第5位为1）
```
因此，这个名称表示一个辅助、公共、安全复制的块。

这些要求和约定使得解码器能够在遇到未知块时做出正确的处理，并保证了PNG文件的可扩展性和兼容性。