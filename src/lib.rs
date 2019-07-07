use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use std::collections::HashMap;
use std::io::Cursor;
use std::io::Write;

#[macro_use]
extern crate derive_new;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NbtValueType {
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

impl Default for NbtValueType {
    fn default() -> Self {
        NbtValueType::End
    }
}

impl NbtValueType {
    fn id(&self) -> u8 {
        match self {
            NbtValueType::End => 0,
            NbtValueType::Byte => 1,
            NbtValueType::Short => 2,
            NbtValueType::Int => 3,
            NbtValueType::Long => 4,
            NbtValueType::Float => 5,
            NbtValueType::Double => 6,
            NbtValueType::ByteArray => 7,
            NbtValueType::String => 8,
            NbtValueType::List => 9,
            NbtValueType::Compound => 10,
            NbtValueType::IntArray => 11,
            NbtValueType::LongArray => 12,
        }
    }

    fn from_id(id: u8) -> Option<Self> {
        match id {
            0 => Some(NbtValueType::End),
            1 => Some(NbtValueType::Byte),
            2 => Some(NbtValueType::Short),
            3 => Some(NbtValueType::Int),
            4 => Some(NbtValueType::Long),
            5 => Some(NbtValueType::Float),
            6 => Some(NbtValueType::Double),
            7 => Some(NbtValueType::ByteArray),
            8 => Some(NbtValueType::String),
            9 => Some(NbtValueType::List),
            10 => Some(NbtValueType::Compound),
            11 => Some(NbtValueType::IntArray),
            12 => Some(NbtValueType::LongArray),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub enum NbtValue {
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

impl Default for NbtValue {
    fn default() -> Self {
        NbtValue::End
    }
}

impl NbtValue {
    pub fn ty(&self) -> NbtValueType {
        match &self {
            NbtValue::End => NbtValueType::End,
            NbtValue::Byte(_) => NbtValueType::Byte,
            NbtValue::Short(_) => NbtValueType::Short,
            NbtValue::Int(_) => NbtValueType::Int,
            NbtValue::Long(_) => NbtValueType::Int,
            NbtValue::Float(_) => NbtValueType::Float,
            NbtValue::Double(_) => NbtValueType::Double,
            NbtValue::ByteArray(_) => NbtValueType::ByteArray,
            NbtValue::String(_) => NbtValueType::String,
            NbtValue::List(_) => NbtValueType::List,
            NbtValue::Compound(_) => NbtValueType::Compound,
            NbtValue::IntArray(_) => NbtValueType::IntArray,
            NbtValue::LongArray(_) => NbtValueType::End,
        }
    }

    pub fn byte(&self) -> Option<NbtTagByte> {
        if let NbtValue::Byte(x) = self {
            Some(x.clone())
        } else {
            None
        }
    }

    pub fn short(&self) -> Option<NbtTagShort> {
        if let NbtValue::Short(x) = self {
            Some(x.clone())
        } else {
            None
        }
    }

    pub fn int(&self) -> Option<NbtTagInt> {
        if let NbtValue::Int(x) = self {
            Some(x.clone())
        } else {
            None
        }
    }

    pub fn long(&self) -> Option<NbtTagLong> {
        if let NbtValue::Long(x) = self {
            Some(x.clone())
        } else {
            None
        }
    }

    pub fn float(&self) -> Option<NbtTagFloat> {
        if let NbtValue::Float(x) = self {
            Some(x.clone())
        } else {
            None
        }
    }

    pub fn double(&self) -> Option<NbtTagDouble> {
        if let NbtValue::Double(x) = self {
            Some(x.clone())
        } else {
            None
        }
    }

    pub fn byte_array(&self) -> Option<NbtTagByteArray> {
        if let NbtValue::ByteArray(x) = self {
            Some(x.clone())
        } else {
            None
        }
    }

    pub fn string(&self) -> Option<NbtTagString> {
        if let NbtValue::String(x) = self {
            Some(x.clone())
        } else {
            None
        }
    }

    pub fn list(&self) -> Option<NbtTagList> {
        if let NbtValue::List(x) = self {
            Some(x.clone())
        } else {
            None
        }
    }

    pub fn compound(&self) -> Option<NbtTagCompound> {
        if let NbtValue::Compound(x) = self {
            Some(x.clone())
        } else {
            None
        }
    }

    pub fn int_array(&self) -> Option<NbtTagIntArray> {
        if let NbtValue::IntArray(x) = self {
            Some(x.clone())
        } else {
            None
        }
    }

    pub fn long_array(&self) -> Option<NbtTagLongArray> {
        if let NbtValue::LongArray(x) = self {
            Some(x.clone())
        } else {
            None
        }
    }
}

pub fn parse_bytes(bytes: &[u8]) -> Result<NbtValue, ()> {
    let mut cursor = Cursor::new(bytes);
    parse(&mut cursor)
}

pub fn parse(cursor: &mut Cursor<&[u8]>) -> Result<NbtValue, ()> {
    // Read root compound - read type first
    let ty = {
        let id = cursor.read_u8().map_err(|_| ())?;
        NbtValueType::from_id(id).ok_or_else(|| ())?
    };
    if ty != NbtValueType::Compound {
        return Err(());
    }

    let name_len = cursor.read_i16::<BigEndian>().map_err(|_| ())?;
    let mut name = String::with_capacity(name_len as usize);
    for _ in 0..name_len {
        let ch = cursor.read_u8().map_err(|_| ())?;
        name.push(ch as char);
    }

    let root = parse_compound(cursor, name)?;

    Ok(NbtValue::Compound(root))
}

fn parse_compound(cursor: &mut Cursor<&[u8]>, name: String) -> Result<NbtTagCompound, ()> {
    let mut compound = NbtTagCompound::new(name.as_str());

    println!("Reading compound {}", compound.name);

    // Read values until NBT_End is reached
    loop {
        let type_id = cursor.read_u8().map_err(|_| ())?;
        println!("111");

        let ty = NbtValueType::from_id(type_id).ok_or_else(|| ())?;
        if ty == NbtValueType::End {
            // Finish early - nothing more to read
            println!("Terminating compound");
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
        println!("Reading tag with type {:?} and name {}", ty, name);

        // Read value
        let value = parse_value(cursor, ty, name.clone())?;

        println!("136");

        compound.values.insert(name, value);
    }

    Ok(compound)
}

fn parse_list(cursor: &mut Cursor<&[u8]>, name: String) -> Result<NbtTagList, ()> {
    // Type of values contained in the list
    let ty = {
        let id = cursor.read_u8().map_err(|_| ())?;
        NbtValueType::from_id(id).ok_or_else(|| ())?
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

fn parse_value(cursor: &mut Cursor<&[u8]>, ty: NbtValueType, name: String) -> Result<NbtValue, ()> {
    Ok(match ty {
        NbtValueType::End => unreachable!(), // Should already be covered
        NbtValueType::Byte => {
            let x = cursor.read_i8().map_err(|_| ())?;
            NbtValue::Byte(NbtTagByte::new(name.clone(), x))
        }
        NbtValueType::Short => {
            let x = cursor.read_i16::<BigEndian>().map_err(|_| ())?;
            NbtValue::Short(NbtTagShort::new(name.clone(), x))
        }
        NbtValueType::Int => {
            let x = cursor.read_i32::<BigEndian>().map_err(|_| ())?;
            NbtValue::Int(NbtTagInt::new(name.clone(), x))
        }
        NbtValueType::Long => {
            let x = cursor.read_i64::<BigEndian>().map_err(|_| ())?;
            NbtValue::Long(NbtTagLong::new(name.clone(), x))
        }
        NbtValueType::Float => {
            let x = cursor.read_f32::<BigEndian>().map_err(|_| ())?;
            NbtValue::Float(NbtTagFloat::new(name.clone(), x))
        }
        NbtValueType::Double => {
            let x = cursor.read_f64::<BigEndian>().map_err(|_| ())?;
            NbtValue::Double(NbtTagDouble::new(name.clone(), x))
        }
        NbtValueType::ByteArray => {
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

            NbtValue::ByteArray(NbtTagByteArray::new(name.clone(), buf))
        }
        NbtValueType::String => {
            let len = cursor.read_u16::<BigEndian>().map_err(|_| ())?;
            let mut buf = String::with_capacity(len as usize);

            for _ in 0..len {
                let ch = cursor.read_u8().map_err(|_| ())?;
                buf.push(ch as char);
            }

            NbtValue::String(NbtTagString::new(name.clone(), buf))
        }
        NbtValueType::List => {
            let list = parse_list(cursor, name)?;
            NbtValue::List(list)
        }
        NbtValueType::Compound => {
            let compound = parse_compound(cursor, name)?;
            NbtValue::Compound(compound)
        }
        NbtValueType::IntArray => {
            let len = cursor.read_i32::<BigEndian>().map_err(|_| ())?;
            if len > 65536 {
                return Err(());
            }

            let mut buf = Vec::with_capacity(len as usize);
            for _ in 0..len {
                let x = cursor.read_i32::<BigEndian>().map_err(|_| ())?;
                buf.push(x);
            }

            NbtValue::IntArray(NbtTagIntArray::new(name.clone(), buf))
        }
        NbtValueType::LongArray => {
            let len = cursor.read_i32::<BigEndian>().map_err(|_| ())?;
            if len > 65536 {
                return Err(());
            }

            let mut buf = Vec::with_capacity(len as usize);
            for _ in 0..len {
                let x = cursor.read_i64::<BigEndian>().map_err(|_| ())?;
                buf.push(x);
            }

            NbtValue::LongArray(NbtTagLongArray::new(name.clone(), buf))
        }
    })
}

pub fn write(buf: &mut Vec<u8>, compound: &NbtTagCompound) {
    write_tag_type(buf, NbtValueType::Compound);
    write_tag_name(buf, &compound.name);
    write_compound(buf, compound);
}

fn write_compound(buf: &mut Vec<u8>, compound: &NbtTagCompound) {
    for val in compound.values.values() {
        write_value(buf, val, true);
    }
}

fn write_value(buf: &mut Vec<u8>, value: &NbtValue, write_name: bool) {
    let ty = value.ty();
    write_tag_type(buf, ty);

    match value {
        NbtValue::End => (),
        NbtValue::Byte(val) => {
            if write_name {
                write_tag_name(buf, &val.name);
            }
            buf.write_i8(val.value).unwrap();
        }
        NbtValue::Short(val) => {
            if write_name {
                write_tag_name(buf, &val.name);
            }
            buf.write_i16::<BigEndian>(val.value).unwrap();
        }
        NbtValue::Int(val) => {
            if write_name {
                write_tag_name(buf, &val.name);
            }
            buf.write_i32::<BigEndian>(val.value).unwrap();
        }
        NbtValue::Long(val) => {
            if write_name {
                write_tag_name(buf, &val.name);
            }
            buf.write_i64::<BigEndian>(val.value).unwrap();
        }
        NbtValue::Float(val) => {
            if write_name {
                write_tag_name(buf, &val.name);
            }
            buf.write_f32::<BigEndian>(val.value).unwrap();
        }
        NbtValue::Double(val) => {
            if write_name {
                write_tag_name(buf, &val.name);
            }
            buf.write_f64::<BigEndian>(val.value).unwrap();
        }
        NbtValue::ByteArray(val) => {
            if write_name {
                write_tag_name(buf, &val.name);
            }

            buf.write_i16::<BigEndian>(val.values.len() as i16).unwrap();
            buf.reserve(val.values.len());

            for x in &val.values {
                buf.write_i8(*x).unwrap();
            }
        }
        NbtValue::String(val) => {
            if write_name {
                write_tag_name(buf, &val.name);
            }

            buf.write_u16::<BigEndian>(val.value.len() as u16).unwrap();
            buf.write(val.value.as_bytes()).unwrap();
        }
        NbtValue::List(val) => {
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
        NbtValue::Compound(val) => {
            if write_name {
                write_tag_name(buf, &val.name);
            }

            write_compound(buf, val);
        }
        NbtValue::IntArray(val) => {
            if write_name {
                write_tag_name(buf, &val.name);
            }

            buf.write_i32::<BigEndian>(val.values.len() as i32).unwrap();

            buf.reserve(val.values.len());

            for x in &val.values {
                buf.write_i32::<BigEndian>(*x).unwrap();
            }
        }
        NbtValue::LongArray(val) => {
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

fn write_tag_type(buf: &mut Vec<u8>, ty: NbtValueType) {
    buf.write_u8(ty.id()).unwrap();
}

#[derive(Clone, Debug, new, Default)]
pub struct NbtTagByte {
    pub name: String,
    pub value: i8,
}

#[derive(Clone, Debug, new, Default)]
pub struct NbtTagShort {
    pub name: String,
    pub value: i16,
}

#[derive(Clone, Debug, new, Default)]
pub struct NbtTagInt {
    pub name: String,
    pub value: i32,
}

#[derive(Clone, Debug, new, Default)]
pub struct NbtTagLong {
    pub name: String,
    pub value: i64,
}

#[derive(Clone, Debug, new, Default)]
pub struct NbtTagFloat {
    pub name: String,
    pub value: f32,
}

#[derive(Clone, Debug, new, Default)]
pub struct NbtTagDouble {
    pub name: String,
    pub value: f64,
}

#[derive(Clone, Debug, new, Default)]
pub struct NbtTagByteArray {
    pub name: String,
    pub values: Vec<i8>,
}

#[derive(Clone, Debug, new, Default)]
pub struct NbtTagString {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Debug, new, Default)]
pub struct NbtTagList {
    pub name: String,
    pub ty: NbtValueType,
    pub values: Vec<NbtValue>,
}

#[derive(Clone, Debug, Default)]
pub struct NbtTagCompound {
    pub name: String,
    values: HashMap<String, NbtValue>,
}

impl NbtTagCompound {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            values: HashMap::new(),
        }
    }

    pub fn get(&self, name: &str) -> Option<NbtValue> {
        self.values.get(name).cloned()
    }

    pub fn set(&mut self, name: &str, value: NbtValue) {
        self.values.insert(name.to_string(), value);
    }
}

#[derive(Clone, Debug, new, Default)]
pub struct NbtTagIntArray {
    pub name: String,
    pub values: Vec<i32>,
}

#[derive(Clone, Debug, new, Default)]
pub struct NbtTagLongArray {
    pub name: String,
    pub values: Vec<i64>,
}
