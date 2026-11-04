use kiss3d::{
    color::Color,
    event::{Action, Key},
    glamx::{Quat, Vec3},
    scene::SceneNode3d,
    window::Window,
};

use rand::random_range;

use engine::cartpole::CartPole;

#[derive(Default)]
pub struct CartPoleModel {
    pub simulation: CartPole,

    pub cart: SceneNode3d,
    pub pendulum: SceneNode3d,
}

impl CartPoleModel {
    pub fn enter_scene(&mut self, scene: &mut SceneNode3d) {
        let cart = &self.simulation;

        self.pendulum = scene.add_group();

        self.pendulum
            .add_sphere(0.05)
            .set_color(Color::new(0.96, 0.98, 0.19, 1.0));

        self.pendulum
            .add_cube(0.05, cart.length as f32, 0.05)
            .set_position(Vec3::new(0.0, (cart.length / 2.0) as f32, 0.0))
            .set_color(Color::new(0.84, 0.25, 0.27, 1.0));

        self.cart = scene
            .add_cube(0.5, 0.2, 0.4)
            .set_color(Color::new(0.14, 0.17, 0.2, 1.0));
    }

    pub fn input_update(&mut self, window: &Window) {
        if window.get_key(Key::Space) == Action::Press {
            self.simulation.apply_tip_impulse(random_range(-1.5..1.5));
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.simulation.update(delta_time);
    }

    pub fn update_visuals(&mut self) {
        let cart = &self.simulation;

        self.pendulum
            .set_position(Vec3::new(cart.pos as f32, 0.0, 0.0))
            .set_rotation(Quat::from_rotation_z(cart.theta as f32));

        self.cart
            .set_position(Vec3::new(cart.pos as f32, -0.1, 0.0));
    }
}
