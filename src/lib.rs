pub mod app;
pub mod cartpole;

#[kiss3d::main]
async fn main() {
    app::run().await;
}
