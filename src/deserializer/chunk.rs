use std::{
    fmt,
    io::{self, Read},
    str,
};

use super::core::RbxReadExt;

const ZSTD_MAGIC_NUMBER: &[u8] = &[0x28, 0xb5, 0x2f, 0xfd];

/// Represents one chunk from a binary model file.
#[derive(Debug)]
pub struct Chunk {
    pub name: [u8; 4],
    pub data: Vec<u8>,
}

impl Chunk {
    /// Reads and decodes a `Chunk` from the given reader.
    pub fn decode<R: Read>(mut reader: R) -> io::Result<Chunk> {
        let header = decode_chunk_header(&mut reader)?;

        let data = if header.compressed_len == 0 {
            let mut data = Vec::with_capacity(header.len as usize);
            reader.take(header.len as u64).read_to_end(&mut data)?;
            data
        } else {
            let mut compressed_data = Vec::with_capacity(header.compressed_len as usize);
            reader
                .take(header.compressed_len as u64)
                .read_to_end(&mut compressed_data)?;

            if &compressed_data[0..4] == ZSTD_MAGIC_NUMBER {
                zstd::bulk::decompress(&compressed_data, header.len as usize)?
            } else {
                lz4::block::decompress(&compressed_data, Some(header.len as i32))?
            }
        };

        assert_eq!(data.len(), header.len as usize);

        Ok(Chunk {
            name: header.name,
            data,
        })
    }
}

#[derive(Debug)]
struct ChunkHeader {
    /// 4-byte short name for the chunk, like "INST" or "PRNT"
    name: [u8; 4],

    /// The length of the chunk's compressed data. For uncompressed chunks, this
    /// is always zero.
    compressed_len: u32,

    /// The length that the chunk's data will have when decompressed. For
    /// uncompressed chunks, this is their length as-is.
    len: u32,

    /// Always zero.
    reserved: u32,
}

impl fmt::Display for ChunkHeader {
    fn fmt(&self, output: &mut fmt::Formatter) -> fmt::Result {
        let name = if let Ok(name) = str::from_utf8(&self.name) {
            name.to_owned()
        } else {
            format!("{:?}", self.name)
        };

        write!(
            output,
            "Chunk \"{}\" (compressed: {}, len: {}, reserved: {})",
            name, self.compressed_len, self.len, self.reserved
        )
    }
}

fn decode_chunk_header<R: Read>(source: &mut R) -> io::Result<ChunkHeader> {
    let mut name = [0; 4];
    source.read_exact(&mut name)?;

    let compressed_len = source.read_le_u32()?;
    let len = source.read_le_u32()?;
    let reserved = source.read_le_u32()?;

    if reserved != 0 {
        panic!(
            "Chunk reserved space was not zero, it was {}. This chunk may be malformed.",
            reserved
        );
    }

    Ok(ChunkHeader {
        name,
        compressed_len,
        len,
        reserved,
    })
}
