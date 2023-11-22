use byteorder::{BigEndian, WriteBytesExt};
use std::collections::HashMap;
use std::path::PathBuf;
use std::io::Write;
use serde::Serialize;
use serde::Deserialize;
use std::fs;
use std::io::{self, BufWriter, BufReader};
use derive_new::new;

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use pyo3::exceptions::{PyTypeError, PyKeyError};

#[cfg(test)]
mod tests;

#[pyclass]
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct NbtTagCompound {
    pub name: String,
    pub values: HashMap<String, NbtTag>,
}

#[pymethods]
impl NbtTagCompound {
    #[new]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            values: HashMap::new(),
        }
    }

/*     pub fn get(&self, name: &str) -> Option<NbtTag> {
        self.values.get(name).cloned()
    }

    pub fn set(&mut self, name: &str, value: NbtTag) {
        self.values.insert(name.to_string(), value);
    } */

/*     pub fn to_json<P: AsRef<std::path::Path>>(&self, path: P) -> io::Result<()> {
        // Open a file for writing.
        let file = fs::File::create(path)?;
        let writer = BufWriter::new(file); // Using a BufWriter for more efficient writes.

        // Write the pretty-printed JSON to the file.
        serde_json::to_writer_pretty(writer, &self)?;
        
        Ok(())
    } */

    /* pub fn to_json<P: AsRef<std::path::Path>>(&self, path: P) -> io::Result<()> {
        // Open a file for writing.
        let file = fs::File::create(path)?;
        let writer = BufWriter::new(file); // Using a BufWriter for more efficient writes.

        // Write the pretty-printed JSON to the file.
        serde_json::to_writer_pretty(writer, &self)?;
        
        Ok(())
    }
 */

   /*  pub fn from_json<P: AsRef<std::path::Path>>(path: P) -> Result<Self, io::Error> {

        let file = fs::File::open(path)?;
        let reader = BufReader::new(file); // Wrap the file in a BufReader, since very large file are expected.

        // Deserialize the JSON data directly from the stream.
        let deserialized_nbt = serde_json::from_reader(reader)?;
        
        Ok(deserialized_nbt)

    } */

/*     pub fn from_json(&self, path: String) -> PyResult<Self> {
        let path = PathBuf::from(path);
        let file = fs::File::open(&path)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("{}", e)))?;
        let reader = BufReader::new(file); // Wrap the file in a BufReader

        // Deserialize the JSON data directly from the stream.
        serde_json::from_reader(reader)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("{}", e)))
    } */
}

/// Represents the type of an NBT (Named Binary Tag) tag.
///
/// NBT is a tag-based binary format used to store structured data.
/// Each `NbtTagType` variant corresponds to a different data type
/// in the NBT specification.
#[pyclass]
#[derive(Clone, Copy, new,  Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

    pub fn from_id(id: u8) -> Option<Self> {
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

/// Represents an NBT (Named Binary Tag) tag.
///
/// This enum encapsulates all possible NBT tags, each variant holding
/// data corresponding to its type.
#[derive(Clone, new, Debug, Serialize, Deserialize)]
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

    //https://github.com/PyO3/pyo3/pull/3582 
    //ENUM Variants are not supported in PyO3. NbtTag like originally implemented cannot be seen in python
/*     pub fn new(nbt_type: NbtTagType)-> Self {
        
        match nbt_type {
            NbtTagType::End => NbtTag::End,
            NbtTagType::Byte => NbtTagByte::new(),
            NbtTagType::Short => NbtTagShort::new(),
            NbtTagType::Int => NbtTagInt::new(),
            NbtTagType::Long => NbtTagLong::new(),
            NbtTagType::Float => NbtTagFloat::new(),
            NbtTagType::Double => NbtTagDouble::new(),
            NbtTagType::ByteArray => NbtTagByteArray::new(),
            NbtTagType::String => NbtTagString::new(),
            NbtTagType::List => NbtTagList::new(),
            NbtTagType::Compound => NbtTagCompound::new(),
            NbtTagType::IntArray => NbtTagIntArray::new(),
            NbtTagType::LongArray => NbtTagLongArray::new()
        }
    } */

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


#[pyclass(get_all)]
#[derive(Clone, Debug)]
pub struct PyNbtTag {
    //pub nbt_tag: &'a NbtTag,
    pub python_dict: Py<PyDict>
}

impl PyNbtTag {

    pub fn new(nbt_tag: &NbtTag) -> Self {
        let python_dict = Self::to_python_dictionary(&nbt_tag);
        Self {
            python_dict
        }
    }

    fn to_python_dictionary(nbt_tag: & NbtTag) -> Py<PyDict> {
        
        Python::with_gil(|py| {
            let dict: Py<PyDict> = PyDict::new(py).into();
            // TODO: Get rid of all these unwraps

            match nbt_tag.ty() {
                NbtTagType::End => {
                    dict.as_ref(py).set_item("END_TAG", 0).unwrap();
                    dict
                },
                NbtTagType::Byte => {
                    let tag_byte = nbt_tag.byte().unwrap();
                    dict.as_ref(py).set_item(tag_byte.name, tag_byte.value).unwrap();
                    dict

                },
                NbtTagType::Short => {
                    let tag_short = nbt_tag.short().unwrap();
                    dict.as_ref(py).set_item(tag_short.name, tag_short.value).unwrap();
                    dict

                },
                NbtTagType::Int => {
                    let tag_int = nbt_tag.int().unwrap_or_default(); //error without default.
                    dict.as_ref(py).set_item(tag_int.name, tag_int.value).unwrap();
                    dict

                },
                NbtTagType::Long => {
                    let tag_long = nbt_tag.long().unwrap();
                    dict.as_ref(py).set_item(tag_long.name, tag_long.value).unwrap();
                    dict

                },
                NbtTagType::Float => {
                    let tag_float = nbt_tag.float().unwrap();
                    dict.as_ref(py).set_item(tag_float.name, tag_float.value).unwrap();
                    dict

                },
                NbtTagType::Double => {
                    let tag_double = nbt_tag.double().unwrap();
                    dict.as_ref(py).set_item(tag_double.name, tag_double.value).unwrap();
                    dict

                },
                NbtTagType::ByteArray => {
                    let tag_byte_array = nbt_tag.byte_array().unwrap();
                    dict.as_ref(py).set_item(tag_byte_array.name, tag_byte_array.values).unwrap();
                    dict

                },
                NbtTagType::String => {
                    let tag_string = nbt_tag.string().unwrap();
                    dict.as_ref(py).set_item(tag_string.name, tag_string.value).unwrap();
                    dict

                },
                NbtTagType::List => {
                    let tag_list = nbt_tag.list().unwrap();
                    let empty_object_array: &[PyObject] = &[];
                    let py_list: &PyList = PyList::new(py, empty_object_array);
                    //not efficient, i am processind the data two times, but for now make it work
                    for list_element in &tag_list.values {
                        let py_list_element = PyNbtTag::new(list_element);
                        let _ = py_list.append(py_list_element.python_dict);
                    }

                    dict.as_ref(py).set_item(tag_list.name, py_list).unwrap();
                    dict

                },
                NbtTagType::Compound => {
                    let tag_compound = nbt_tag.compound().unwrap();
                    //let empty_object_array: &[PyObject] = &[];
                    let py_dict: &PyDict = PyDict::new(py);

                    for (key, value) in tag_compound.values.iter() {
                        let py_tag = PyNbtTag::new(value);
                        let _ = py_dict.set_item(key, py_tag.python_dict);
                    }

                    dict.as_ref(py).set_item(tag_compound.name, py_dict).unwrap();
                    dict

                },
                NbtTagType::IntArray => {
                    let tag_int_array = nbt_tag.int_array().unwrap();
                    dict.as_ref(py).set_item(tag_int_array.name, tag_int_array.values).unwrap();
                    dict

                },
                NbtTagType::LongArray => {
                    let tag_long_array = nbt_tag.long_array().unwrap();
                    dict.as_ref(py).set_item(tag_long_array.name, tag_long_array.values).unwrap();
                    dict

                }
            }
        })
    }
}

#[pyclass]
#[derive(Clone, new, Debug, Default, Serialize, Deserialize)]
pub struct NbtTagByte {
    pub name: String,
    pub value: i8,
}

#[pyclass]
#[derive(Clone, new, Debug, Default, Serialize, Deserialize)]
pub struct NbtTagShort {
    pub name: String,
    pub value: i16,
}

#[pyclass]
#[derive(Clone, new, Debug, Default, Serialize, Deserialize)]
pub struct NbtTagInt {
    pub name: String,
    pub value: i32,
}

#[pyclass]
#[derive(Clone, new, Debug, Default, Serialize, Deserialize)]
pub struct NbtTagLong {
    pub name: String,
    pub value: i64,
}

#[pyclass]
#[derive(Clone, new, Debug, Default, Serialize, Deserialize)]
pub struct NbtTagFloat {
    pub name: String,
    pub value: f32,
}

#[pyclass]
#[derive(Clone, new, Debug, Default, Serialize, Deserialize)]
pub struct NbtTagDouble {
    pub name: String,
    pub value: f64,
}

#[pyclass]
#[derive(Clone, new, Debug, Default, Serialize, Deserialize)]
pub struct NbtTagByteArray {
    pub name: String,
    pub values: Vec<i8>,
}

#[pyclass]
#[derive(Clone, new, Debug, Default, Serialize, Deserialize)]
pub struct NbtTagString {
    pub name: String,
    pub value: String,
}

#[pyclass]
#[derive(Clone, new, Debug, Default, Serialize, Deserialize)]
pub struct NbtTagList {
    pub name: String,
    pub ty: NbtTagType,
    pub values: Vec<NbtTag>,
}

#[pyclass]
#[derive(Clone, new, Debug, Default, Serialize, Deserialize)]
pub struct NbtTagIntArray {
    pub name: String,
    pub values: Vec<i32>,
}

#[pyclass]
#[derive(Clone, new, Debug, Default, Serialize, Deserialize)]
pub struct NbtTagLongArray {
    pub name: String,
    pub values: Vec<i64>,
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
