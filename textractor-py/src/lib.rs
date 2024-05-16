use pyo3::prelude::*;

pub mod extraction;

/// A Python module implemented in Rust.
#[pymodule]
fn textractors(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(extraction::py_extract_text_from_file, m)?)?;
    m.add_function(wrap_pyfunction!(extraction::py_extract_text_from_bytes, m)?)?;
    Ok(())
}
