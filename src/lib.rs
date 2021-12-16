use wasm_bindgen::prelude::*;

mod days;

pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn greet(s: &str) {
    log(&format!("Hello {}!", s));
}

#[wasm_bindgen]
pub fn run(day: u8) -> Vec<usize> {
    let (part1, part2) = days::select_day(day, |s| log(s));
    return vec![part1, part2];
}