// main lib file for the rust core module

use pyo3::prelude::*; // importing necessary items from the pyo3 crate to create Python bindings

// declaring the submodules for core, ffi, and layout_result
mod ffi;
mod core;

#[pymodule]
fn rust_core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(ffi::layout_api::compute_layout_dto, m)?)?;
    Ok(())
}