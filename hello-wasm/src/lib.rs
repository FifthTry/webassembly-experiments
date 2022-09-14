// use wasm_bindgen::prelude::*;

// #[wasm_bindgen(module = "the-wasm-import-module")]
// extern "C" {
//     fn foo();
//     fn translate_fancy(my_struct: String) -> String;
//     // ...
// }

// pub unsafe extern "C" fn run(a: i32, b: i32) -> i32 {
//     double(a) + double(b)
// }

// #[wasm_bindgen]
#[no_mangle]
pub unsafe extern "C" fn run(a: i32, b: i32) -> i32 {
    return a + b;
}
