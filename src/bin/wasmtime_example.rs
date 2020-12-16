//put me in src/bin/ and then run:   cargo run --bin wasmtime_example


use anyhow::Result;
use wasmtime::*;


fn main() -> Result<()> {
let wasm_bytes = include_bytes!("../wasm/fib.wasm");
    let s = match String::from_utf8(wasm_bytes.to_vec()) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    let swasm_bytes = s.as_bytes();

    let store = Store::default();
    let module = Module::from_binary(store.engine(), swasm_bytes)?;
    let instance = Instance::new(&store, &module, &[])?;

    // Invoke `gcd` export
    let func = instance
        .get_func("func")
        .ok_or(anyhow::format_err!("failed to find function export"))?
        .get1::<i32, i32>()?;

    println!("Result {}", func(25)?);



    Ok(())
}