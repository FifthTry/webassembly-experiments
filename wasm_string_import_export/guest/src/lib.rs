use std::ffi::CString;
use std::os::raw::c_char;

static HELLO: &'static str = "hello from rust";

#[no_mangle]
pub fn get_hello() -> *mut c_char {
    let s = CString::new(HELLO).unwrap();
    s.into_raw()
}

#[no_mangle]
pub fn get_hello_len() -> usize {
    HELLO.len()
}
