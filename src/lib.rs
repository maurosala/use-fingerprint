mod font_module;

use wasm_bindgen::prelude::*;
use sha2::{Digest, Sha256};
use web_sys::WebGlRenderingContext;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlCanvasElement;
use js_sys::Int32Array;
use wasm_bindgen::JsCast;
use font_module::font_list;

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
    let document = web_sys::window().unwrap().document().unwrap();
    let body = document.body().unwrap();
    
    // let fonts = vec!["sans-serif-thin","ARNO PRO","Agency FB","Arabic Typesetting","Arial Unicode MS","AvantGarde Bk BT","BankGothic Md BT","Batang","Bitstream Vera Sans Mono","Calibri","Century","Century Gothic","Clarendon","EUROSTILE","Franklin Gothic","Futura Bk BT","Futura Md BT","GOTHAM","Gill Sans","HELV","Haettenschweiler","Helvetica Neue","Humanst521 BT","Leelawadee","Letter Gothic","Levenim MT","Lucida Bright","Lucida Sans","Menlo","MS Mincho","MS Outlook","MS Reference Specialty","MS UI Gothic","MT Extra","MYRIAD PRO","Marlett","Meiryo UI","Microsoft Uighur","Minion Pro","Monotype Corsiva","PMingLiU","Pristina","SCRIPTINA","Segoe UI Light","Serifa","SimHei","Small Fonts","Staccato222 BT","TRAJAN PRO","Univers CE 55 Medium","Vrinda","ZWAdobeF"];

    let fonts = font_list();

    let mut fields = vec![];

    let span = document.create_element("span")?;
    let _dn = span.set_attribute("position", "fixed");
    let _dn = span.set_attribute("z-index", "-1");
    let span: web_sys::HtmlSpanElement = span
    .dyn_into::<web_sys::HtmlSpanElement>()
    .map_err(|_| ())
    .unwrap();
    body.append_child(&span).unwrap();
    span.set_inner_html("Hèll0òàù+!@#%&*()_+=-.,;:<>{}[]|\\/\"'`~^?¿¡mmmMMMmmmlllmmmLLL₹▁₺₸ẞॿmmmiiimmmIIImmmwwwmmmWWW");
    span.style().set_property("font-size", "128px").unwrap();
    for font in fonts {
        span.style().set_property("font-family", font).unwrap();
        let f = format!("{}_{}", span.get_bounding_client_rect().width(), span.get_bounding_client_rect().height());
        if fields.contains(&f) {
            continue;
        }
        fields.push(f);
    }
    body.remove_child(&span).unwrap();

    // Ok(format!("{}", fields.join("|")))
    Ok(format!("{:x}", Sha256::digest(fields.join("|"))))
}

fn im_online() -> Result<bool, JsValue> {
    let window = web_sys::window().expect("Missing window");
    let navigator = window.navigator();

    Ok(navigator.on_line())
}
