use wasmer::{Instance, Module, Store};
use wasmer_wasi::WasiState;

fn main() {
    let store = Store::default();
    let f = "/Users/shobhitsharma/repos/playground/hello-wasmer/hello-as/build/release.wasm";
    // let f =
    //     "/Users/shobhitsharma/repos/playground/hello-wasmer/hello-wasm/target/wasm32-wasi/debug/hello-wasm.wasm";
    let module = Module::from_file(&store, f).unwrap();

    // Create the `WasiEnv`.
    let mut wasi_env = WasiState::new("command-name")
        .args(&["Gordon"])
        .finalize()
        .unwrap();

    // Generate an `ImportObject`.
    let import_object = wasi_env.import_object(&module).unwrap();

    // Let's instantiate the module with the imports.
    let instance = Instance::new(&module, &import_object).unwrap();

    // Let's call the `_start` function, which is our `main` function in Rust.
    let start = instance.exports.get_function("_start").unwrap();
    start.call(&[]).unwrap();
}
