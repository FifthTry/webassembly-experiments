#[allow(clippy::all)]
mod host {
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
  pub fn http(request: Httprequest<'_,>,) -> Httpresponse{
    unsafe {
      let Httprequest{ path:path0, method:method0, } = request;
      let vec1 = path0;
      let ptr1 = vec1.as_ptr() as i32;
      let len1 = vec1.len() as i32;
      let vec2 = method0;
      let ptr2 = vec2.as_ptr() as i32;
      let len2 = vec2.len() as i32;
      let ptr3 = __HOST_RET_AREA.0.as_mut_ptr() as i32;
      #[link(wasm_import_module = "host")]
      extern "C" {
        #[cfg_attr(target_arch = "wasm32", link_name = "http: func(request: record { path: string, method: string }) -> record { data: string }")]
        #[cfg_attr(not(target_arch = "wasm32"), link_name = "host_http: func(request: record { path: string, method: string }) -> record { data: string }")]
        fn wit_import(_: i32, _: i32, _: i32, _: i32, _: i32, );
      }
      wit_import(ptr1, len1, ptr2, len2, ptr3);
      let len4 = *((ptr3 + 4) as *const i32) as usize;
      Httpresponse{data:String::from_utf8(Vec::from_raw_parts(*((ptr3 + 0) as *const i32) as *mut _, len4, len4)).unwrap(), }
    }
  }
  
  #[repr(align(4))]
  struct __HostRetArea([u8; 8]);
  static mut __HOST_RET_AREA: __HostRetArea = __HostRetArea([0; 8]);
}
