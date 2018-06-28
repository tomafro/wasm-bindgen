#![feature(proc_macro, wasm_custom_section, wasm_import_module, used)]

extern crate wasm_bindgen;

mod foo {

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

}
