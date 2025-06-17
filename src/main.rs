use wasm_bindgen::prelude::*;
use yew::prelude::*;
use log::Level;

mod app;
mod audio;
mod components;
mod patterns;

#[wasm_bindgen(start)]
pub fn run_app() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    console_log::init_with_level(Level::Debug).expect("Failed to initialize logger");
    
    App::<app::App>::new().mount_to_body();
    Ok(())
} 