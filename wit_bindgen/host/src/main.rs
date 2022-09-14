mod bindings;

use crate::bindings::say;
use bindings::say::SayData;
use wit_bindgen_host_wasmtime_rust::wasmtime::*;
// wit_bindgen_host_wasmtime_rust::import!(
//     "/Users/shobhitsharma/repos/playground/wit-bindgen-example/wits/say.wit"
// );
use wasmtime_wasi::sync::WasiCtxBuilder;

fn main() {
    let path = if cfg!(not(debug_assertions)) {
        "./target/wasm32-wasi/release/guest.wasm"
    } else {
        "./target/wasm32-wasi/debug/guest.wasm"
    };
    let path2 = if cfg!(not(debug_assertions)) {
        "./target/wasm32-wasi/release/guest2.wasm"
    } else {
        "./target/wasm32-wasi/debug/guest2.wasm"
    };

    let mut config = Config::new();
    config.cache_config_load_default().unwrap();
    config.wasm_backtrace_details(WasmBacktraceDetails::Disable);

    let engine = Engine::new(&config).unwrap();
    let module = Module::from_file(&engine, path2).unwrap();
    let mut linker: Linker<SayData> = Linker::new(&engine);
    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()
        .expect("WASI Error")
        .build();

    let mut store = Store::new(&engine, SayData { wasi });
    wasmtime_wasi::add_to_linker(&mut linker, |s| &mut s.wasi).expect("msg");

    let (import, _i) =
        say::Say::instantiate(&mut store, &module, &mut linker, |cx| cx).expect("Unable to run");
    let resp = import
        .hello(&mut store, "Shobhit")
        .expect("Fn did not execute correctly");
    println!("{}", resp);
}

fn default_wasi() -> wasmtime_wasi::WasiCtx {
    wasmtime_wasi::sync::WasiCtxBuilder::new()
        .inherit_stdio()
        .build()
}

struct Context<I, E> {
    wasi: wasmtime_wasi::WasiCtx,
    imports: I,
    exports: E,
}
