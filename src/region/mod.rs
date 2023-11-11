use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use flate2::read::ZlibDecoder;
use std::path::PathBuf;

const HEADER_LENGTH: usize = 4096;


struct RegionFile {
    file_path: PathBuf,
    file: File,
}

impl RegionFile {
    pub fn new(file_path: PathBuf) -> io::Result<Self> {
        let file = File::open(&file_path)?;
        Ok(RegionFile { file_path, file })
    }


    fn read_header(&mut self) -> io::Result<Vec<u8>> {
        let mut file = File::open(file_path)?;
        let mut header = vec![0; HEADER_LENGTH];
        file.read_exact(&mut header)?;
        Ok(header)
    }
    
    fn parse_chunk_offsets(&self, header: &[u8]) -> Vec<(u32, u32)> {
        header
            .chunks(4)
            .map(|chunk| {
                let offset = u32::from_be_bytes([chunk[0], chunk[1], chunk[2], 0]) << 4;
                let size = u32::from(chunk[3]) * 4096;
                (offset, size)
            })
            .collect()
    }
    

    /// Reads a chunk from the file based on the provided offset and size.
    fn read_chunk(&mut self, offset: u32, size: u32) -> io::Result<Vec<u8>> {
        let mut chunk_data = vec![0; size as usize];
        file.seek(SeekFrom::Start(offset as u64))?;
        file.read_exact(&mut chunk_data)?;
    
        // Decompress chunk data
        let mut decoder = ZlibDecoder::new(&chunk_data[..]);
        let mut decompressed_data = Vec::new();
        decoder.read_to_end(&mut decompressed_data)?;
        
        Ok(decompressed_data)
    }

    /// Public method to process the region file.
    pub fn process(&mut self) -> io::Result<()> {
        let header = self.read_header()?;
        let chunk_offsets = self.parse_chunk_offsets(&header[..4096]); // Adjust as needed

        for (offset, size) in chunk_offsets {
            if offset == 0 { continue; } // Skip if the chunk is not present
            let chunk_data = self.read_chunk(offset, size)?;
            // Process the chunk data (decompress and parse NBT)
        }

        Ok(())
    }
}

