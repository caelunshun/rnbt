use byteorder::{BigEndian, ReadBytesExt};
use std::collections::HashMap;
use std::io::Cursor;

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

pub fn parse(bytes: &[u8]) -> Result<NbtValue, ()> {
    let mut cursor = Cursor::new(bytes);

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

    let root = parse_compound(&mut cursor, name)?;

    Ok(NbtValue::Compound(root))
}

fn parse_compound(cursor: &mut Cursor<&[u8]>, name: String) -> Result<NbtTagCompound, ()> {
    let mut compound = NbtTagCompound::new(name, HashMap::new());

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

#[derive(Clone, Debug, new)]
pub struct NbtTagByte {
    pub name: String,
    pub value: i8,
}

#[derive(Clone, Debug, new)]
pub struct NbtTagShort {
    pub name: String,
    pub value: i16,
}

#[derive(Clone, Debug, new)]
pub struct NbtTagInt {
    pub name: String,
    pub value: i32,
}

#[derive(Clone, Debug, new)]
pub struct NbtTagLong {
    pub name: String,
    pub value: i64,
}

#[derive(Clone, Debug, new)]
pub struct NbtTagFloat {
    pub name: String,
    pub value: f32,
}

#[derive(Clone, Debug, new)]
pub struct NbtTagDouble {
    pub name: String,
    pub value: f64,
}

#[derive(Clone, Debug, new)]
pub struct NbtTagByteArray {
    pub name: String,
    pub values: Vec<i8>,
}

#[derive(Clone, Debug, new)]
pub struct NbtTagString {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Debug, new)]
pub struct NbtTagList {
    pub name: String,
    pub ty: NbtValueType,
    pub values: Vec<NbtValue>,
}

#[derive(Clone, Debug, new)]
pub struct NbtTagCompound {
    pub name: String,
    pub values: HashMap<String, NbtValue>,
}

impl NbtTagCompound {
    pub fn get(&self, name: &str) -> Option<NbtValue> {
        self.values.get(name).cloned()
    }
}

#[derive(Clone, Debug, new)]
pub struct NbtTagIntArray {
    pub name: String,
    pub values: Vec<i32>,
}

#[derive(Clone, Debug, new)]
pub struct NbtTagLongArray {
    pub name: String,
    pub values: Vec<i64>,
}
