use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

mod lib; 

#[pymodule]
fn fast_nbt(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_from_binary_file, m)?)?;
    Ok(())
}

#[pyfunction]
fn read_from_binary_file() -> PyResult<String> {
    // Call your actual Rust function here and return the result.
    Ok("Hello from Rust!".to_string())
}