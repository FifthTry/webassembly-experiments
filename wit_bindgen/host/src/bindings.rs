#[allow(clippy::all)]
pub mod say {
    #[allow(unused_imports)]
    use wit_bindgen_host_wasmtime_rust::{anyhow, wasmtime};

    /// Auxiliary data associated with the wasm exports.
    ///
    /// This is required to be stored within the data of a
    /// `Store<T>` itself so lifting/lowering state can be managed
    /// when translating between the host and wasm.
    // #[derive(Default)]
    pub struct SayData {
        pub wasi: wasmtime_wasi::WasiCtx,
    }
    pub struct Say<T> {
        get_state: Box<dyn Fn(&mut T) -> &mut SayData + Send + Sync>,
        cabi_realloc: wasmtime::TypedFunc<(i32, i32, i32, i32), i32>,
        canonical_abi_free: wasmtime::TypedFunc<(i32, i32, i32), ()>,
        hello: wasmtime::TypedFunc<(i32, i32), (i32,)>,
        memory: wasmtime::Memory,
    }
    impl<T> Say<T> {
        #[allow(unused_variables)]

        /// Adds any intrinsics, if necessary for this exported wasm
        /// functionality to the `linker` provided.
        ///
        /// The `get_state` closure is required to access the
        /// auxiliary data necessary for these wasm exports from
        /// the general store's state.
        pub fn add_to_linker(
            linker: &mut wasmtime::Linker<T>,
            get_state: impl Fn(&mut T) -> &mut SayData + Send + Sync + Copy + 'static,
        ) -> anyhow::Result<()> {
            Ok(())
        }

        /// Instantiates the provided `module` using the specified
        /// parameters, wrapping up the result in a structure that
        /// translates between wasm and the host.
        ///
        /// The `linker` provided will have intrinsics added to it
        /// automatically, so it's not necessary to call
        /// `add_to_linker` beforehand. This function will
        /// instantiate the `module` otherwise using `linker`, and
        /// both an instance of this structure and the underlying
        /// `wasmtime::Instance` will be returned.
        ///
        /// The `get_state` parameter is used to access the
        /// auxiliary state necessary for these wasm exports from
        /// the general store state `T`.
        pub fn instantiate(
            mut store: impl wasmtime::AsContextMut<Data = T>,
            module: &wasmtime::Module,
            linker: &mut wasmtime::Linker<T>,
            get_state: impl Fn(&mut T) -> &mut SayData + Send + Sync + Copy + 'static,
        ) -> anyhow::Result<(Self, wasmtime::Instance)> {
            Self::add_to_linker(linker, get_state)?;
            let instance = linker.instantiate(&mut store, module)?;
            Ok((Self::new(store, &instance, get_state)?, instance))
        }

        /// Low-level creation wrapper for wrapping up the exports
        /// of the `instance` provided in this structure of wasm
        /// exports.
        ///
        /// This function will extract exports from the `instance`
        /// defined within `store` and wrap them all up in the
        /// returned structure which can be used to interact with
        /// the wasm module.
        pub fn new(
            mut store: impl wasmtime::AsContextMut<Data = T>,
            instance: &wasmtime::Instance,
            get_state: impl Fn(&mut T) -> &mut SayData + Send + Sync + Copy + 'static,
        ) -> anyhow::Result<Self> {
            let mut store = store.as_context_mut();
            let cabi_realloc = instance
                .get_typed_func::<(i32, i32, i32, i32), i32, _>(&mut store, "cabi_realloc")?;
            let canonical_abi_free = instance
                .get_typed_func::<(i32, i32, i32), (), _>(&mut store, "canonical_abi_free")?;
            let hello = instance.get_typed_func::<(i32, i32), (i32,), _>(
                &mut store,
                "hello: func(name: string) -> string",
            )?;
            let memory = instance
                .get_memory(&mut store, "memory")
                .ok_or_else(|| anyhow::anyhow!("`memory` export not a memory"))?;
            Ok(Say {
                cabi_realloc,
                canonical_abi_free,
                hello,
                memory,
                get_state: Box::new(get_state),
            })
        }
        pub fn hello(
            &self,
            mut caller: impl wasmtime::AsContextMut<Data = T>,
            name: &str,
        ) -> Result<String, wasmtime::Trap> {
            let func_canonical_abi_free = &self.canonical_abi_free;
            let func_cabi_realloc = &self.cabi_realloc;
            let memory = &self.memory;
            let vec0 = name;
            let ptr0 = func_cabi_realloc.call(&mut caller, (0, 0, 1, vec0.len() as i32))?;
            memory
                .data_mut(&mut caller)
                .store_many(ptr0, vec0.as_bytes())?;
            let (result1_0,) = self.hello.call(&mut caller, (ptr0, vec0.len() as i32))?;
            let load2 = memory.data_mut(&mut caller).load::<i32>(result1_0 + 0)?;
            let load3 = memory.data_mut(&mut caller).load::<i32>(result1_0 + 4)?;
            let ptr4 = load2;
            let len4 = load3;

            let data4 = copy_slice(&mut caller, memory, ptr4, len4, 1)?;
            func_canonical_abi_free.call(&mut caller, (ptr4, len4, 1))?;
            Ok(String::from_utf8(data4).map_err(|_| wasmtime::Trap::new("invalid utf-8"))?)
        }
    }
    use wit_bindgen_host_wasmtime_rust::rt::copy_slice;
    use wit_bindgen_host_wasmtime_rust::rt::RawMem;
}
