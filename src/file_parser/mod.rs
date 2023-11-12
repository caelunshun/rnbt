use crate::nbt_tag::*;
use crate::region::*;

use byteorder::{BigEndian, ReadBytesExt};
use std::io::Cursor;
use std::path::PathBuf;
use std::fs;
use flate2::read::GzDecoder;
use std::io::BufReader;
use std::io::Read;

#[cfg(test)]
mod tests;

pub enum FileType {
    Nbt,
    Region,
}

pub enum ReadMode {
    EntireFile,
    Stream,
}

pub struct FileParser {
    file_path: PathBuf,
    read_mode: ReadMode,
    file_type: FileType,
}

impl FileParser {
    pub fn new(file_path: &PathBuf, read_mode: ReadMode, file_type: FileType) -> Self {
        FileParser { 
            file_path: file_path.to_path_buf(), 
            read_mode,
            file_type
        }

    }

    pub fn parse(&self) -> std::io::Result<NbtTag> {
        let buf = match self.read_mode {
            ReadMode::EntireFile => self.read_entire_file()?,
            ReadMode::Stream => self.read_stream()?,
        };

        // Handle the result from parse_bytes
        match parse_bytes(&buf) {
            Ok(nbt_tag) => Ok(nbt_tag),  // On success, return the NbtTag
            Err(_) => Err(std::io::Error::new(std::io::ErrorKind::Other, "Parse error")),  // On error, return an std::io::Error
        }
    }

    pub fn read (&self) -> std::io::Result<Vec<u8>> {
        let buf = match self.read_mode {
            ReadMode::EntireFile => self.read_entire_file()?,
            ReadMode::Stream => self.read_stream()?,
        };

        Ok(buf)
    }

    fn read_entire_file(&self) -> std::io::Result<Vec<u8>> {
        
        // Open the file and create a buffered reader for efficient reading
        let file = fs::File::open(&self.file_path)?;
        // let decoder = GzDecoder::new(file);
        let mut reader = BufReader::new(file);

        // Read the entire contents into a buffer
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;


        Ok(buf)
    }

    fn read_stream(&self) -> std::io::Result<Vec<u8>> {
        // Implementation for streaming read
        // ...
        //let mut buf = Vec::new();
        //buf = "not implemented".as_bytes().to_vec();
        todo!("not implemented yet");
        //Ok(buf)
    }

}


//TODO: put these guys in FileParser, workaround for region file
pub fn parse_bytes(bytes: &[u8]) -> Result<NbtTag, ()> {
    let mut cursor = Cursor::new(bytes);
    
    // Read root compound - read type first
    let ty = {
        let id = cursor.read_u8().map_err(|_| ())?;
        NbtTagType::from_id(id).ok_or_else(|| ())?
    };
    if ty != NbtTagType::Compound {
        return Err(());
    }

    let name_len = cursor.read_i16::<BigEndian>().map_err(|_| ())?;
    let mut name = String::with_capacity(name_len as usize);
    for _ in 0..name_len {
        let ch = cursor.read_u8().map_err(|_| ())?;
        name.push(ch as char);
    }

    let root = parse_compound(&mut cursor, name)?;

    Ok(NbtTag::Compound(root))
}

fn parse_compound(cursor: &mut Cursor<&[u8]>, name: String) -> Result<NbtTagCompound, ()> {
    let mut compound = NbtTagCompound::new(name.as_str());

    // Read values until NBT_End is reached
    loop {
        let type_id = cursor.read_u8().map_err(|_| ())?;

        let ty = NbtTagType::from_id(type_id).ok_or_else(|| ())?;
        if ty == NbtTagType::End {
            // Finish early - nothing more to read
            break;
        }

        // Read name
        let name = {
            let len = cursor.read_i16::<BigEndian>().map_err(|_| ())?;
            let mut name = String::with_capacity(len as usize);
            for _ in 0..len {
                let ch = cursor.read_u8().map_err(|_| ())?;
                name.push(ch as char);
            }

            name
        };

        // Read value
        let value = parse_value(cursor, ty, name.clone())?;

        compound.values.insert(name, value);
    }

    Ok(compound)
}

fn parse_list(cursor: &mut Cursor<&[u8]>, name: String) -> Result<NbtTagList, ()> {
    // Type of values contained in the list
    let ty = {
        let id = cursor.read_u8().map_err(|_| ())?;
        NbtTagType::from_id(id).ok_or_else(|| ())?
    };

    // Length of list, in number of values (not bytes)
    let len = cursor.read_i32::<BigEndian>().map_err(|_| ())?;
    if len > 65536 {
        return Err(());
    }

    let mut values = Vec::with_capacity(len as usize);

    for _ in 0..len {
        let val = parse_value(cursor, ty, "".to_string())?;
        values.push(val)
    }

    Ok(NbtTagList::new(name, ty, values))
}

fn parse_value(cursor: &mut Cursor<&[u8]>, ty: NbtTagType, name: String) -> Result<NbtTag, ()> {
    Ok(match ty {
        NbtTagType::End => unreachable!(), // Should already be covered
        NbtTagType::Byte => {
            let x = cursor.read_i8().map_err(|_| ())?;
            NbtTag::Byte(NbtTagByte::new(name.clone(), x))
        }
        NbtTagType::Short => {
            let x = cursor.read_i16::<BigEndian>().map_err(|_| ())?;
            NbtTag::Short(NbtTagShort::new(name.clone(), x))
        }
        NbtTagType::Int => {
            let x = cursor.read_i32::<BigEndian>().map_err(|_| ())?;
            NbtTag::Int(NbtTagInt::new(name.clone(), x))
        }
        NbtTagType::Long => {
            let x = cursor.read_i64::<BigEndian>().map_err(|_| ())?;
            NbtTag::Long(NbtTagLong::new(name.clone(), x))
        }
        NbtTagType::Float => {
            let x = cursor.read_f32::<BigEndian>().map_err(|_| ())?;
            NbtTag::Float(NbtTagFloat::new(name.clone(), x))
        }
        NbtTagType::Double => {
            let x = cursor.read_f64::<BigEndian>().map_err(|_| ())?;
            NbtTag::Double(NbtTagDouble::new(name.clone(), x))
        }
        NbtTagType::ByteArray => {
            let len = cursor.read_i32::<BigEndian>().map_err(|_| ())?;
            if len > 65536 {
                // Yeah... no.
                return Err(());
            }

            let mut buf = Vec::with_capacity(len as usize);
            for _ in 0..len {
                let x = cursor.read_i8().map_err(|_| ())?;
                buf.push(x);
            }

            NbtTag::ByteArray(NbtTagByteArray::new(name.clone(), buf))
        }
        NbtTagType::String => {
            let len = cursor.read_u16::<BigEndian>().map_err(|_| ())?;
            let mut buf = String::with_capacity(len as usize);

            for _ in 0..len {
                let ch = cursor.read_u8().map_err(|_| ())?;
                buf.push(ch as char);
            }

            NbtTag::String(NbtTagString::new(name.clone(), buf))
        }
        NbtTagType::List => {
            let list = parse_list(cursor, name)?;
            NbtTag::List(list)
        }
        NbtTagType::Compound => {
            let compound = parse_compound(cursor, name)?;
            NbtTag::Compound(compound)
        }
        NbtTagType::IntArray => {
            let len = cursor.read_i32::<BigEndian>().map_err(|_| ())?;
            if len > 65536 {
                return Err(());
            }

            let mut buf = Vec::with_capacity(len as usize);
            for _ in 0..len {
                let x = cursor.read_i32::<BigEndian>().map_err(|_| ())?;
                buf.push(x);
            }

            NbtTag::IntArray(NbtTagIntArray::new(name.clone(), buf))
        }
        NbtTagType::LongArray => {
            let len = cursor.read_i32::<BigEndian>().map_err(|_| ())?;
            if len > 65536 {
                return Err(());
            }

            let mut buf = Vec::with_capacity(len as usize);
            for _ in 0..len {
                let x = cursor.read_i64::<BigEndian>().map_err(|_| ())?;
                buf.push(x);
            }

            NbtTag::LongArray(NbtTagLongArray::new(name.clone(), buf))
        }
    })
}
