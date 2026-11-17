use std::fs;
pub use std::time::Instant;

use kiss3d::{
    camera::{Camera3d, OrbitCamera3d, Projection},
    color::Color,
    event::{Action, Key},
    glamx::Vec3,
    light::Light,
    scene::SceneNode3d,
    window::Window,
};

use engine::{ball::Ball, engine::Engine};
use nalgebra::Vector3;
use rand::random_range;

use crate::{ballmodel::BallModel, cartpolemodel::CartPoleModel};

mod ballmodel;
mod cartpolemodel;

// TODO: Use Serde for the entire stuff including the ball to spawn

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

    camera.set_projection(Projection::Orthographic);

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

    let mut ball_model = BallModel::default();

    let mut last_time = Instant::now();
    while window.render_3d(&mut scene, &mut camera).await {
        let now = Instant::now();
        let delta_time = now.duration_since(last_time).as_secs_f32();
        last_time = now;

        if window.get_key(Key::W) == Action::Press {
            let cam_pos = camera.eye();
            let cam_pos = Vector3::<f32>::new(cam_pos.x, cam_pos.y, cam_pos.z);

            let ball_dir = (sim.cart.tip_pos() - cam_pos).normalize() * 20.0;

            let ball = Ball {
                pos: cam_pos,
                vel: Vector3::<f32>::new(ball_dir.x, ball_dir.y, ball_dir.z),
                net_force: Vector3::<f32>::new(0.0, 0.0, 0.0),
                radius: 0.2,
                mass: 1.0,
            };

            ball_model.enter_scene(&mut scene, &ball);
            sim.add_ball(ball);
        }

        if window.get_key(Key::Space) == Action::Press {
            sim.cart.apply_tip_impulse(random_range(-1.5..1.5));
        }

        sim.update(delta_time);
        cart_model.update_visuals(&sim.cart);
        ball_model.update_visuals(sim.ball.as_ref());
    }
}
