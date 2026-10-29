use kiss3d::prelude::*;

use crate::pendulum::InvPendulum;

pub mod pendulum;

#[kiss3d::main]
async fn main() {
    let mut window = Window::new("Control Lab").await;
    let mut camera = OrbitCamera3d::default();
    let mut scene = SceneNode3d::empty();

    let mut state = InvPendulum::default();
    state.enter_scene(&mut scene);

    while window.render_3d(&mut scene, &mut camera).await {
        state.render();
    }
}
