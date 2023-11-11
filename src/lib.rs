use std::fs;

use std::path::PathBuf;


pub mod nbt_tag;
pub mod file_parser;

#[derive(Clone, Debug, Default)]
pub struct McWorldDescriptor {
    pub input_path: PathBuf,
    pub version: String,
    pub raw_data: nbt_tag::NbtTagCompound,
}

impl McWorldDescriptor {
    pub fn new(input_path: PathBuf) -> std::io::Result<Self> {
        let raw_data = Self::read_from_binary_file(&input_path)?;

        Ok(McWorldDescriptor {
            input_path,
            version: "0.0.0".to_string(),
            raw_data,
        })
    }

    fn read_from_binary_file(input_path: &PathBuf) -> std::io::Result<nbt_tag::NbtTagCompound> {
        
        let bin_content = file_parser::FileParser::new(input_path, file_parser::ReadMode::EntireFile);
        let root = bin_content.parse()?;


        Ok(root.compound().unwrap())
    }

}