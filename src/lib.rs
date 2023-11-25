pub mod nbt_tag;
pub mod file_parser;
pub mod region;
pub mod generic_bin;



use std::collections::HashMap;
use std::fs;
use std::io::{self, BufWriter, BufReader};
use std::path::PathBuf;
use nbt_tag::{NbtTagCompound, SerializablePyDict};
use nbt_tag::PyNbtTag;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::wrap_pyfunction;
use log::info;
use pyo3_log;

#[pymodule]
fn rnbt(py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();
    m.add_class::<PyMcWorldDescriptor>()?;
    m.add_class::<nbt_tag::PyNbtTag>()?;
    m.add_function(wrap_pyfunction!(load_binary, m)?)?;
    m.add_function(wrap_pyfunction!(py_log, m)?)?;

    Ok(())
}
#[pyfunction]
fn py_log(message: String)  {
    info!("{}", message);
}



#[pyfunction]
fn load_binary(input_path: String) -> PyResult<PyMcWorldDescriptor> {   
    let path_buf = PathBuf::from(input_path);
    let mc_world = McWorldDescriptor::new(path_buf)?; 
    PyMcWorldDescriptor::new(mc_world).map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("{}", e)))
}

#[pyclass]
#[derive(Clone, Debug)]
pub struct PyMcWorldDescriptor {
    /* #[pyo3(get, set)]
    pub input_path: String,
    #[pyo3(get, set)]
    pub version: String,
    #[pyo3(get, set)]
    pub tag_compounds_list: Vec::<Py<PyDict>>, */
    mc_world_descriptor: McWorldDescriptor,
    //TEST
    #[pyo3(get, set)]
    pub ser_tag_compounts_list: Vec::<nbt_tag::SerializablePyDict>
}

#[pymethods]
impl PyMcWorldDescriptor {
    #[new]
    pub fn new(rust_mc_world_descriptor: McWorldDescriptor) -> std::io::Result<Self> {
        
        //let path_str = rust_mc_world_descriptor.input_path.to_str().unwrap().to_string(); 
        let mut py_tag_list = Vec::<nbt_tag::SerializablePyDict>::new();
        
        rust_mc_world_descriptor.tag_compounds_list.iter().for_each(|item| {
            let tag_root = nbt_tag::NbtTag::Compound(item.clone());
            py_tag_list.push(PyNbtTag::new(&tag_root).ser_python_dict)
        });
    
        //TEST
        //let py_tag_list_clone = py_tag_list.clone();

        Ok(PyMcWorldDescriptor{ 
            mc_world_descriptor: rust_mc_world_descriptor, 
            ser_tag_compounts_list: py_tag_list 
        })
        /* Ok(PyMcWorldDescriptor {
            input_path: path_str,
            version: "0.0.0".to_string(),
            tag_compounds_list: py_tag_list.into_iter().map(|s| s.get_py_dict().clone()).collect(),
            ser_tag_compounts_list: py_tag_list_clone
        }) */
    }

    pub fn to_json(&self, path: String) -> PyResult<()> {

        self.mc_world_descriptor.to_json(path).map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("{}", e)))
        // Open a file for writing.
/*         let file = fs::File::create(path)?;
        let writer = BufWriter::new(file); // Using a BufWriter for more efficient writes.

        // Write the pretty-printed JSON to the file.
        serde_json::to_writer_pretty(writer, &self.ser_tag_compounts_list)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("{}", e)))?;
    
            Ok(())
 */
    }

    /* pub fn from_json(&self, path: String) -> PyResult<Self> {
        let path = PathBuf::from(path);
        let file = fs::File::open(&path)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("{}", e)))?;
        let reader = BufReader::new(file); // Wrap the file in a BufReader

        // Deserialize the JSON data directly from the stream.
        let tag_compound = serde_json::from_reader(reader)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("{}", e)));
    } */


}

#[pyclass]
#[derive(Clone, Debug, Default)]
pub struct McWorldDescriptor {
    pub input_path: PathBuf,
    pub version: String,
    pub tag_compounds_list: Vec<nbt_tag::NbtTagCompound>,
}

impl McWorldDescriptor {
    pub fn new(input_path: PathBuf) -> std::io::Result<Self> {
        let cloned_input_path = input_path.clone();
        //let tag_compounds_list = Self::read_from_binary_file(input_path)?;
        //let tag_compounds_list = Vec::<nbt_tag::NbtTagCompound>::new();
        //let tag_compounds_list = Self::read_from_binary_file(&input_path)
        //    .map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("{}", e)))?;

        if let Some(ext) = input_path.extension().and_then(|e| e.to_str()) {
            
            let mut nbt_tag_compounds_list = Vec::<nbt_tag::NbtTagCompound>::new();

            if ext == "mcr" || ext == "mca" {
                let region_file = region::RegionFile::new(input_path)?;
                nbt_tag_compounds_list = match region_file.to_compounds_list(){
                    Ok(c) => c,
                    Err(e) => return Err(e),
                }
            }
            else if ext == "nbt" || ext == "litematic" {
                let bin_content = generic_bin::GenericBinFile::new(input_path, generic_bin::FileType::Nbt)?;
                nbt_tag_compounds_list = match bin_content.to_compounds_list(){
                    Ok(c) => c,
                    Err(e) => return Err(e),
                }
            }
            Ok(McWorldDescriptor {
                input_path: cloned_input_path,
                version: "0.0.0".to_string(),
                tag_compounds_list: nbt_tag_compounds_list,
            })
        }
        else{
            //TODO: read a file not only based on the extension, but checking the internal format
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Unsupported file type"))
        } 

        
    }


        pub fn to_json<P: AsRef<std::path::Path>>(&self, path: P) -> io::Result<()> {
        // Open a file for writing.
        let file = fs::File::create(path)?;
        let writer = BufWriter::new(file); // Using a BufWriter for more efficient writes.

        // Write the pretty-printed JSON to the file.
        serde_json::to_writer_pretty(writer, &self.tag_compounds_list)?;
        
        Ok(())
    } 

    /* fn read_from_binary_file(input_path: PathBuf) -> std::io::Result<Vec<nbt_tag::NbtTagCompound>> {
        if let Some(ext) = input_path.extension().and_then(|e| e.to_str()) {
            
            let mut nbt_tag_compounds_list = Vec::<nbt_tag::NbtTagCompound>::new();

            if ext == "mcr" || ext == "mca" {
                let region_file = region::RegionFile::new(input_path)?;
                nbt_tag_compounds_list = match region_file.to_compounds_list(){
                    Ok(c) => c,
                    Err(e) => return Err(e),
                }
            }
            else if ext == "nbt" || ext == "litematic" {
                let bin_content = generic_bin::GenericBinFile::new(input_path, generic_bin::FileType::Nbt)?;
                nbt_tag_compounds_list = match bin_content.to_compounds_list(){
                    Ok(c) => c,
                    Err(e) => return Err(e),
                }
            }
            Ok(nbt_tag_compounds_list)
        }
        else{
            //TODO: read a file not only based on the extension, but checking the internal format
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Unsupported file type"))
        } 

        
    }*/

}