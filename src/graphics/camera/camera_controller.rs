use crate::graphics::vectors::Vec3;

pub enum LocalDirection {
    Forward,
    Backward,
    Left,
    Right,
    Up,
    Down,
}

pub struct CameraController {
    pub position: Vec3<f32>,
    pub view_direction: Vec3<f32>,
}

impl CameraController {
    pub fn new(position: Vec3<f32>, view_direction: Vec3<f32>) -> Self {
        Self {
            position,
            view_direction,
        }
    }

    pub fn shift(&mut self, offset: Vec3<f32>) {
        self.position = self.position + offset;
    }

    pub fn walk(&mut self, direction: LocalDirection, step_size: f32) {
        let forward = self.view_direction * (step_size, step_size, step_size);
        let up: Vec3<f32> = Vec3::new(0.0, 1.0, 0.0);
        let right = forward.cross(up).normalize() * (step_size, step_size, step_size);
        let negative: Vec3<f32> = Vec3::new(-1.0, -1.0, -1.0);

        match direction {
            LocalDirection::Forward => self.shift(forward),
            LocalDirection::Backward => self.shift(forward * negative),
            LocalDirection::Right => self.shift(right),
            LocalDirection::Left => self.shift(right * negative),
            LocalDirection::Up => self.shift(up),
            LocalDirection::Down => self.shift(up * negative),
        }
    }

    pub fn look(&mut self, view_direction: Vec3<f32>) {
        self.view_direction = view_direction;
    }
}
