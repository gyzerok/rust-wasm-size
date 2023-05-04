mod parser;
mod utils;

use nom::error::ErrorKind;
use parser::root;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-game-of-life!");
}

#[wasm_bindgen]
pub fn parse(s: &str) {
    match root::<(&str, ErrorKind)>(s) {
        Ok(_) => alert("Parsing ok"),
        Err(_) => alert("Parsing err"),
    };
}
