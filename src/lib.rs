use js_sys::Array;
use wasm_bindgen::prelude::*;

mod days;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn run(day: u8) -> JsValue {
    let mut output = String::new();

    let (part1, part2) = days::select_day(day, |s| {
        output.push_str(&s);
        output.push_str("\n\n");
        log(s)
    });
    return JsValue::from(vec![JsValue::from(part1), JsValue::from(part2), JsValue::from(output)].into_iter().collect::<Array>());
}

#[wasm_bindgen]
pub fn init() {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    console_error_panic_hook::set_once();
}