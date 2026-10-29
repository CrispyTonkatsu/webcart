use kiss3d::prelude::*;

use crate::cartpole::CartPole;

pub mod cartpole;

#[kiss3d::main]
async fn main() {
    let mut window = Window::new("Control Lab").await;
    window.set_background_color(Color::new(0.05, 0.06, 0.09, 1.0));
    window.set_ambient(0.15);

    window.set_shadows_enabled(true);
    window.set_shadow_resolution(2048);

    let mut camera = OrbitCamera3d::default();
    let mut scene = SceneNode3d::empty();

    // Ground plane (shadow receiver).
    scene
        .add_cube(14.0, 0.4, 14.0)
        .set_position(Vec3::new(0.0, -1.0, 0.0))
        .set_color(Color::new(0.7, 0.7, 0.7, 1.0));

    // Directional light
    scene
        .add_light(
            Light::directional(Vec3::new(-0.6, -1.0, -0.3))
                .with_color(Color::new(1.0, 0.96, 0.85, 1.0))
                .with_intensity(2.5),
        )
        .set_position(Vec3::new(0.0, 6.0, 0.0));

    // Spot light pointing down at the scene, casting a sharper shadow. `reorient`
    // aims the node's forward (-Z) vector from the light position toward the floor.
    scene
        .add_light(
            Light::spot(0.35, 0.55, 30.0)
                .with_color(Color::new(0.8, 0.85, 1.0, 1.0))
                .with_intensity(12.0),
        )
        .reorient(Vec3::new(4.0, 6.0, 4.0), Vec3::ZERO, Vec3::Y);

    // A non-shadow-casting fill light to lift the ambient look.
    scene
        .add_light(
            Light::point(40.0)
                .with_color(Color::new(0.3, 0.3, 0.4, 1.0))
                .with_intensity(1.5)
                .with_casts_shadows(false),
        )
        .set_position(Vec3::new(-5.0, 3.0, -5.0));

    // Simulation setup

    // TODO: Probably make the simple thing set itself up in the constructor
    let mut cart = CartPole {
        pos: 0.0,       // Start in the middle of the track (meters)
        vel: 0.0,       // Starting stationary
        theta: 0.1,     // Slight initial tilt (radians, ~5.7 degrees) so it falls
        omega: 0.0,     // Starting with no angular velocity
        force: 0.0,     // No external push applied yet
        gravity: 9.81,  // Earth gravity ($m/s^2$)
        length: 1.0,    // Pole length of 1 meter
        tip_mass: 0.1,  // Light tip mass (0.1 kg)
        cart_mass: 1.0, // Heavier cart mass (1.0 kg)
        pendulum: SceneNode3d::empty(),
        cart: SceneNode3d::empty(),
    };

    cart.enter_scene(&mut scene);

    let mut last_time = std::time::Instant::now();
    while window.render_3d(&mut scene, &mut camera).await {
        let now = std::time::Instant::now();
        let delta_time = now.duration_since(last_time).as_secs_f64();
        last_time = now;

        cart.input_update(&window);
        cart.controller_update();
        cart.dynamics_update(delta_time);
        cart.update_visuals();
    }
}
