use engine::ball::Ball;
use kiss3d::{glamx::Vec3, scene::SceneNode3d};

#[derive(Default)]
pub struct BallModel {
    pub ball: Option<SceneNode3d>,
}

impl BallModel {
    pub fn enter_scene(&mut self, scene: &mut SceneNode3d, ball: &Ball) {
        if let Some(node) = &mut self.ball {
            node.remove();
            self.ball = None;
        }

        let pos = ball.pos;

        self.ball = Some(scene.add_sphere(ball.radius).set_position(Vec3 {
            x: pos.x,
            y: pos.y,
            z: pos.z,
        }));
    }

    pub fn update_visuals(&mut self, ball: Option<&Ball>) {
        if self.ball.is_none() {
            return;
        }

        let node = self.ball.as_mut().unwrap();

        if ball.is_none() {
            node.remove();
            self.ball = None;

            return;
        }

        let pos = ball.unwrap().pos;

        node.set_position(Vec3 {
            x: pos.x,
            y: pos.y,
            z: pos.z,
        });
    }
}
