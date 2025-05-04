use super::core::{FILE_MAGIC_HEADER, FILE_SIGNATURE, FILE_VERSION, RbxReadExt};
use std::io::Read;

/// All the information contained in the header before any chunks are read from
/// the file.
pub(crate) struct FileHeader {
    /// The number of instance types (represented for us as `TypeInfo`) that are
    /// in this file. Generally useful to pre-size some containers before
    /// reading the file.
    pub(crate) num_types: u32,

    /// The total number of instances described by this file.
    pub(crate) num_instances: u32,
}

impl FileHeader {
    pub(crate) fn decode<R: Read>(mut source: R) -> Result<Self, Box<dyn std::error::Error>> {
        let mut magic_header = [0; 8];
        source.read_exact(&mut magic_header)?;

        if magic_header != FILE_MAGIC_HEADER {
            return Err("Bad header".into());
        }

        let mut signature = [0; 6];
        source.read_exact(&mut signature)?;

        if signature != FILE_SIGNATURE {
            return Err("Bad header".into());
        }

        let version = source.read_le_u16()?;

        if version != FILE_VERSION {
            return Err("Unknown file version".into());
        }

        let num_types = source.read_le_u32()?;
        let num_instances = source.read_le_u32()?;

        let mut reserved = [0; 8];
        source.read_exact(&mut reserved)?;

        if reserved != [0; 8] {
            return Err("Bad header".into());
        }

        Ok(Self {
            num_types,
            num_instances,
        })
    }
}
