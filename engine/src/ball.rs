use nalgebra::Vector3;
use serde::{Deserialize, Serialize};

use crate::simulatable::Simulatable;

#[derive(Serialize, Deserialize)]
pub struct Ball {
    pos: Vector3<f32>,
    vel: Vector3<f32>,
    net_force: Vector3<f32>,
    mass: f32,
    radius: f32,
}

impl Ball {
    pub fn pos(&self) -> Vector3<f32> {
        self.pos
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }
}

impl Simulatable for Ball {
    fn update(&mut self, delta_time: f32) {
        let accel = self.net_force / self.mass;
        self.vel += accel * delta_time;
        self.pos += self.vel * delta_time;
    }
}
