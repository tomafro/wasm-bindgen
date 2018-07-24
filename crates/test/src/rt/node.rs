//! Support for printing status information of a test suite in node.js
//!
//! This currently uses the same output as `libtest`, only reimplemented here
//! for node itself.

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{Promise, JsFuture};
use futures::future::{self, Future};
use js_sys::eval;

pub struct Node {
    fs: NodeFs,
}

#[wasm_bindgen]
extern {
    type NodeFs;
    #[wasm_bindgen(method, js_name = writeSync, structural)]
    fn write_sync(this: &NodeFs, fd: i32, data: &[u8]);

    // Not using `js_sys::Error` because node's errors specifically have a
    // `stack` attribute.
    type NodeError;
    #[wasm_bindgen(method, getter, js_class = "Error", structural)]
    fn stack(this: &NodeError) -> String;
}

impl Node {
    pub fn new() -> Box<Future<Item = Option<Node>, Error = JsValue>> {
        if super::detect::is_browser() {
            return Box::new(future::ok(None))
        }

        let import = eval("import(\"fs\")").unwrap();

        Box::new(
            JsFuture::from(Promise::from(import))
                .map(NodeFs::from)
                .map(|fs| Node { fs })
                .map(Some)
        )
    }
}

impl super::Formatter for Node {
    fn writeln(&self, line: &str) {
        super::console_log(line);
    }

    fn log_start(&self, name: &str) {
        let data = format!("test {} ... ", name);
        self.fs.write_sync(2, data.as_bytes());
    }

    fn log_success(&self) {
        self.fs.write_sync(2, b"ok\n");
    }

    fn log_ignored(&self) {
        self.fs.write_sync(2, b"ignored\n");
    }

    fn log_failure(&self, err: JsValue) -> String {
        self.fs.write_sync(2, b"ignored\n");
        // TODO: should do a checked cast to `NodeError`
        NodeError::from(err).stack()
    }
}
