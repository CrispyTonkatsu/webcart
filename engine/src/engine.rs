use serde::{Deserialize, Serialize};

use crate::{ball::Ball, cartpole::CartPole, simulatable::Simulatable};

#[derive(Serialize, Deserialize)]
pub struct Engine {
    pub cart: CartPole,
    pub ball: Option<Ball>,
}

impl Engine {
    pub fn update(&mut self, delta_time: f32) {
        // Updating the cart as regular
        self.cart.update(delta_time);

        if let Some(ball) = &mut self.ball {
            ball.update(delta_time);

            let to_tip = self.cart.tip_pos() - ball.pos();
            let radius_sum = ball.radius() + self.cart.tip_radius();

            if to_tip.magnitude() <= radius_sum {
                println!("Hit");
                self.ball = None
            }
        }
    }
}
