use std::ffi::OsString;
use std::io::{Write, Read};
use std::path::Path;
use std::fs::File;

use failure::{ResultExt, Error};
use rouille::{self, Response, Request};
use wasm_bindgen_cli_support::wasm2es6js::Config;

pub fn spawn(module: &str, tmpdir: &Path, args: &[OsString]) -> Result<(), Error> {
    let mut js_to_execute = format!(r#"
        import {{ Context }} from './{0}';
        import * as wasm from './{0}_bg';

        document.getElementById('output').innerHTML = "Loading wasm module...";

        async function main(test) {{
            await wasm.booted;
            const cx = await Context.new();
            window.global_cx = cx;

            // Forward runtime arguments. These arguments are also arguments to the
            // `wasm-bindgen-test-runner` which forwards them to node which we
            // forward to the test harness. this is basically only used for test
            // filters for now.
            cx.args({1:?});

            cx.run(test.map(s => wasm[s]));
        }}

        const tests = [];
    "#,
        module, args,
    );

    let tests = super::find_tests(tmpdir, module)?;
    if tests.len() == 0 {
        println!("no tests to run!");
        return Ok(())
    }
    for test in tests {
        js_to_execute.push_str(&format!("tests.push('{}')\n", test));
    }
    // And as a final addendum, exit with a nonzero code if any tests fail.
    js_to_execute.push_str("
        main(tests)
    ");

    let js_path = tmpdir.join("run.js");
    File::create(&js_path)
        .and_then(|mut f| f.write_all(js_to_execute.as_bytes()))
        .context("failed to write JS file")?;

    let mut wasm = Vec::new();
    let wasm_name = format!("{}_bg.wasm", module);
    File::open(tmpdir.join(&wasm_name))
        .and_then(|mut f| f.read_to_end(&mut wasm))?;
    let output = Config::new()
        .fetch(Some(format!("/{}", wasm_name)))
        .generate(&wasm)?;
    let js = output.js()?;

    File::create(tmpdir.join(format!("{}_bg.js", module)))
        .and_then(|mut f| f.write_all(js.as_bytes()))
        .context("failed to write JS file")?;

    let tmpdir = tmpdir.to_path_buf();
    println!("Listening on port 8000");
    rouille::start_server("localhost:8000", move |request| {
        let url = request.url();
        if url == "/" {
            return Response::from_data("text/html", include_str!("index.html"));
        }
        let mut response = try_asset(&request, &tmpdir);
        if !response.is_success() {
            response = try_asset(&request, ".".as_ref());
        }
        response.headers.retain(|(k, _)| k != "Cache-Control");
        return response
    });

    fn try_asset(request: &Request, dir: &Path) -> Response {
        let response = rouille::match_assets(request, dir);
        if response.is_success() {
            return response
        }

        if let Some(part) = request.url().split('/').last() {
            if !part.contains(".") {
                let new_request = Request::fake_http(
                    request.method(),
                    format!("{}.js", request.url()),
                    request.headers()
                        .map(|(a, b)| (a.to_string(), b.to_string()))
                        .collect(),
                    Vec::new(),
                );
                let response = rouille::match_assets(&new_request, dir);
                if response.is_success() {
                    return response
                }
            }
        }
        response
    }
}
