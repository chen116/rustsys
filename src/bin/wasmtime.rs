
use anyhow::Result;
use wasmtime::*;
// use wasmtime_wasi::{Wasi, WasiCtx};

fn main() -> Result<()> {
let wasm_bytes = include_bytes!("../wasm/add.wasm");

    let store = Store::default();
    let module = Module::from_binary(store.engine(), wasm_bytes)?;
    let instance = Instance::new(&store, &module, &[])?;

    // Invoke `gcd` export
    let gcd = instance
        .get_func("add")
        .ok_or(anyhow::format_err!("failed to find `gcd` function export"))?
        .get2::<i32, i32, i32>()?;

    println!("gcd(6, 27) = {}", gcd(6, 27)?);

// let wasm_bytes = include_bytes!("../wasm/hi.wasm");

//     tracing_subscriber::FmtSubscriber::builder()
//         .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
//         .with_ansi(true)
//         .init();

//     let store = Store::default();
//     let mut linker = Linker::new(&store);

//     // Create an instance of `Wasi` which contains a `WasiCtx`. Note that
//     // `WasiCtx` provides a number of ways to configure what the target program
//     // will have access to.
//     let wasi = Wasi::new(&store, WasiCtx::new(std::env::args())?);
//     wasi.add_to_linker(&mut linker)?;
//     // Instantiate our module with the imports we've created, and run it.
//     let module = Module::from_binary(store.engine(), wasm_bytes)?;
//     linker.module("", &module)?;
//     linker.get_default("")?.get0::<()>()?()?;

    Ok(())
}