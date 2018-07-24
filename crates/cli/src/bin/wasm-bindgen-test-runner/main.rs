#[macro_use]
extern crate failure;
extern crate parity_wasm;
extern crate rouille;
extern crate wasm_bindgen_cli_support;

use std::env;
use std::fs::{self, File};
use std::io::{self, Write, Read};
use std::path::{PathBuf, Path};
use std::process;

use failure::{ResultExt, Error};
use parity_wasm::elements::{Module, Deserialize};
use wasm_bindgen_cli_support::Bindgen;

mod node;
mod server;

fn main() {
    let err = match rmain() {
        Ok(()) => return,
        Err(e) => e,
    };
    eprintln!("error: {}", err);
    for cause in err.causes().skip(1) {
        eprintln!("\tcaused by: {}", cause);
    }
    process::exit(1);
}

fn rmain() -> Result<(), Error> {
    let mut args = env::args_os().skip(1);

    // Currently no flags are supported, and assume there's only one argument
    // which is the wasm file to test. This'll want to improve over time!
    let wasm_file_to_test = match args.next() {
        Some(file) => PathBuf::from(file),
        None => bail!("must have a file to test as first argument"),
    };

    // Assume a cargo-like directory layout and generate output at
    // `target/wasm32-unknown-unknown/wbg-tmp/...`
    let tmpdir = wasm_file_to_test.parent() // chop off file name
        .and_then(|p| p.parent())           // chop off `deps`
        .and_then(|p| p.parent())           // chop off `debug`
        .map(|p| p.join("wbg-tmp"))
        .ok_or_else(|| {
            format_err!("file to test doesn't follow the expected Cargo conventions")
        })?;

    // Make sure there's no stale state from before
    drop(fs::remove_dir_all(&tmpdir));
    fs::create_dir(&tmpdir)
        .context("creating temporary directory")?;

    let node = true;

    print!("Executing bindgen ...\r");
    io::stdout().flush()?;

    // For now unconditionally generate wasm-bindgen code tailored for node.js,
    // but eventually we'll want more options here for browsers!
    let mut b = Bindgen::new();
    b.debug(true)
        .nodejs(node)
        .nodejs_experimental_modules(node)
        .input_path(&wasm_file_to_test)
        .keep_debug(false)
        .generate(&tmpdir)
        .context("executing `wasm-bindgen` over the wasm file")?;

    print!("                     \r");
    io::stdout().flush()?;

    let module = wasm_file_to_test.file_stem()
        .and_then(|s| s.to_str())
        .ok_or_else(|| format_err!("invalid filename passed in"))?;

    if node {
        return node::execute(&module, &tmpdir, &args.collect::<Vec<_>>())
    }

    server::spawn(&module, &tmpdir, &args.collect::<Vec<_>>())
}

fn find_tests(tmpdir: &Path, module: &str) -> Result<Vec<String>, Error> {
    // Collect all tests that the test harness is supposed to run. We assume
    // that any exported function with the prefix `__wbg_test` is a test we need
    // to execute.
    //
    // Note that we're collecting *JS objects* that represent the functions to
    // execute, and then those objects are passed into wasm for it to execute
    // when it sees fit.
    let mut wasm = Vec::new();
    let wasm_file = tmpdir.join(format!("{}_bg.wasm", module));
    File::open(wasm_file).and_then(|mut f| f.read_to_end(&mut wasm))
        .context("failed to read wasm file")?;
    let module = Module::deserialize(&mut &wasm[..])
        .context("failed to deserialize wasm module")?;
    let mut ret = Vec::new();
    if let Some(exports) = module.export_section() {
        for export in exports.entries() {
            if !export.field().starts_with("__wbg_test") {
                continue
            }
            ret.push(export.field().to_string());
        }
    }
    Ok(ret)
}
