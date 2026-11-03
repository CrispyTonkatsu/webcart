use kiss3d::window::CanvasSetup;

pub mod app;
pub mod cartpole;

#[kiss3d::main]
async fn main() {
    app::run(Some(CanvasSetup {
        canvas_id: "webcart-canvas".to_string(),
        ..CanvasSetup::default()
    }))
    .await;
}
