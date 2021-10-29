#![recursion_limit = "256"]
mod app;
mod components;
//use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

//#[wasm_bindgen]
pub fn run_app() {
    yew::start_app::<app::App>();
}