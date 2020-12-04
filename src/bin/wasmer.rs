
use wasmer::{imports, Instance, Module, NativeFunc, Store};
use wasmer_compiler_cranelift::Cranelift;
use wasmer_engine_jit::JIT;


// use wasmer::{Store, Module, Instance};
// use wasmer_runtime::{error, imports, instantiate, Func};
// use wasmer_wasi::{generate_import_object_from_env, get_wasi_version};
// use wasmer_wasi::WasiState;
 fn main() {
    let store = Store::new(&JIT::new(&Cranelift::default()).engine());

    // Now let's get the .wasm file as bytes
    let wasm_bytes = include_bytes!("../wasm/fib.wasm");
    let s = match String::from_utf8(wasm_bytes.to_vec()) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    let swasm_bytes = s.as_bytes();
    // With the Store and the wasm bytes we can create a wasm Module which is
    // a non-runnable representation of the contents of the wasm file.
    let module = Module::new(&store, &swasm_bytes[..]).expect("create module");

    // We create an empty ImportObject for the next step because we don't need to
    // import anything into `add.wasm`.
    let import_object = imports! {};

    // With our Module and our ImportObject we can create an Instance, which is the runnable
    // representation of the Wasm file.
    let instance = Instance::new(&module, &import_object).expect("instantiate module");

    // We can get functions from our Instance and execute them.
    // We get the add_one function as a NativeFunc that takes one u32 argument
    // and returns one u32 value.
    let add_one: NativeFunc<u32, u32> = instance
        .exports
        .get_native_function("fib")
        .expect("add_one function in Wasm module");
    let result = add_one.call(40).unwrap();

    // Log the result
    println!("Result: {}", result);


    // Let's declare the Wasm module with the text representation.
    // let wasm_bytes = std::fs::read(wasm_path)?;
//      println!("ddd");

//     let wasm_bytes = include_bytes!("../wasm/hi.wasm");
// let wasi_env = WasiState::new("command name")
//     // .args(&["world"])
//     // .env("KEY", "VALUE")
//     .finalize()?;
//        let store = Store::default();
//      println!("ddds");

//      let module = Module::from_binary(&store, wasm_bytes)?;
//         let wasi_version = get_wasi_version(&module, true).expect("Could not detect WASI ABI in Wasm module");
//     // let wasi_version = get_wasi_version(wasm_bytes, true).expect("Could not detect WASI ABI in Wasm module");
//     let import_object = generate_import_object_from_env(&store,wasi_env, wasi_version);
//     // let instance = instantiate(wasm_bytes, &import_object)?;
//     // let instance = module.instantiate(&import_object)?;
//     let instance = Instance::new(&module, &import_object)?;

// let start = instance.exports.get_function("main")?;
// start.call(&[])?;

    // let import_object = generate_import_object_from_state(state, wasi_version);
    // let instance = instantiate(wasm_bytes, &import_object)?;


    // print!("{}", String::from_utf8_lossy(wasm_bytes));

}