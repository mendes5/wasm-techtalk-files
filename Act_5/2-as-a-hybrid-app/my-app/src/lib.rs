use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    let canvas = document.create_element("canvas")?;
    body.append_child(&canvas)?;

    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
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
