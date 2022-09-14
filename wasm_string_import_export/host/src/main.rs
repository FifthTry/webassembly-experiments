use wasmtime::*;

fn main() {
    let f = "target/wasm32-unknown-unknown/release/guest.wasm";
    let engine = Engine::default();
    let module = Module::from_file(&engine, f).unwrap();
    let mut linker = Linker::new(&engine);

    let mut store = Store::new(&engine, {});

    let instance = linker.instantiate(&mut store, &module).unwrap();
    let mem = match instance.get_export(&mut store, "memory") {
        Some(Extern::Memory(mem)) => mem,
        _ => panic!("failed to find host memory"),
    };
    let typed_func = match instance.get_export(&mut store, "get_hello") {
        Some(ins) => match ins.into_func() {
            Some(f) => f.typed::<(), i32, _>(&store).unwrap(),
            None => panic!("Not a function"),
        },
        None => panic!("Not found "),
    };
    Uint8Array()
}
