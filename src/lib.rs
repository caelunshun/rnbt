use byteorder::BigEndian;
use std::collections::HashMap;
use std::io::Cursor;

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

pub fn parse(bytes: &[u8]) -> Result<NbtValue, ()> {
    let mut cursor = Cursor::new(bytes);

    let mut finished = false;

    // Read root compound
    let root = {
        let type_id = cursor.read_u8().map_err(|_| ())?;
        if NbtValueType::from_id(type_id) != NbtValueType::Compound {
            return Err(());
        }

        let name_len = cursor.read_i16::<BigEndian>().map_err(|_| ())?;
        let mut name = String::with_capacity(name_len as usize);
        for _ in 0..name_len {
            let ch = cursor.read_u8().map_err(|_| ())?;
            name.push(ch as char);
        }
    };

    while !finished {}

    Ok(NbtValue::Compound(root))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, new)]
pub struct NbtTagByte {
    pub name: String,
    pub value: i8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, new)]
pub struct NbtTagShort {
    pub name: String,
    pub value: i16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, new)]
pub struct NbtTagInt {
    pub name: String,
    pub value: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, new)]
pub struct NbtTagLong {
    pub name: String,
    pub value: i64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, new)]
pub struct NbtTagFloat {
    pub name: String,
    pub value: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, new)]
pub struct NbtTagDouble {
    pub name: String,
    pub value: f64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, new)]
pub struct NbtTagByteArray {
    pub name: String,
    pub values: Vec<i8>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, new)]
pub struct NbtTagString {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NbtTagList {
    pub name: String,
    pub ty: NbtValueType,
    pub values: Vec<NbtValue>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, new)]
pub struct NbtTagCompound {
    pub name: String,
    pub values: HashMap<String, NbtValue>,
}

impl NbtTagCompound {
    pub fn get(&self, name: &str) -> Option<NbtValue> {
        self.values.get(name).cloned()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, new)]
pub struct NbtTagIntArray {
    pub name: String,
    pub values: Vec<i32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, new)]
pub struct NbtTagLongArray {
    pub name: String,
    pub values: Vec<i64>,
}
