//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

use textractor_wasm::extract_text;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    let data = include_bytes!("../tests/fixtures/sample.pdf");
    let text = extract_text(data);
}
