use crc::{crc32, Hasher32};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn fingerprint() -> Result<String, JsValue> {
    let window = web_sys::window().expect("should have a window in this context");
    let document = web_sys::window().unwrap().document().unwrap();
    let performance = window
        .performance()
        .expect("performance should be available");

    let _start = performance.now();

    let canvas = document.create_element("canvas")?;
    let _dn = canvas.set_attribute("display", "none");
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.begin_path();
    context.fill_style();
    context.fill_text("❤️🤪🎉👋", 50.0, 70.0).unwrap();
    context.stroke();

    let daturl = canvas.to_data_url().unwrap();
    let mut digest = crc32::Digest::new_with_initial(crc32::IEEE, 0u32);
    digest.write(&daturl.as_bytes());

    let _end = performance.now();

    let payload = format!(
        "{{\"ms\": {:.2}, \"id\": \"{:X}\" }}",
        _end - _start,
        digest.sum32(),
    );

    Ok(payload)
}