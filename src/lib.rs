use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::collections::HashMap;
use std::io::Cursor;
use std::io::Write;
use std::io::Read;
use serde::Serialize;
use serde::Deserialize;
use std::fs;
use std::io::{self, BufWriter, BufReader};
use flate2::read::GzDecoder;
use std::path::PathBuf;

// use pyo3::prelude::*;
// use pyo3::wrap_pyfunction;

#[macro_use]
extern crate derive_new;


// #[pymodule]
// fn fast_nbt(py: Python, m: &PyModule) -> PyResult<()> {
//     m.add_function(wrap_pyfunction!(read_from_binary_file, m)?)?;
//     Ok(())
// }

// #[pyfunction]
// fn read_from_binary_file() -> PyResult<String> {
//     // Call your actual Rust function here and return the result.
//     Ok("Hello from Rust!".to_string())
// }



#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NbtTagType {
    End,
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
    ByteArray,
    String,
    List,
    Compound,
    IntArray,
    LongArray,
}

impl Default for NbtTagType {
    fn default() -> Self {
        NbtTagType::End
    }
}

impl NbtTagType {
    fn id(&self) -> u8 {
        match self {
            NbtTagType::End => 0,
            NbtTagType::Byte => 1,
            NbtTagType::Short => 2,
            NbtTagType::Int => 3,
            NbtTagType::Long => 4,
            NbtTagType::Float => 5,
            NbtTagType::Double => 6,
            NbtTagType::ByteArray => 7,
            NbtTagType::String => 8,
            NbtTagType::List => 9,
            NbtTagType::Compound => 10,
            NbtTagType::IntArray => 11,
            NbtTagType::LongArray => 12,
        }
    }

    fn from_id(id: u8) -> Option<Self> {
        match id {
            0 => Some(NbtTagType::End),
            1 => Some(NbtTagType::Byte),
            2 => Some(NbtTagType::Short),
            3 => Some(NbtTagType::Int),
            4 => Some(NbtTagType::Long),
            5 => Some(NbtTagType::Float),
            6 => Some(NbtTagType::Double),
            7 => Some(NbtTagType::ByteArray),
            8 => Some(NbtTagType::String),
            9 => Some(NbtTagType::List),
            10 => Some(NbtTagType::Compound),
            11 => Some(NbtTagType::IntArray),
            12 => Some(NbtTagType::LongArray),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NbtTag {
    End,
    Byte(NbtTagByte),
    Short(NbtTagShort),
    Int(NbtTagInt),
    Long(NbtTagLong),
    Float(NbtTagFloat),
    Double(NbtTagDouble),
    ByteArray(NbtTagByteArray),
    String(NbtTagString),
    List(NbtTagList),
    Compound(NbtTagCompound),
    IntArray(NbtTagIntArray),
    LongArray(NbtTagLongArray),
}

impl Default for NbtTag {
    fn default() -> Self {
        NbtTag::End
    }
}

impl NbtTag {
    pub fn ty(&self) -> NbtTagType {
        match &self {
            NbtTag::End => NbtTagType::End,
            NbtTag::Byte(_) => NbtTagType::Byte,
            NbtTag::Short(_) => NbtTagType::Short,
            NbtTag::Int(_) => NbtTagType::Int,
            NbtTag::Long(_) => NbtTagType::Int,
            NbtTag::Float(_) => NbtTagType::Float,
            NbtTag::Double(_) => NbtTagType::Double,
            NbtTag::ByteArray(_) => NbtTagType::ByteArray,
            NbtTag::String(_) => NbtTagType::String,
            NbtTag::List(_) => NbtTagType::List,
            NbtTag::Compound(_) => NbtTagType::Compound,
            NbtTag::IntArray(_) => NbtTagType::IntArray,
            NbtTag::LongArray(_) => NbtTagType::End,
        }
    }

    pub fn byte(&self) -> Option<NbtTagByte> {
        if let NbtTag::Byte(x) = self {
            Some(x.clone())
        } else {
            None
        }
    }

    pub fn short(&self) -> Option<NbtTagShort> {
        if let NbtTag::Short(x) = self {
            Some(x.clone())
        } else {
            None
        }
    }

    pub fn int(&self) -> Option<NbtTagInt> {
        if let NbtTag::Int(x) = self {
            Some(x.clone())
        } else {
            None
        }
    }

    pub fn long(&self) -> Option<NbtTagLong> {
        if let NbtTag::Long(x) = self {
            Some(x.clone())
        } else {
            None
        }
    }

    pub fn float(&self) -> Option<NbtTagFloat> {
        if let NbtTag::Float(x) = self {
            Some(x.clone())
        } else {
            None
        }
    }

    pub fn double(&self) -> Option<NbtTagDouble> {
        if let NbtTag::Double(x) = self {
            Some(x.clone())
        } else {
            None
        }
    }

    pub fn byte_array(&self) -> Option<NbtTagByteArray> {
        if let NbtTag::ByteArray(x) = self {
            Some(x.clone())
        } else {
            None
        }
    }

    pub fn string(&self) -> Option<NbtTagString> {
        if let NbtTag::String(x) = self {
            Some(x.clone())
        } else {
            None
        }
    }

    pub fn list(&self) -> Option<NbtTagList> {
        if let NbtTag::List(x) = self {
            Some(x.clone())
        } else {
            None
        }
    }

    pub fn compound(&self) -> Option<NbtTagCompound> {
        if let NbtTag::Compound(x) = self {
            Some(x.clone())
        } else {
            None
        }
    }

    pub fn int_array(&self) -> Option<NbtTagIntArray> {
        if let NbtTag::IntArray(x) = self {
            Some(x.clone())
        } else {
            None
        }
    }

    pub fn long_array(&self) -> Option<NbtTagLongArray> {
        if let NbtTag::LongArray(x) = self {
            Some(x.clone())
        } else {
            None
        }
    }

}

pub fn parse_bytes(bytes: &[u8]) -> Result<NbtTag, ()> {
    let mut cursor = Cursor::new(bytes);
    parse(&mut cursor)
}

pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<NbtTag, ()> {
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

    let root = parse_compound(cursor, name)?;

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

pub fn write(buf: &mut Vec<u8>, compound: &NbtTagCompound) {
    write_tag_type(buf, NbtTagType::Compound);
    write_tag_name(buf, &compound.name);
    write_compound(buf, compound);
}

fn write_compound(buf: &mut Vec<u8>, compound: &NbtTagCompound) {
    for val in compound.values.values() {
        write_value(buf, val, true);
    }
}

fn write_value(buf: &mut Vec<u8>, value: &NbtTag, write_name: bool) {
    let ty = value.ty();
    write_tag_type(buf, ty);

    match value {
        NbtTag::End => (),
        NbtTag::Byte(val) => {
            if write_name {
                write_tag_name(buf, &val.name);
            }
            buf.write_i8(val.value).unwrap();
        }
        NbtTag::Short(val) => {
            if write_name {
                write_tag_name(buf, &val.name);
            }
            buf.write_i16::<BigEndian>(val.value).unwrap();
        }
        NbtTag::Int(val) => {
            if write_name {
                write_tag_name(buf, &val.name);
            }
            buf.write_i32::<BigEndian>(val.value).unwrap();
        }
        NbtTag::Long(val) => {
            if write_name {
                write_tag_name(buf, &val.name);
            }
            buf.write_i64::<BigEndian>(val.value).unwrap();
        }
        NbtTag::Float(val) => {
            if write_name {
                write_tag_name(buf, &val.name);
            }
            buf.write_f32::<BigEndian>(val.value).unwrap();
        }
        NbtTag::Double(val) => {
            if write_name {
                write_tag_name(buf, &val.name);
            }
            buf.write_f64::<BigEndian>(val.value).unwrap();
        }
        NbtTag::ByteArray(val) => {
            if write_name {
                write_tag_name(buf, &val.name);
            }

            buf.write_i16::<BigEndian>(val.values.len() as i16).unwrap();
            buf.reserve(val.values.len());

            for x in &val.values {
                buf.write_i8(*x).unwrap();
            }
        }
        NbtTag::String(val) => {
            if write_name {
                write_tag_name(buf, &val.name);
            }

            buf.write_u16::<BigEndian>(val.value.len() as u16).unwrap();
            buf.write(val.value.as_bytes()).unwrap();
        }
        NbtTag::List(val) => {
            if write_name {
                write_tag_name(buf, &val.name);
            }

            write_tag_type(buf, val.ty);
            buf.write_i32::<BigEndian>(val.values.len() as i32).unwrap();

            for val in &val.values {
                // Finally, an actual application of recursion
                write_value(buf, val, false);
            }
        }
        NbtTag::Compound(val) => {
            if write_name {
                write_tag_name(buf, &val.name);
            }

            write_compound(buf, val);
        }
        NbtTag::IntArray(val) => {
            if write_name {
                write_tag_name(buf, &val.name);
            }

            buf.write_i32::<BigEndian>(val.values.len() as i32).unwrap();

            buf.reserve(val.values.len());

            for x in &val.values {
                buf.write_i32::<BigEndian>(*x).unwrap();
            }
        }
        NbtTag::LongArray(val) => {
            if write_name {
                write_tag_name(buf, &val.name);
            }

            buf.write_i32::<BigEndian>(val.values.len() as i32).unwrap();

            buf.reserve(val.values.len());

            for x in &val.values {
                buf.write_i64::<BigEndian>(*x).unwrap();
            }
        }
    }
}

fn write_tag_name(buf: &mut Vec<u8>, s: &str) {
    buf.write_i16::<BigEndian>(s.len() as i16).unwrap();
    buf.write(s.as_bytes()).unwrap();
}

fn write_tag_type(buf: &mut Vec<u8>, ty: NbtTagType) {
    buf.write_u8(ty.id()).unwrap();
}

#[derive(Clone, Debug, new, Default, Serialize, Deserialize)]
pub struct NbtTagByte {
    pub name: String,
    pub value: i8,
}

#[derive(Clone, Debug, new, Default, Serialize, Deserialize)]
pub struct NbtTagShort {
    pub name: String,
    pub value: i16,
}

#[derive(Clone, Debug, new, Default, Serialize, Deserialize)]
pub struct NbtTagInt {
    pub name: String,
    pub value: i32,
}

#[derive(Clone, Debug, new, Default, Serialize, Deserialize)]
pub struct NbtTagLong {
    pub name: String,
    pub value: i64,
}

#[derive(Clone, Debug, new, Default, Serialize, Deserialize)]
pub struct NbtTagFloat {
    pub name: String,
    pub value: f32,
}

#[derive(Clone, Debug, new, Default, Serialize, Deserialize)]
pub struct NbtTagDouble {
    pub name: String,
    pub value: f64,
}

#[derive(Clone, Debug, new, Default, Serialize, Deserialize)]
pub struct NbtTagByteArray {
    pub name: String,
    pub values: Vec<i8>,
}

#[derive(Clone, Debug, new, Default, Serialize, Deserialize)]
pub struct NbtTagString {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Debug, new, Default, Serialize, Deserialize)]
pub struct NbtTagList {
    pub name: String,
    pub ty: NbtTagType,
    pub values: Vec<NbtTag>,
}

#[derive(Clone, Debug, Default)]
pub struct McWorldDescriptor {
    pub input_path: PathBuf,
    pub version: String,
    pub raw_data: NbtTagCompound,
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

    pub fn read_from_binary_file(input_path: &PathBuf) -> std::io::Result<NbtTagCompound> {
        
        // Open the file and create a buffered reader for efficient reading
        let file = fs::File::open(&input_path)?;
        let decoder = GzDecoder::new(file);
        let mut reader = BufReader::new(decoder);

        // Read the entire contents into a buffer
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;

        // Parse NBT #TODO: remove unwrap and handle possible errors
        let root = parse_bytes(&buf).unwrap();

        Ok(root.compound().unwrap())
    }

}


#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct NbtTagCompound {
    pub name: String,
    pub values: HashMap<String, NbtTag>,
}

impl NbtTagCompound {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            values: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<NbtTag> {
        self.values.get(name).cloned()
    }

    pub fn set(&mut self, name: &str, value: NbtTag) {
        self.values.insert(name.to_string(), value);
    }

    pub fn to_json<P: AsRef<std::path::Path>>(&self, path: P) -> io::Result<()> {
        // Open a file for writing.
        let file = fs::File::create(path)?;
        let writer = BufWriter::new(file); // Using a BufWriter for more efficient writes.

        // Write the pretty-printed JSON to the file.
        serde_json::to_writer_pretty(writer, &self)?;
        
        Ok(())
    }

    pub fn from_json<P: AsRef<std::path::Path>>(path: P) -> Result<Self, io::Error> {

        let file = fs::File::open(path)?;
        let reader = BufReader::new(file); // Wrap the file in a BufReader, since very large file are expected.

        // Deserialize the JSON data directly from the stream.
        let deserialized_nbt = serde_json::from_reader(reader)?;
        
        Ok(deserialized_nbt)
    }
}

#[derive(Clone, Debug, new, Default, Serialize, Deserialize)]
pub struct NbtTagIntArray {
    pub name: String,
    pub values: Vec<i32>,
}

#[derive(Clone, Debug, new, Default, Serialize, Deserialize)]
pub struct NbtTagLongArray {
    pub name: String,
    pub values: Vec<i64>,
}
