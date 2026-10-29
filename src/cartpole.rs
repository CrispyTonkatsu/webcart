use kiss3d::{
    color::Color,
    event::{Action, Key},
    glamx::{Quat, Vec3},
    scene::SceneNode3d,
    window::Window,
};
use rand::random_range;

#[derive(Default)]
pub struct CartPole {
    pub pos: f64,
    pub vel: f64,

    pub theta: f64,
    pub omega: f64,

    pub force: f64,
    pub gravity: f64,

    pub length: f64,
    pub tip_mass: f64,
    pub cart_mass: f64,

    pub cart: SceneNode3d,
    pub pendulum: SceneNode3d,
}

impl CartPole {
    pub fn enter_scene(&mut self, scene: &mut SceneNode3d) {
        self.pendulum = scene.add_group();

        self.pendulum
            .add_sphere(0.05)
            .set_color(Color::new(0.96, 0.98, 0.19, 1.0));

        self.pendulum
            .add_cube(0.05, self.length as f32, 0.05)
            .set_position(Vec3::new(0.0, (self.length / 2.0) as f32, 0.0))
            .set_color(Color::new(0.84, 0.25, 0.27, 1.0));

        self.cart = scene
            .add_cube(0.5, 0.2, 0.4)
            .set_color(Color::new(0.14, 0.17, 0.2, 1.0));
    }

    pub fn input_update(&mut self, window: &Window) {
        if window.get_key(Key::Space) == Action::Press {
            self.omega += random_range(-1.5..1.5);
        }
    }

    pub fn controller_update(&mut self) {
        let position_gain = {
            let error = self.pos;
            let der_error = self.vel;

            let p = 0.3;
            let d = 0.5;

            p * error + d * der_error
        };

        let angle_gain = {
            let error = self.theta;
            let der_error = self.omega;

            let p = 30.0;
            let d = 5.0;

            -p * error - d * der_error
        };

        self.force = position_gain + angle_gain
    }

    pub fn dynamics_update(&mut self, delta_time: f64) {
        // Just for better readability
        let cos = f64::cos(self.theta);
        let sin = f64::sin(self.theta);

        // NOTE: Obtained by solving for \ddot{x}
        let accel = {
            (self.force - self.tip_mass * self.length * self.omega.powi(2) * sin
                + self.tip_mass * self.gravity * sin * cos)
                / (self.cart_mass + self.tip_mass * sin.powi(2))
        };

        // NOTE: Obtained by solving for \ddot{\theta}, needs to be done after \ddot{x}
        let alpha = { (accel * cos + self.gravity * sin) / self.length };

        self.vel += accel * delta_time;
        self.pos += self.vel * delta_time;

        self.omega += alpha * delta_time;
        self.theta += self.omega * delta_time;
    }

    pub fn update_visuals(&mut self) {
        self.pendulum
            .set_position(Vec3::new(self.pos as f32, 0.0, 0.0))
            .set_rotation(Quat::from_rotation_z(self.theta as f32));

        self.cart
            .set_position(Vec3::new(self.pos as f32, -0.1, 0.0));
    }
}
