mod guest;
mod host;

use crate::guest as guest_imports;
use crate::host as host_exports;
use crate::host_exports::host::Host;

use wit_bindgen_host_wasmtime_rust::wasmtime::*;

#[derive(Default)]
pub struct HostExports {}

impl host_exports::host::Host for HostExports {
    fn http(
        &mut self,
        request: host_exports::host::Httprequest<'_>,
    ) -> host_exports::host::Httpresponse {
        panic!("Not implemented");
    }
}

struct Context<I, E> {
    wasi: wasmtime_wasi::WasiCtx,
    imports: I,
    exports: E,
}

fn main() {
    let path2 = "./target/wasm32-unknown-unknown/release/guest.wasm";

    let mut config = Config::new();
    config.cache_config_load_default().unwrap();
    config.wasm_backtrace_details(WasmBacktraceDetails::Disable);

    let engine = Engine::new(&config).unwrap();
    let module = Module::from_file(&engine, path2).unwrap();
    let mut linker: Linker<Context<HostExports, guest_imports::guest::GuestData>> =
        Linker::new(&engine);

    let mut store = Store::new(
        &engine,
        Context {
            wasi: default_wasi(),
            imports: HostExports {},
            exports: guest_imports::guest::GuestData {},
        },
    );
    host_exports::host::add_to_linker(&mut linker, |cx| &mut cx.imports);
    guest_imports::guest::Guest::add_to_linker(&mut linker, |cx| &mut cx.exports);

    let (import, _i) =
        guest_imports::guest::Guest::instantiate(&mut store, &module, &mut linker, |cx| {
            &mut cx.exports
        })
        .expect("Unable to run");
    let resp = import
        .run(&mut store, "Shobhit")
        .expect("Fn did not execute correctly");
    println!("{}", resp);
}

fn default_wasi() -> wasmtime_wasi::WasiCtx {
    wasmtime_wasi::sync::WasiCtxBuilder::new()
        .inherit_stdio()
        .build()
}
