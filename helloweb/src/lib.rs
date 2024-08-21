mod utils;

use hello::hello;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    let str = String::from(hello::greetings_from_rust());
    alert(&str);
}

#[wasm_bindgen]
pub fn greeting_string() -> String {
    let hello = String::from(hello::greetings_from_rust());
    return hello.into();
}
