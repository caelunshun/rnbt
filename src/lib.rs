use std::fs;
use std::path::PathBuf;


pub mod nbt_tag;
pub mod file_parser;
pub mod region;

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
        if let Some(ext) = input_path.extension().and_then(|e| e.to_str()) {
            if ext == "mcr" || ext == "mca" {
                let region_file = region::RegionFile::new(input_path)?;
                
                // for chunk_index  in 0..region_file.num_chunks() {
                //     region_file.read_chunk(chunk_index)?;
                // }
                let chunks_list = region_file.get_chunks_as_nbtcompound();
            }
        }
        
        //TODO: FileParser shall not be used by McWorldDescriptor directly
        let bin_content = file_parser::FileParser::new(input_path, file_parser::ReadMode::EntireFile, file_parser::FileType::Nbt);
        let root = bin_content.parse()?;

        // If the extension is not mcr or mca, or if there's no extension,
        // handle this case appropriately
        //Err(io::Error::new(io::ErrorKind::InvalidInput, "Unsupported file extension"))


        Ok(root.compound().unwrap())
    }

}