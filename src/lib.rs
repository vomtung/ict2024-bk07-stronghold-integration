use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet() -> String {
    // Trả về chuỗi "Hello from Rust!"
    "Hello from Rust!".to_string()
}
