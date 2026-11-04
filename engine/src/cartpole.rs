use wasm_bindgen::prelude::*;

#[derive(Default)]
#[wasm_bindgen]
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
}

#[wasm_bindgen]
impl CartPole {
    pub fn apply_cart_impulse(&mut self, impulse: f64) {
        self.vel += impulse;
    }

    pub fn apply_tip_impulse(&mut self, impulse: f64) {
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

    fn dynamics_update(&mut self, delta_time: f64) {
        if delta_time > 0.8 {
            return;
        }

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

    pub fn update(&mut self, delta_time: f64) {
        self.controller_update();
        self.dynamics_update(delta_time);
    }
}
