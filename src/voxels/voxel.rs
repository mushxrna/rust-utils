use crate::vectors::{Vec3, Vec4};

#[derive(Clone, Copy, PartialEq)]
pub struct Voxel {
    pub position: Vec3<i32>,
    pub color: Vec4<f32>,
    pub id: u32,
}

impl Voxel {
    pub fn new(position: Vec3<i32>, color: Vec4<f32>, id: u32) -> Self {
        Self {
            position,
            color,
            id,
        }
    }
}
