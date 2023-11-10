use std::io::Read;
use std::fs;
use flate2::read::GzDecoder;
use std::path::PathBuf;
use std::io::BufReader;

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

    pub fn read_from_binary_file(input_path: &PathBuf) -> std::io::Result<nbt_tag::NbtTagCompound> {
        
        // Open the file and create a buffered reader for efficient reading
        let file = fs::File::open(&input_path)?;
        let decoder = GzDecoder::new(file);
        let mut reader = BufReader::new(decoder);

        // Read the entire contents into a buffer
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;

        // Parse NBT #TODO: remove unwrap and handle possible errors
        let root = file_parser::parse_bytes(&buf).unwrap();

        Ok(root.compound().unwrap())
    }

}