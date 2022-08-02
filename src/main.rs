/**
 * My first rust program, a Javascript runtime using deno_core
 *  https://deno.com/blog/roll-your-own-javascript-runtime
 */
use deno_core::error::AnyError;
use std::rc::Rc;
use deno_core::op;
use deno_core::Extension;
use reqwest;

#[op]
async fn op_http_get_request(uri : String) -> Result<String, AnyError> {
    let response = reqwest::get(uri)
        .await?
        .text()
        //.json::<HashMap<String, String>>()
        .await?;
    Ok(response)
}

#[op]
async fn op_read_file(path : String) -> Result<String, AnyError> {
    let contents = tokio::fs::read_to_string(path).await?;
    Ok(contents)
}

#[op]
async fn op_write_file(path : String, contents : String) -> Result<(), AnyError> {
    tokio::fs::write(path, contents).await?;
    Ok(())
}

#[op]
fn op_remove_file(path : String) -> Result<(), AnyError> {
    std::fs::remove_file(path)?;
    Ok(())
}

async fn zjs(file_path: &str) -> Result<(), AnyError> {
    let main_module = deno_core::resolve_path(file_path)?;
    let zjs_extension = Extension::builder().ops(vec![
        op_read_file::decl(),
        op_write_file::decl(),
        op_remove_file::decl(),
    ]).build();

    // HTTP Extension
    let http_extension = Extension::builder().ops(vec![
        op_http_get_request::decl(),
    ]).build();

    let mut js_runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
        extensions : vec![zjs_extension, http_extension],
        ..Default::default()
    });
    js_runtime.execute_script("[zjs:runtime.js]", include_str!("./runtime.js")).unwrap();

    let mod_id = js_runtime.load_main_module(&main_module, None).await?;
    let result = js_runtime.mod_evaluate(mod_id);
    js_runtime.run_event_loop(false).await?;
    result.await?
}

fn main() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build().unwrap();
    if let Err(error) = runtime.block_on(zjs("./example.js")) {
        eprintln!("error: {}", error);
    }
}
