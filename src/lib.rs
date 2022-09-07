#![recursion_limit = "256"]

mod pages;
mod api;
mod types;
mod components;
mod route;
mod app;

// use pages::Home;
use wasm_bindgen::prelude::wasm_bindgen;
use yew::App;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<app::App>::new().mount_to_body();
}
