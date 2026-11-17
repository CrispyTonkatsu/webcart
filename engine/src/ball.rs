use nalgebra::Vector3;
use serde::{Deserialize, Serialize};

use crate::simulatable::Simulatable;

#[derive(Serialize, Deserialize, Default)]
pub struct Ball {
    pub pos: Vector3<f32>,
    pub vel: Vector3<f32>,
    pub net_force: Vector3<f32>,
    pub mass: f32,
    pub radius: f32,
}

impl Simulatable for Ball {
    fn update(&mut self, delta_time: f32) {
        let accel = self.net_force / self.mass;
        self.vel += accel * delta_time;
        self.pos += self.vel * delta_time;
    }
}
