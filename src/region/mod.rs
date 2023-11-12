use crate::file_parser;
use crate::file_parser::{FileParser, ReadMode};
use crate::nbt_tag::*;
use crate::generic_bin::*;

use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use flate2::read::ZlibDecoder;
use flate2::read::GzDecoder;
use std::path::PathBuf;

const HEADER_LENGTH: usize = 4096;
const CHUNK_HEADER_LENGTH: usize = 4;
const CHUNK_HEADER_COMPRESSION: usize = CHUNK_HEADER_LENGTH + 1;

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
}

pub struct RegionFile {
    bin_content: GenericBinFile,
    num_chunks: usize,
    chunk_offsets: Vec<(u32, u32)>,
    //chunks_as_nbt: Vec<NbtTagCompound>,
}

impl RegionFile {
    pub fn new(file_path: &PathBuf) -> io::Result<Self> {
        let generic_bin = GenericBinFile::new(file_path, FileType::Region)?;
        let mut region_file = RegionFile { bin_content: generic_bin, num_chunks: 0, chunk_offsets: Vec::new() };

        //let region_fp = FileParser::new(&file_path, ReadMode::EntireFile, FileType::Region);
        let region_content = region_file.bin_content.get_raw_data();

        let header = match Self::read_header(&region_content)
        {
            Ok(h) => h,
            Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
        };

        let offsets = Self::parse_chunk_offsets(&header);
        let num_chunks = offsets.len();

        region_file.chunk_offsets = offsets;
        region_file.num_chunks = num_chunks;

        Ok(region_file)
    }

    /// Returns the number of chunks in the region file.
    pub fn get_chunks_num(&self) -> usize {
        self.num_chunks
    }

    pub fn to_compounds_list(&self) -> std::io::Result<Vec<NbtTagCompound>> {
        let chunks_as_nbt = self.process_all_chunks()?;
        Ok(chunks_as_nbt)
    }
    
    /// Public method to process the region file.
    fn process_all_chunks(&self) -> io::Result<Vec<NbtTagCompound>> {

        let mut processed_chunks_list = Vec::new();

        for index in 0..self.num_chunks {
            let (offset, _) = self.chunk_offsets[index];    
            if offset == 0 { 
                continue; // Skip if the chunk is not present
            }
            
            let chunk_data = self.read_and_decompress_chunk(index)?;
            let chunk_nbt = file_parser::parse_bytes(&chunk_data)
                .map_err(|_| io::Error::new(io::ErrorKind::Other, "Parse error"))?;

            //TODO: remove unwrap and handle errors
            processed_chunks_list.push(chunk_nbt.compound().unwrap());
        }

        Ok(processed_chunks_list)
    }

    /// Reads a chunk from the file based on the provided offset and size.
    /// 
    /// https://minecraft.fandom.com/wiki/Region_file_format
    /// 
    /// A Chunk is always represented as 4096 bytes.
    /// The first 4 bytes (big endian) represent the actual length of the chunk.
    /// The fifth byte is the compression method (usually zlib)
    /// The rest x bytes (where x is the u32 of the first 4 bytes) are the actual chunk data, which is compressed.
    /// 
    fn read_and_decompress_chunk(&self, index: usize) -> io::Result<Vec<u8>> {
        if index < self.chunk_offsets.len() {
            let (offset, size) = self.chunk_offsets[index];
            let raw_data = self.bin_content.get_raw_data();

            if (offset as usize) < raw_data.len() && (offset as usize) + (size as usize) <= raw_data.len() {
                let chunk_data = &raw_data[offset as usize..(offset as usize) + (size as usize)];

                let real_chunk_len_slice = &chunk_data[..CHUNK_HEADER_LENGTH];

                if real_chunk_len_slice.len() == 4 {
                    let bytes = [real_chunk_len_slice[0], real_chunk_len_slice[1], real_chunk_len_slice[2], real_chunk_len_slice[3]];
                    
                    let real_chunk_len = u32::from_be_bytes(bytes) as usize;
                    let chunk_compression_method = &chunk_data[CHUNK_HEADER_LENGTH..CHUNK_HEADER_COMPRESSION];
                    let chunk_payload = &chunk_data[CHUNK_HEADER_COMPRESSION..CHUNK_HEADER_COMPRESSION + real_chunk_len];

                    //Self::decode_binary_data(chunk_payload, chunk_compression_method)
                    self.bin_content.decode_binary_data(chunk_payload, chunk_compression_method)
                }
                else {
                    Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid or Unsupported chunk header length"))
                }
                
            } else {
                Err(io::Error::new(io::ErrorKind::InvalidInput, "Chunk offset/size out of bounds"))
            }
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid chunk index"))
        }
    }
    
    // fn decode_binary_data(chunk_payload: &[u8], chunk_compression_method: &[u8]) -> io::Result<Vec<u8>> {
    //     // Decompress chunk data
    //     // acoording to minecraft wiki case Gzip and not compressed are not used in practice
    //     // but they are officially supported
    //     match CompressionType::from_u8(chunk_compression_method[0]) {
    //         Some(CompressionType::Gzip) => {
    //             // Gzip compression
    //             let mut decoder = GzDecoder::new(chunk_payload);
    //             let mut chunk_decompressed_payload = Vec::new();
    //             decoder.read_to_end(&mut chunk_decompressed_payload)?;
    //             Ok(chunk_decompressed_payload)
    //         },
    //         Some(CompressionType::Zlib) => { 
    //             // Zlib compression
    //             let mut decoder = ZlibDecoder::new(chunk_payload);
    //             let mut chunk_decompressed_payload = Vec::new();
    //             decoder.read_to_end(&mut chunk_decompressed_payload)?;
    //             Ok(chunk_decompressed_payload)
    //         },
    //         Some(CompressionType::Uncompressed) => {
    //             // Data is uncompressed
    //             let chunk_decompressed_payload = chunk_payload.to_vec();
    //             Ok(chunk_decompressed_payload)
    //         },
    //         _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "Unknown compression format"))
    //     }
    // }

    fn read_header(region_content: &Vec<u8>) -> Result<&[u8], &'static str> {
        if region_content.len() >= HEADER_LENGTH {
            Ok(&region_content[..HEADER_LENGTH])
        } 
        else {
            Err("INVALID REGIORN FILE: Data is shorter than expected header length.")
        }
    }
    
    fn parse_chunk_offsets(header: &[u8]) -> Vec<(u32, u32)> {
        header
            .chunks(4)
            .map(|chunk| {
                let offset = u32::from_be_bytes([chunk[0], chunk[1], chunk[2], 0]) << 4;
                let size = u32::from(chunk[3]) * 4096;
                (offset, size)
            })
            .collect()
    }
    
}

