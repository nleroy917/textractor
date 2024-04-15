use pyo3::prelude::*;

use std::io::Read;

use anyhow::Result;

use textractor::extraction::extract;

#[pyfunction(name = "extract_text_from_file")]
pub fn py_extract_text_from_file(path: String) -> Result<String> {
    let path = std::path::Path::new(&path);
    let file = std::fs::File::open(path)?;
    let mut reader = std::io::BufReader::new(file);
    let mut data = Vec::new();

    reader.read_to_end(&mut data)?;

    let text = extract(&data)?;

    match text {
        Some(text) => Ok(text),
        None => Err(anyhow::anyhow!("Unsupported file type")),
    }
}

#[pyfunction(name = "extract_text_from_bytes")]
pub fn py_extract_text_from_bytes(data: &[u8]) -> Result<String> {
    let text = extract(data)?;

    match text {
        Some(text) => Ok(text),
        None => Err(anyhow::anyhow!("Unsupported file type")),
    }
}
