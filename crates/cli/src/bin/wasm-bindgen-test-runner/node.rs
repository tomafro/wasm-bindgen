use std::env;
use std::ffi::OsString;
use std::fs::File;
use std::io::{Write, Read};
use std::path::Path;
use std::process::Command;

use failure::{ResultExt, Error};
use parity_wasm::elements::{Module, Deserialize};

pub fn execute(module: &str, tmpdir: &Path, args: &[OsString]) -> Result<(), Error> {
    let mut js_to_execute = format!(r#"
        import {{ exit }} from 'process';

        let cx = null;

        // override `console.log` and `console.error` before we import tests to
        // ensure they're bound correctly in wasm. This'll allow us to intercept
        // all these calls and capture the output of tests
        const prev_log = console.log;
        console.log = function() {{
            if (cx === null)  {{
                prev_log.apply(null, arguments);
            }} else {{
                cx.console_log(prev_log, arguments);
            }}
        }};
        const prev_error = console.error;
        console.error = function() {{
            if (cx === null) {{
                prev_error.apply(null, arguments);
            }} else {{
                cx.console_error(prev_error, arguments);
            }}
        }};

        async function main(tests) {{
            const support = await import("./{0}");
            const wasm = await import("./{0}_bg");

            // Hack for now to support 0 tests in a binary. This should be done
            // better...
            if (support.Context === undefined)
                process.exit(0);

            cx = await support.Context.new();

            // Forward runtime arguments. These arguments are also arguments to the
            // `wasm-bindgen-test-runner` which forwards them to node which we
            // forward to the test harness. this is basically only used for test
            // filters for now.
            cx.args(process.argv.slice(2));

            if (!cx.run(tests.map(n => wasm[n])))
                exit(1);
        }}

        const tests = [];
    "#,
        module
    );

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
    if let Some(exports) = module.export_section() {
        for export in exports.entries() {
            if !export.field().starts_with("__wbg_test") {
                continue
            }
            js_to_execute.push_str(&format!("tests.push('{}')\n", export.field()));
        }
    }

    // And as a final addendum, exit with a nonzero code if any tests fail.
    js_to_execute.push_str("
        main(tests)
            .catch(e => {
                console.error(e);
                exit(1);
            });
    ");

    let js_path = tmpdir.join("run.mjs");
    File::create(&js_path)
        .and_then(|mut f| f.write_all(js_to_execute.as_bytes()))
        .context("failed to write JS file")?;

    let loader = tmpdir.join("loader.mjs");
    File::create(&loader)
        .and_then(|mut f| f.write_all(include_bytes!("loader.mjs")))
        .context("failed to write JS file")?;

    let mut opts = env::var("NODE_OPTIONS").unwrap_or_default();
    opts.push_str(" --experimental-modules");
    opts.push_str(&format!(" --loader {}", loader.display()));
    exec(
        Command::new("node")
            .env("NODE_OPTIONS", &opts)
            .arg(&js_path)
            .args(args)
    )
}

#[cfg(unix)]
fn exec(cmd: &mut Command) -> Result<(), Error> {
    use std::os::unix::prelude::*;
    Err(Error::from(cmd.exec()).context("failed to execute `node`").into())
}

#[cfg(windows)]
fn exec(cmd: &mut Command) -> Result<(), Error> {
    let status = cmd.status()?;
    process::exit(status.code().unwrap_or(3));
}
