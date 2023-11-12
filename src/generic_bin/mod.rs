use crate::file_parser;
use crate::nbt_tag::NbtTagCompound;
use std::io;
use std::path::PathBuf;
use flate2::read::ZlibDecoder;
use flate2::read::GzDecoder;
use std::io::Read;

pub enum FileType {
    Nbt,
    Region,
}

pub enum CompressionType {
    Uncompressed = 0,
    Gzip = 1,
    Zlib = 2,
}

impl CompressionType {
    fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(CompressionType::Uncompressed),
            1 => Some(CompressionType::Gzip),
            2 => Some(CompressionType::Zlib),
            _ => None,
        }
    }

    fn to_u8(self) -> u8 {
        match self {
            CompressionType::Uncompressed => 0,
            CompressionType::Gzip => 1,
            CompressionType::Zlib => 2,
        }
    }
}

pub struct GenericBinFile {
    raw_data: Vec<u8>
}

impl GenericBinFile {
    pub fn new(file_path: &PathBuf, file_type: FileType) -> io::Result<Self> {
        let bin_file = file_parser::FileParser::new(file_path, file_parser::ReadMode::EntireFile, file_type).read()?;
        Ok(GenericBinFile { raw_data: bin_file})
    }

    pub fn get_raw_data(&self) -> &Vec<u8> {
        &self.raw_data
    }

    pub fn to_compounds_list(&self) -> std::io::Result<Vec<NbtTagCompound>> {

        let uncompressed_data = self.try_decode_data()?;

        let root = match file_parser::parse_bytes(&uncompressed_data) {
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


    pub fn try_decode_data(&self) -> io::Result<Vec<u8>> {
        
        let methods = [CompressionType::Gzip, CompressionType::Zlib, CompressionType::Uncompressed];
        
        for method in methods {
            let uncompressed_data = match self.decode_binary_data(&self.raw_data, [method.to_u8()].as_slice()) {
                Ok(uncompressed_data) => uncompressed_data,
                Err(_) => continue,
            };
            
            return Ok(uncompressed_data);        
        }
        
        Err(io::Error::new(io::ErrorKind::Other, "All decompression attempts failed"))
    }

    pub fn decode_binary_data(&self, chunk_payload: &[u8], chunk_compression_method: &[u8]) -> io::Result<Vec<u8>> {
        // Decompress chunk data
        // acoording to minecraft wiki case Gzip and not compressed are not used in practice
        // but they are officially supported
        match CompressionType::from_u8(chunk_compression_method[0]) {
            Some(CompressionType::Gzip) => {
                // Gzip compression
                let mut decoder = GzDecoder::new(chunk_payload);
                let mut chunk_decompressed_payload = Vec::new();
                decoder.read_to_end(&mut chunk_decompressed_payload)?;
                Ok(chunk_decompressed_payload)
            },
            Some(CompressionType::Zlib) => { 
                // Zlib compression
                let mut decoder = ZlibDecoder::new(chunk_payload);
                let mut chunk_decompressed_payload = Vec::new();
                decoder.read_to_end(&mut chunk_decompressed_payload)?;
                Ok(chunk_decompressed_payload)
            },
            Some(CompressionType::Uncompressed) => {
                // Data is uncompressed
                let chunk_decompressed_payload = chunk_payload.to_vec();
                Ok(chunk_decompressed_payload)
            },
            _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "Unknown compression format"))
        }
    }
}
