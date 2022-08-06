pub use web_sys::{
    Window,
    Document,
    Element
};
pub use wasm_bindgen::prelude::JsValue;

pub type ElementResult<T> = std::result::Result<T, JsValue>;

pub fn document() -> Document {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("unable to get `document` node");
    document
}

pub fn window() -> Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn get_element_by_id(id : &str) -> Option<Element> {
    document().get_element_by_id(id)
}
