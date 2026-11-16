use std::cell::RefCell;

use crate::cartpole::CartPole;

pub struct Engine {
    cart: CartPole,
    ball: Option<CartPole>,
}
