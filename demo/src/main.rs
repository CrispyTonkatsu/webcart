use std::fs;
pub use std::time::Instant;

use engine::engine::Engine;
use kiss3d::{
    camera::{self, OrbitCamera3d},
    color::Color,
    event::Key,
    glamx::Vec3,
    light::Light,
    scene::SceneNode3d,
    window::Window,
};

use crate::cartpolemodel::CartPoleModel;

mod cartpolemodel;

#[kiss3d::main]
async fn main() {
    let contents = fs::read_to_string("./test.json").expect("File not found.");
    let sim = serde_json::from_str::<Engine>(contents.as_str());

    if sim.is_err() {
        println!("This is so cool");
        return;
    }

    let mut sim = sim.unwrap();

    let mut window = Window::new("Control Lab").await;

    window.rebind_close_key(Some(Key::Escape));

    window.set_shadows_enabled(true);
    window.set_shadow_resolution(2048);

    let mut camera = OrbitCamera3d::new(
        Vec3 {
            x: 3.0,
            y: 3.0,
            z: 3.0,
        },
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
    );

    camera.set_projection(camera::Projection::Orthographic);

    let mut scene = SceneNode3d::empty();

    // Ground plane (shadow receiver).
    scene
        .add_cube(14.0, 0.4, 14.0)
        .set_position(Vec3::new(0.0, -1.0, 0.0))
        .set_color(Color::new(0.91, 1.0, 0.98, 1.0));

    // Directional light
    scene
        .add_light(
            Light::directional(Vec3::new(0.0, -0.5, -0.5))
                .with_color(Color::new(0.41, 0.51, 0.73, 1.0))
                .with_intensity(2.5),
        )
        .set_position(Vec3::new(0.0, 10.0, -5.0));

    // A non-shadow-casting fill light to lift the ambient look.
    scene
        .add_light(
            Light::point(40.0)
                .with_color(Color::new(0.41, 0.51, 0.73, 1.0))
                .with_intensity(1.5)
                .with_casts_shadows(false),
        )
        .set_position(Vec3::new(-5.0, 3.0, -5.0));

    let mut cart_model = CartPoleModel::default();
    cart_model.enter_scene(&mut scene, &sim.cart);

    let mut last_time = Instant::now();
    while window.render_3d(&mut scene, &mut camera).await {
        let now = Instant::now();
        let delta_time = now.duration_since(last_time).as_secs_f32();
        last_time = now;

        cart_model.update_input(&window, &mut sim.cart);
        sim.update(delta_time);
        cart_model.update_visuals(&sim.cart);
    }
}
