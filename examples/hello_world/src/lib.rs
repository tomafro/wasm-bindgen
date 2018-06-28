#![feature(proc_macro, wasm_custom_section, wasm_import_module, used)]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

mod foo {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        pub fn alert(s: &str);
    }
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    foo::alert(&format!("Hello, {}!", name));
}
