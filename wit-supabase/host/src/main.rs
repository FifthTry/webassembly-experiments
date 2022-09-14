mod guest;
mod host;

use crate::guest as guest_imports;
use crate::host as host_exports;

use wit_bindgen_host_wasmtime_rust::wasmtime::*;

#[derive(Default)]
pub struct HostExports {
    gd: guest_imports::guest::GuestData,
}

impl host_exports::host::Host for HostExports {
    fn http(
        &mut self,
        request: host_exports::host::Httprequest<'_>,
    ) -> host_exports::host::Httpresponse {
        panic!("Not implemented");
    }
}

fn main() {
    let path2 = "./target/wasm32-unknown-unknown/release/guest.wasm";

    let mut config = Config::new();
    config.cache_config_load_default().unwrap();
    config.wasm_backtrace_details(WasmBacktraceDetails::Disable);

    let engine = Engine::new(&config).unwrap();
    let module = Module::from_file(&engine, path2).unwrap();
    let mut linker: Linker<HostExports> = Linker::new(&engine);

    let mut guest_linker: Linker<guest_imports::guest::GuestData> = Linker::new(&engine);

    let mut store = Store::new(&engine, guest_imports::guest::GuestData {});
    host_exports::host::add_to_linker(&mut linker, |cx| cx);
    guest_imports::guest::Guest::add_to_linker(&mut guest_linker, |cx| cx);

    let (import, _i) =
        guest_imports::guest::Guest::instantiate(&mut store, &module, &mut guest_linker, |cx| cx)
            .expect("Unable to run");
    let resp = import
        .run(&mut store, "Shobhit")
        .expect("Fn did not execute correctly");
    println!("{}", resp);
}
