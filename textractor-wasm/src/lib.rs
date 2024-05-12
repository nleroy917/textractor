mod utils;

use textractor::extraction::extract;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn extract_text(data: &[u8]) -> String {
    utils::set_panic_hook();
    extract(data).unwrap_or(Some("".to_string())).unwrap()
}
