use crate::file_parser;
use crate::file_parser::{FileParser, FileType, ReadMode};
use crate::nbt_tag::*;

use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use flate2::read::ZlibDecoder;
use std::path::PathBuf;

const HEADER_LENGTH: usize = 4096;


pub struct RegionFile {
    raw_data: Vec<u8>,
    num_chunks: usize,
    chunk_offsets: Vec<(u32, u32)>,
    chunks_as_nbt: Vec<NbtTagCompound>,
}

impl RegionFile {
    pub fn new(file_path: &PathBuf) -> io::Result<Self> {
        let region_fp = FileParser::new(&file_path, ReadMode::EntireFile, FileType::Region);
        let region_content = region_fp.read()?;

        let header = match Self::read_header(&region_content)
        {
            Ok(h) => h,
            Err(e) => return Err(io::Error::new(io::ErrorKind::Other, e)),
        };

        let offsets = Self::parse_chunk_offsets(&header);
        let num_chunks = offsets.len();
        let chunks_as_nbt = Self::process_all_chunks(&region_content, num_chunks, &offsets)?;


        Ok(RegionFile {raw_data: region_content, num_chunks, chunk_offsets: offsets, chunks_as_nbt})
    }

    /// Returns the number of chunks in the region file.
    pub fn get_chunks_num(&self) -> usize {
        self.num_chunks
    }

    pub fn get_chunks_as_nbtcompound(&self) -> &Vec<NbtTagCompound> {
        &self.chunks_as_nbt
    }
    
    /// Public method to process the region file.
    fn process_all_chunks(region_content: &Vec<u8>,num_chunks: usize, chunk_offsets: &Vec<(u32, u32)>) -> io::Result<Vec<NbtTagCompound>> {

        let mut processed_chunks_list = Vec::new();

        for index in 0..num_chunks {
            let (offset, _) = chunk_offsets[index];
            
            if offset == 0 { 
                continue; // Skip if the chunk is not present
            }
            
            let chunk_data = Self::read_and_decompress_chunk(region_content, chunk_offsets, index)?;
            let chunk_nbt = file_parser::parse_bytes(&chunk_data)
                .map_err(|_| io::Error::new(io::ErrorKind::Other, "Parse error"))?;


            //TODO: remove unwrap and handle errors
            processed_chunks_list.push(chunk_nbt.compound().unwrap());

        }

        Ok(processed_chunks_list)
    }

    /// Reads a chunk from the file based on the provided offset and size.
    fn read_and_decompress_chunk(raw_data: &Vec<u8>, chunk_offsets: &Vec<(u32, u32)>,index: usize) -> io::Result<Vec<u8>> {
        if index < chunk_offsets.len() {
            let (offset, size) = chunk_offsets[index];
            
            if (offset as usize) < raw_data.len() && (offset as usize) + (size as usize) <= raw_data.len() {
                let chunk_data = &raw_data[offset as usize..(offset as usize) + (size as usize)];

                // Decompress chunk data
                let mut decoder = ZlibDecoder::new(chunk_data);
                let mut chunk_decompressed_data = Vec::new();
                decoder.read_to_end(&mut chunk_decompressed_data)?;
                
                Ok(chunk_decompressed_data)
            } else {
                Err(io::Error::new(io::ErrorKind::InvalidInput, "Chunk offset/size out of bounds"))
            }
        } else {
            Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid chunk index"))
        }
    }

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
