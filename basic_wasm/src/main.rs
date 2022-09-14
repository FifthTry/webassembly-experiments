// use wasmer::{Instance, Module, Store};
// use wasmer_wasi::WasiState;

// fn main() {
//     let store = Store::default();
//     let f = "/Users/shobhitsharma/repos/playground/hello-wasmer/hello-as/build/release.wasm";
//     // let f =
//     //     "/Users/shobhitsharma/repos/playground/hello-wasmer/hello-wasm/target/wasm32-wasi/debug/hello-wasm.wasm";
//     let module = Module::from_file(&store, f).unwrap();

//     // Create the `WasiEnv`.
//     let mut wasi_env = WasiState::new("command-name")
//         .args(&["Gordon"])
//         .finalize()
//         .unwrap();

//     // Generate an `ImportObject`.
//     let import_object = wasi_env.import_object(&module).unwrap();

//     // Let's instantiate the module with the imports.
//     let instance = Instance::new(&module, &import_object).unwrap();

//     // Let's call the `_start` function, which is our `main` function in Rust.
//     let start = instance.exports.get_function("_start").unwrap();
//     start.call(&[]).unwrap();
// }

use wasmtime::*;
// use wasmtime_wasi::sync::WasiCtxBuilder;
// FPM
fn main() {
    // Compile our module and create a `Linker` which has WASI functions defined
    // within it.
    // let f = "/Users/shobhitsharma/repos/playground/hello-wasmer/hello-as/build/release.wasm";
    let f = "/Users/shobhitsharma/repos/playground/hello-wasmer/hello-wasm/target/wasm32-wasi/release/hello_wasm.wasm";

    let engine = Engine::default();
    let module = Module::from_file(&engine, f).unwrap();
    let mut linker = Linker::new(&engine);
    linker.func_wrap("env", "double", |x: i32| x * 2).unwrap();

    let mut store = Store::new(&engine, {});

    // Instantiate our module with the imports we've created, and run it.
    let instance = linker.instantiate(&mut store, &module).unwrap();
    // let exports = instance.exports(&mut store);
    // exports.into_iter().map(|f| {
    //     dbg!(f.name());
    // });
    // let mem = match instance.get_export(&mut store, "memory") {
    //     Some(Extern::Memory(mem)) => mem,
    //     _ => panic!("failed to find host memory"),
    // };
    // mem.data(&mut store).
    // dbg!(mem.read(&mut store, 0, &mut [0]));
    // mem.write(&mut store, 1, &mut ["Shobhit Sharma"]);
    // dbg!(mem.data(&mut store).get(1));
    // dbg!(mem);
    // let typed_func = match instance.get_export(&mut store, "run") {
    //     Some(ins) => match ins.into_func() {
    //         Some(f) => f.typed::<String, String, _>(&store).unwrap(),
    //         None => panic!("Not a function"),
    //     },
    //     None => panic!("Not found "),
    // };
    let typed_func = match instance.get_export(&mut store, "run") {
        Some(ins) => match ins.into_func() {
            Some(f) => f.typed::<(i32, i32), i32, _>(&store).unwrap(),
            None => panic!("Not a function"),
        },
        None => panic!("Not found "),
    };

    // let hello = instance
    //     .get_typed_func::<&str, String, _>(&mut store, "run")
    //     .unwrap();

    match typed_func.call(&mut store, (1, 2)) {
        Ok(res) => {
            println!("{}", res)
        }
        Err(trap) => {
            panic!("execution of `foo` resulted in a wasm trap: {}", trap);
        }
    }
}
