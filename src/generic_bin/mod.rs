use crate::file_parser;
use crate::nbt_tag::NbtTagCompound;
use std::io;
use std::path::PathBuf;

pub struct GenericBinFile {
    raw_data: Vec<u8>
}

impl GenericBinFile {
    pub fn new(file_path: &PathBuf) -> io::Result<Self> {
        let bin_file = file_parser::FileParser::new(file_path, file_parser::ReadMode::EntireFile, file_parser::FileType::Nbt).read()?;
        Ok(GenericBinFile { raw_data: bin_file})
    }

    pub fn to_compounds_list(&self) -> std::io::Result<Vec<NbtTagCompound>> {
        let root = match file_parser::parse_bytes(&self.raw_data) {
            Ok(nbt_tag) => nbt_tag,  // On success, return the NbtTag
            Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Invalid NBT file")),
        };
        
        let compound = match root.compound() {
            Some(nbt_tag) => nbt_tag,  // On success, return the NbtTag
            None => return Err(std::io::Error::new(std::io::ErrorKind::Other, "Invalid Compound tag")),
        };

        let mut compunds_list = Vec::new();
        compunds_list.push(compound);

        Ok(compunds_list)
    }
}
