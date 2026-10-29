use kiss3d::{
    glamx::{Quat, Vec3},
    scene::SceneNode3d,
};

#[derive(Default)]
pub struct InvPendulum {
    x: f64,
    v: f64,
    theta: f64,
    omega: f64,
    group: SceneNode3d,
}

impl InvPendulum {
    pub fn enter_scene(&mut self, scene: &mut SceneNode3d) {
        self.group = scene.add_group();
        let mut sphere = self.group.add_sphere(0.01);
        sphere
            .add_cube(0.1, 1.0, 0.1)
            .set_position(Vec3::new(0.0, 0.1, 0.0));

        sphere.set_rotation(Quat::from_rotation_x(std::f32::consts::PI / 2.0));
    }

    pub fn update(&mut self, delta_time: f64) {}

    pub fn render(&mut self) {}
}
