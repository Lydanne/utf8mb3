extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn encode(input_str: &str) -> String {
    utf8mb3::encode(input_str)
}

#[wasm_bindgen]
pub fn decode(input_str: &str) -> String {
    utf8mb3::decode(input_str)
}

#[wasm_bindgen]
pub fn include_encode_utf8mb3(input_str: &str) -> bool {
    utf8mb3::include_encode_utf8mb3(input_str)
}

#[wasm_bindgen]
pub fn include_utf8mb4(input_str: &str) -> bool {
    utf8mb3::include_utf8mb4(input_str)
}
