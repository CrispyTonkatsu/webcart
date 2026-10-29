use kiss3d::prelude::*;

pub mod pendulum;

#[kiss3d::main]
async fn main() {
    let mut window = Window::new("Control Lab").await;
    let mut camera = OrbitCamera3d::default();
    let mut scene = SceneNode3d::empty();

    let mut c = scene.add_cube(1.0, 1.0, 1.0);
    c.set_color(RED);

    let rot = Quat::from_axis_angle(Vec3::Y, 0.014);

    while window.render_3d(&mut scene, &mut camera).await {
        c.rotate(rot);
    }
}
