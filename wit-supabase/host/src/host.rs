#[allow(clippy::all)]
pub mod host {
  #[allow(unused_imports)]
  use wit_bindgen_host_wasmtime_rust::{wasmtime, anyhow};
  #[derive(Clone)]
  pub struct Httpresponse {
    pub data: String,
  }
  impl core::fmt::Debug for Httpresponse {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
      f.debug_struct("Httpresponse").field("data", &self.data).finish()}
  }
  #[derive(Clone)]
  pub struct Httprequest<'a,> {
    pub path: &'a  str,
    pub method: &'a  str,
  }
  impl<'a,> core::fmt::Debug for Httprequest<'a,> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
      f.debug_struct("Httprequest").field("path", &self.path).field("method", &self.method).finish()}
  }
  pub trait Host: Sized {
    fn http(&mut self,request: Httprequest<'_,>,) -> Httpresponse;
    
  }
  
  pub fn add_to_linker<T, U>(linker: &mut wasmtime::Linker<T>, get: impl Fn(&mut T) -> &mut U+ Send + Sync + Copy + 'static) -> anyhow::Result<()> 
  where U: Host
  {
    use wit_bindgen_host_wasmtime_rust::rt::get_memory;
    use wit_bindgen_host_wasmtime_rust::rt::get_func;
    linker.func_wrap("host", "http: func(request: record { path: string, method: string }) -> record { data: string }", move |mut caller: wasmtime::Caller<'_, T>,arg0:i32,arg1:i32,arg2:i32,arg3:i32,arg4:i32| {
      
      let func = get_func(&mut caller, "cabi_realloc")?;
      let func_cabi_realloc = func.typed::<(i32, i32, i32, i32), i32, _>(&caller)?;
      let memory = &get_memory(&mut caller, "memory")?;
      let (mem, data) = memory.data_and_store_mut(&mut caller);
      let mut _bc = wit_bindgen_host_wasmtime_rust::BorrowChecker::new(mem);
      let host = get(data);
      let ptr0 = arg0;
      let len0 = arg1;
      let ptr1 = arg2;
      let len1 = arg3;
      let param0 = Httprequest{path:_bc.slice_str(ptr0, len0)?, method:_bc.slice_str(ptr1, len1)?, };
      let result = host.http(param0, );
      let Httpresponse{ data:data2, } = result;
      let vec3 = data2;
      let ptr3 = func_cabi_realloc.call(&mut caller, (0, 0, 1, vec3.len() as i32))?;
      let caller_memory = memory.data_mut(&mut caller);
      caller_memory.store_many(ptr3, vec3.as_bytes())?;
      caller_memory.store(arg4 + 4, wit_bindgen_host_wasmtime_rust::rt::as_i32(vec3.len() as i32))?;
      caller_memory.store(arg4 + 0, wit_bindgen_host_wasmtime_rust::rt::as_i32(ptr3))?;
      Ok(())
    })?;
    Ok(())
  }
  use wit_bindgen_host_wasmtime_rust::rt::RawMem;
}
