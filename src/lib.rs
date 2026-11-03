use wasm_bindgen::prelude::wasm_bindgen;

pub mod app;
pub mod cartpole;

#[kiss3d::main]
#[wasm_bindgen]
pub async fn main() {
    console_error_panic_hook::set_once();

    app::run().await;
}
