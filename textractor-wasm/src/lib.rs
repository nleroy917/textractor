mod utils;

use textractor::extraction::extract;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn extract_text(data: &[u8]) -> String {
    utils::set_panic_hook();
    let res = extract(data);
    match res {
        Ok(text) => {
            match text {
                Some(t) => t,
                None => "No text extracted".to_string(),
            }
            
        },
        Err(e) => format!("Error: {}", e),
    }
}
