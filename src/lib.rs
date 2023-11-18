use std::path::PathBuf;

pub mod nbt_tag;
pub mod file_parser;
pub mod region;
pub mod generic_bin;

use nbt_tag::NbtTagCompound;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

#[pymodule]
fn rnbt(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<McWorldDescriptor>()?;
    m.add_class::<nbt_tag::NbtTagCompound>()?;
    m.add_function(wrap_pyfunction!(load_binary, m)?)?;
    Ok(())
}

#[pyfunction]
fn load_binary(input_path: String) -> PyResult<McWorldDescriptor> {   
    let path_buf = PathBuf::from(input_path);
    McWorldDescriptor::new(path_buf).map_err(|e| PyErr::new::<pyo3::exceptions::PyIOError, _>(format!("{}", e)))   
}

#[pyclass(get_all)]
#[derive(Clone, Debug, Default)]
pub struct McWorldDescriptor {
    pub input_path: PathBuf,
    pub version: String,
    pub tag_compounds_list: Vec<nbt_tag::NbtTagCompound>,
}

#[pymethods]
impl McWorldDescriptor {
    #[new]
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