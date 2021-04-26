use wasm_bindgen::prelude::*;
use web_sys::console;
use wasm_bindgen::JsCast;


// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world!!"));

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let canvas = document.create_element("canvas")?;
    body.append_child(&canvas)?;

    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into()
        .unwrap();

    let width = 640.0;
    let height = 640.0;

    canvas.set_width(width as u32);
    canvas.set_height(height as u32);

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.set_fill_style(&JsValue::from_str("rgba(255, 0, 0, 0.1)"));

    for i in 0..30 {
        let x_size = i as f64 * (width / 30.0);
        let y_size = i as f64 * (height / 30.0);

        context.fill_rect(
            x_size / 2.0,
            y_size / 2.0,
            width - x_size,
            height - y_size,
        );
    }

    Ok(())
}
