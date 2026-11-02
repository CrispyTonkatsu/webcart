pub mod app;
pub mod cartpole;

#[wasm_bindgen]
#[kiss3d::main]
async fn main() {
    app::run().await;
}
