#![doc(hidden)]

use std::cell::{RefCell, Cell};
use std::fmt;
use std::mem;

use console_error_panic_hook;
use futures::future::Future;
use js_sys::{Array, Function};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{self, Promise};

pub mod node;
pub mod browser;
pub mod detect;

/// Runtime test harness support instantiated in JS.
///
/// The node.js entry script instantiates a `Context` here which is used to
/// drive test execution.
#[wasm_bindgen]
pub struct Context {
    filter: Option<String>,
    current_test: RefCell<Option<String>>,
    succeeded: Cell<usize>,
    ignored: Cell<usize>,
    failures: RefCell<Vec<(String, String)>>,
    current_log: RefCell<String>,
    current_error: RefCell<String>,
    ignore_this_test: Cell<bool>,
    formatter: Box<Formatter>,
}

trait Formatter {
    fn writeln(&self, line: &str);
    fn log_start(&self, name: &str);
    fn log_success(&self);
    fn log_ignored(&self);
    fn log_failure(&self, err: JsValue) -> String;
}

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    #[doc(hidden)]
    pub fn console_log(s: &str);

    // General-purpose conversion into a `String`.
    #[wasm_bindgen(js_name = String)]
    fn stringify(val: &JsValue) -> String;
}

pub fn log(args: &fmt::Arguments) {
    console_log(&args.to_string());
}

#[wasm_bindgen]

impl Context {
    pub fn new() -> Promise {
        console_error_panic_hook::set_once();

        let context = node::Node::new()
            .map(|node| {
                match node {
                    Some(node) => Box::new(node) as Box<Formatter>,
                    None => Box::new(browser::Browser::new()),
                }
            })
            .map(|formatter| {
                Context {
                    filter: None,
                    current_test: RefCell::new(None),
                    succeeded: Cell::new(0),
                    ignored: Cell::new(0),
                    failures: RefCell::new(Vec::new()),
                    current_log: RefCell::new(String::new()),
                    current_error: RefCell::new(String::new()),
                    ignore_this_test: Cell::new(false),
                    formatter,
                }.into()
            });

        wasm_bindgen_futures::rust2js(context)
    }

    /// Inform this context about runtime arguments passed to the test
    /// harness.
    ///
    /// Eventually this will be used to support flags, but for now it's just
    /// used to support test filters.
    pub fn args(&mut self, args: Vec<JsValue>) {
        for arg in args {
            let arg = arg.as_string().unwrap();
            if arg.starts_with("-") {
                panic!("flag {} not supported", arg);
            } else if self.filter.is_some() {
                panic!("more than one filter argument cannot be passed");
            }
            self.filter = Some(arg);
        }
    }

    /// Executes a list of tests, returning whether any of them failed.
    ///
    /// This is the main entry point for executing tests. All the tests passed
    /// in are the JS `Function` object that was plucked off the
    /// `WebAssembly.Instance` exports list. This allows us to invoke it but
    /// still catch JS exceptions.
    pub fn run(&self, tests: Vec<JsValue>) -> bool {
        let this = JsValue::null();
        let args = Array::new();
        args.push(JsValue::from(self as *const Context as u32));

        let noun = if tests.len() == 1 { "test" } else { "tests" };
        self.formatter.writeln(&format!("running {} {}", tests.len(), noun));
        self.formatter.writeln("");

        for test in tests {
            self.ignore_this_test.set(false);
            let test = Function::from(test);
            match test.apply(&this, &args) {
                Ok(_) => {
                    if self.ignore_this_test.get() {
                        self.log_ignore()
                    } else {
                        self.log_success()
                    }
                }
                Err(e) => self.log_failure(e),
            }
            drop(self.current_test.borrow_mut().take());
            *self.current_log.borrow_mut() = String::new();
            *self.current_error.borrow_mut() = String::new();
        }
        self.log_results();
        self.failures.borrow().len() == 0
    }

    fn log_start(&self, test: &str) {
        let mut current_test = self.current_test.borrow_mut();
        assert!(current_test.is_none());
        *current_test = Some(test.to_string());
        self.formatter.log_start(test);
    }

    fn log_success(&self) {
        self.formatter.log_success();
        self.succeeded.set(self.succeeded.get() + 1);
    }

    fn log_ignore(&self) {
        self.formatter.log_ignored();
        self.ignored.set(self.ignored.get() + 1);
    }

    fn log_failure(&self, err: JsValue) {
        let name = self.current_test.borrow().as_ref().unwrap().clone();
        let log = mem::replace(&mut *self.current_log.borrow_mut(), String::new());
        let error = mem::replace(&mut *self.current_error.borrow_mut(), String::new());
        let mut msg = String::new();
        if log.len() > 0 {
            msg.push_str("log output:\n");
            msg.push_str(&tab(&log));
            msg.push_str("\n");
        }
        if error.len() > 0 {
            msg.push_str("error output:\n");
            msg.push_str(&tab(&error));
            msg.push_str("\n");
        }
        msg.push_str("JS exception that was thrown:\n");
        msg.push_str(&tab(&self.formatter.log_failure(err)));
        self.failures.borrow_mut().push((name, msg));
    }

    fn log_results(&self) {
        let failures = self.failures.borrow();
        if failures.len() > 0 {
            self.formatter.writeln("\nfailures:\n");
            for (test, logs) in failures.iter() {
                let msg = format!("---- {} output ----\n{}", test, tab(logs));
                self.formatter.writeln(&msg);
            }
            self.formatter.writeln("failures:\n");
            for (test, _) in failures.iter() {
                self.formatter.writeln(&format!("    {}", test));
            }
        }
        self.formatter.writeln("");
        self.formatter.writeln(&format!(
            "test result: {}. \
             {} passed; \
             {} failed; \
             {} ignored\n",
            if failures.len() == 0 { "ok" } else { "FAILED" },
            self.succeeded.get(),
            failures.len(),
            self.ignored.get(),
        ));
    }

    pub fn console_log(&self, original: &Function, args: &Array) {
        self.log(original, args, &self.current_log)
    }

    pub fn console_error(&self, original: &Function, args: &Array) {
        self.log(original, args, &self.current_error)
    }

    fn log(&self, orig: &Function, args: &Array, dst: &RefCell<String>) {
        if self.current_test.borrow().is_none() {
            drop(orig.apply(&JsValue::null(), args));
            return
        }
        let mut log = dst.borrow_mut();
        args.for_each(&mut |val, idx, _array| {
            if idx != 0 {
                log.push_str(" ");
            }
            log.push_str(&stringify(&val));
        });
        log.push_str("\n");
    }
}

impl Context {
    pub fn execute(&self, name: &str, f: impl FnOnce()) {
        self.log_start(name);
        if let Some(filter) = &self.filter {
            if !name.contains(filter) {
                self.ignore_this_test.set(true);
                return
            }
        }
        f();
    }
}

fn tab(s: &str) -> String {
    let mut result = String::new();
    for line in s.lines() {
        result.push_str("    ");
        result.push_str(line);
        result.push_str("\n");
    }
    return result;
}
