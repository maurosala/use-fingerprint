use wasm_bindgen::prelude::*;
use sha2::{Digest, Sha256};
use web_sys;

#[wasm_bindgen]
pub fn fingerprint() -> Result<String, JsValue> {
    let window = web_sys::window().expect("should have a window in this context");
    let performance = window
        .performance()
        .expect("performance should be available");

    let _start = performance.now();

    let canvas = canvas()?;
    let browser = browser()?;


    let _end = performance.now();

    let payload = format!(
        "{{\"ms\": {:.2}, \"fingerprint\": \"{}\", \"browser\": \"{}\" }}",
        _end - _start,
        canvas,
        browser
    );

    Ok(payload)
}

fn canvas() -> Result<String, JsValue> {
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

    Ok(format!("{:x}", Sha256::digest(canvas.to_data_url().unwrap().as_str())))
}

fn browser() -> Result<String, JsValue> {
    let window = web_sys::window().expect("should have a window in this context");
    
    let navigator = window.navigator();
    let screen = window.screen().unwrap();

    let user_agent = navigator.user_agent().unwrap();
    let language = navigator.language().unwrap();
    let platform = navigator.platform().unwrap();
    let max_touch_points = navigator.max_touch_points();
    let hardware_concurrency = navigator.hardware_concurrency();
    let app_name = navigator.app_name();
    let product = navigator.product();
    let app_code_name = navigator.app_code_name().unwrap();
    let app_version = navigator.app_version().unwrap();
    let on_line = navigator.on_line();
    // let incognito = navigator.cookie_enabled();
    
    let color_depth = screen.color_depth().unwrap();
    let height = screen.height().unwrap();
    let width = screen.width().unwrap();
    let pixel_depth = screen.pixel_depth().unwrap();
    
    let fields = format!("{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}", 
    user_agent, language, max_touch_points, hardware_concurrency, app_name, platform, product, app_code_name, app_version, on_line, color_depth, height, width, pixel_depth); 


    // Ok(format!("{}", incognito))
    Ok(format!("{:x}", Sha256::digest(fields)))
}