use nalgebra::Vector3;
use serde::{Deserialize, Serialize};

use crate::simulatable::Simulatable;

#[derive(Serialize, Deserialize)]
pub struct CartPole {
    pos: f32,
    vel: f32,

    theta: f32,
    omega: f32,

    force: f32,
    gravity: f32,

    length: f32,
    tip_mass: f32,
    cart_mass: f32,

    tip_radius: f32,
}

impl CartPole {
    pub fn apply_cart_impulse(&mut self, impulse: f32) {
        self.vel += impulse;
    }

    pub fn apply_tip_impulse(&mut self, impulse: f32) {
        self.omega += impulse;
    }

    fn controller_update(&mut self) {
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

    fn dynamics_update(&mut self, delta_time: f32) {
        if delta_time > 0.8 {
            return;
        }

        // Just for better readability
        let cos = f32::cos(self.theta);
        let sin = f32::sin(self.theta);

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

    pub fn pos(&self) -> f32 {
        self.pos
    }

    pub fn theta(&self) -> f32 {
        self.theta
    }

    pub fn pole_length(&self) -> f32 {
        self.length
    }

    pub fn tip_pos(&self) -> Vector3<f32> {
        Vector3::<f32>::new(
            self.pos - self.length * f32::sin(self.theta),
            self.length * f32::cos(self.theta),
            0.0,
        )
    }

    pub fn tip_radius(&self) -> f32 {
        self.tip_radius
    }
}

impl Simulatable for CartPole {
    fn update(&mut self, delta_time: f32) {
        self.controller_update();
        self.dynamics_update(delta_time);
    }
}
