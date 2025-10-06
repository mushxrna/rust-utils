use crate::{NumericCollectionType, NumericType, VectorType};

#[derive(Clone, Copy)]
pub struct Vec3<T: NumericType> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: NumericType> VectorType<T> for Vec3<T> {
    fn len(&self) -> usize {
        3
    }
    fn get(&self, index: usize) -> Option<T> {
        match index {
            0 => Some(self.x),
            1 => Some(self.y),
            2 => Some(self.z),
            _ => None,
        }
    }
}

impl<T: NumericType> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    pub fn from_collection<Z: NumericCollectionType<T>>(collection: Z) -> Result<Self, String> {
        if collection.len() != Some(3) {
            Err(String::from(
                "invalid collection length. cannot convert to Vec3",
            ))
        } else {
            Ok(Self {
                x: collection.get(0).unwrap(),
                y: collection.get(1).unwrap(),
                z: collection.get(2).unwrap(),
            })
        }
    }
}
