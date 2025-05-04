mod chunk;
mod core;
mod header;

use core::RbxReadExt;
use std::{
    collections::{HashMap, HashSet},
    io::Read,
};

pub struct ClassMap {
    pub class_to_props: HashMap<String, HashSet<String>>,
    pub id_to_class: HashMap<u32, String>,
}

impl ClassMap {
    fn new() -> Self {
        return Self {
            class_to_props: HashMap::new(),
            id_to_class: HashMap::new(),
        };
    }
}

pub fn deserialize<R: RbxReadExt>(mut reader: R) -> Result<ClassMap, Box<dyn std::error::Error>> {
    let mut class_map = ClassMap::new();
    header::FileHeader::decode(&mut reader)?;
    loop {
        let chunk = chunk::Chunk::decode(&mut reader)?;
        let mut chunk_data = chunk.data.as_slice();

        match &chunk.name {
            b"INST" => {
                let type_id = chunk_data.read_le_u32().unwrap();
                let type_name = chunk_data.read_string().unwrap();
                class_map
                    .id_to_class
                    .entry(type_id)
                    .insert_entry(type_name.clone());
            }
            b"PROP" => {
                let type_id = chunk_data.read_le_u32().unwrap();
                let class_name = class_map.id_to_class.get(&type_id).unwrap();
                let prop_name = chunk_data.read_string().unwrap();
                class_map
                    .class_to_props
                    .entry(class_name.clone())
                    .or_default()
                    .insert(prop_name.clone());
            }
            b"END\0" => {
                break;
            }
            _ => {}
        }
    }

    return Ok(class_map);
}
