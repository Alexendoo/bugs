#![feature(use_extern_macros)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub extern fn add(a: i32, b: i32) -> String {
    format!("{} + {} = {}", a, b, a + b)
}