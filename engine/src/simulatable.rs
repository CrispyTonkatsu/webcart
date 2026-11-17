pub trait Simulatable {
    fn update(&mut self, delta_time: f32);
}
