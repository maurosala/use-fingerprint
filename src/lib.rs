use wasm_bindgen::prelude::*;
use sha2::{Digest, Sha256};
use web_sys::WebGlRenderingContext;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlCanvasElement;
use js_sys::{Int32Array, Array, Object};
use wasm_bindgen::JsCast;


#[wasm_bindgen]
pub fn fingerprint() -> Result<String, JsValue> {
    let window = web_sys::window().expect("Missing window");
    let performance = window
        .performance()
        .expect("performance should be available");

    let _start = performance.now();

    let canvas = canvas().unwrap();
    let browser = browser().unwrap();
    let webgl = webgl().unwrap();
    let font = fonts().unwrap();

    let _online = im_online()?;
    let _incognito = im_incognito()?;

    let _end = performance.now();

    let id = format!("{:x}", Sha256::digest(format!("{}{}{}{}", canvas, browser, webgl, font).as_str()));

    let payload = format!(
        "{{\"ms\": {:.2}, \"canvas\": \"{}\", \"browser\": \"{}\", \"webgl\": \"{}\", \"font\": \"{}\", \"id\": \"{}\" }}",
        _end - _start,
        canvas,
        browser,
        webgl,
        font,
        id
    );

    Ok(payload)
}

fn canvas() -> Result<String, JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();

    let canvas = document.create_element("canvas")?;
    let _dn = canvas.set_attribute("display", "none");
    let canvas: HtmlCanvasElement = canvas
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    context.begin_path();
    context.fill_style();
    context.fill_text("❤️🤪🎉👋", 50.0, 70.0).unwrap();
    context.stroke();

    Ok(format!("{:x}", Sha256::digest(canvas.to_data_url().unwrap().as_str())))
}

fn browser() -> Result<String, JsValue> {
    let window = web_sys::window().expect("Missing window");
    
    let navigator = window.navigator();
    let screen = window.screen().unwrap();

    let user_agent = navigator.user_agent().unwrap();
    let language = navigator.language().unwrap();
    let languages = navigator.languages().join("_");
    let platform = navigator.platform().unwrap();
    let max_touch_points = navigator.max_touch_points();
    let hardware_concurrency = navigator.hardware_concurrency();
    let app_name = navigator.app_name();
    let product = navigator.product();
    let app_code_name = navigator.app_code_name().unwrap();
    let app_version = navigator.app_version().unwrap();
    
    let color_depth = screen.color_depth().unwrap();
    let height = screen.height().unwrap();
    let width = screen.width().unwrap();
    let pixel_depth = screen.pixel_depth().unwrap();
    
    let fields = format!("{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}|{}",
    user_agent, language, languages, max_touch_points, hardware_concurrency, app_name, platform, product, app_code_name, app_version, color_depth, height, width, pixel_depth); 

    // Ok(format!("{}", fields))
    Ok(format!("{:x}", Sha256::digest(fields)))
}

fn webgl() -> Result<String, JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();

    let canvas = document.create_element("canvas")?;
    let _dn = canvas.set_attribute("display", "none");
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("webgl")
        .unwrap()
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()
        .unwrap();

    let vendor = context.get_parameter(WebGlRenderingContext::VENDOR as u32).unwrap().as_string().unwrap();
    let renderer = context.get_parameter(WebGlRenderingContext::RENDERER as u32).unwrap().as_string().unwrap();
    let version = context.get_parameter(WebGlRenderingContext::VERSION as u32).unwrap().as_string().unwrap();
    let shading_language_version = context.get_parameter(WebGlRenderingContext::SHADING_LANGUAGE_VERSION).unwrap().as_string().unwrap();
    let max_vertex_attribs = context.get_parameter(WebGlRenderingContext::MAX_VERTEX_ATTRIBS).unwrap().as_f64().unwrap() as u32;
    let max_vertex_uniform_vectors = context.get_parameter(WebGlRenderingContext::MAX_VERTEX_UNIFORM_VECTORS).unwrap().as_f64().unwrap() as u32;
    let max_fragment_uniform_vectors = context.get_parameter(WebGlRenderingContext::MAX_FRAGMENT_UNIFORM_VECTORS).unwrap().as_f64().unwrap() as u32;
    let max_texture_image_units = context.get_parameter(WebGlRenderingContext::MAX_TEXTURE_IMAGE_UNITS).unwrap().as_f64().unwrap() as u32;
    let max_texture_size = context.get_parameter(WebGlRenderingContext::MAX_TEXTURE_SIZE).unwrap().as_f64().unwrap() as u32;
    let max_cube_map_texture_size = context.get_parameter(WebGlRenderingContext::MAX_CUBE_MAP_TEXTURE_SIZE).unwrap().as_f64().unwrap() as u32;
    let max_renderbuffer_size = context.get_parameter(WebGlRenderingContext::MAX_RENDERBUFFER_SIZE).unwrap().as_f64().unwrap() as u32;
    let max_combined_texture_image_units = context.get_parameter(WebGlRenderingContext::MAX_COMBINED_TEXTURE_IMAGE_UNITS).unwrap().as_f64().unwrap() as u32;
    let max_viewport_dims = Int32Array::from(context.get_parameter(WebGlRenderingContext::MAX_VIEWPORT_DIMS).unwrap()).to_string();
    let max_vertex_texture_image_units = context.get_parameter(WebGlRenderingContext::MAX_VERTEX_TEXTURE_IMAGE_UNITS).unwrap().as_f64().unwrap() as u32;
    let extensions = context.get_supported_extensions().unwrap().join("|");

    let fields = format!("{}|{}|{}|{}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{}|{:?}|{}",
    vendor, renderer, version, shading_language_version, max_vertex_attribs, max_vertex_uniform_vectors, max_fragment_uniform_vectors, max_texture_image_units, max_texture_size, max_cube_map_texture_size, max_renderbuffer_size, max_combined_texture_image_units, max_viewport_dims, max_vertex_texture_image_units, extensions);

    // Ok(format!("{}", fields))
    Ok(format!("{:x}", Sha256::digest(fields)))
}

fn fonts() -> Result<String, JsValue> {
    let font_faces_set = web_sys::window()
    .unwrap()
    .document()
    .unwrap()
    .font_faces()
    .unwrap();

    let font_faces_array = Array::from(font_faces_set);

    // Create a JS object to hold font names
    let mut fields = String::new();

    // Iterate over the FontFaceSet and extract font names
    for i in 0..font_faces_array.length() {
        let font_face = font_faces_array.get(i);
        let font_face_name = font_face.dyn_into::<web_sys::FontFace>().unwrap().family();
        fields.push_str(&format!("{}|", font_face_name));
        // font_names.set(&JsValue::from_str(&font_face_name), &JsValue::TRUE).unwrap();
    }

    // font_names.into();


    // let document = web_sys::window().unwrap().document().unwrap();
    // let fonts = document.fonts();

    // let mut fields = String::new();
    // for font in fonts.iter() {
    //     fields.push_str(&format!("{}|", font));
    // }

    // let fonts = document.fonts();

    // let mut fields = String::new();
    // for font in fonts.iter() {
    //     fields.push_str(&format!("{}|", font));
    // }

    Ok(format!("{}", fields))
    // Ok(format!("{:x}", Sha256::digest(fields)))
}

fn im_online() -> Result<bool, JsValue> {
    let window = web_sys::window().expect("Missing window");
    let navigator = window.navigator();

    Ok(navigator.on_line())
}

fn im_incognito() -> Result<bool, JsValue> {
    // TODO: Implement this
    // let window = web_sys::window().expect("Missing window");
    // let navigator = window.navigator();

    Ok(false)
}