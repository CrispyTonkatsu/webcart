use kiss3d::window::CanvasSetup;

pub mod app;
pub mod cartpole;

#[kiss3d::main]
async fn main() {
    console_error_panic_hook::set_once();

    app::run(Some(CanvasSetup {
        canvas_id: "webcart-canvas".to_string(),
        ..CanvasSetup::default()
    }))
    .await;
}
