mod app;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run_app() {
    yew::Renderer::<app::App>::new().render();
}
