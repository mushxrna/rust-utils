use crate::{
    generics::NumericCollectionType,
    vectors::{Vec3, Vec4},
};

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
    pub fn new_default<T: NumericCollectionType<i32>>(position: T) -> Self {
        Self {
            position: Vec3::new(
                position.get(0).unwrap(),
                position.get(1).unwrap(),
                position.get(1).unwrap(),
            ),
            color: Vec4::new(1.0, 0.0, 0.0, 1.0),
            id: 0,
        }
    }
}
