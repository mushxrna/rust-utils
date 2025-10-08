use crate::{NumericCollectionType, NumericType, VectorType};

#[derive(Clone, Copy, PartialEq)]
pub struct Vec4<T: NumericType> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T: NumericType> VectorType<T> for Vec4<T> {
    fn len(&self) -> usize {
        4
    }
    fn get(&self, index: usize) -> Option<T> {
        match index {
            0 => Some(self.x),
            1 => Some(self.y),
            2 => Some(self.z),
            3 => Some(self.w),
            _ => None,
        }
    }
}

impl<T: NumericType> NumericCollectionType<T> for Vec4<T> {
    fn len(&self) -> Option<usize> {
        Some(4)
    }
    fn get(&self, index: usize) -> Option<T> {
        match index {
            0 => Some(self.x),
            1 => Some(self.y),
            2 => Some(self.z),
            3 => Some(self.w),
            _ => None,
        }
    }
}

impl<T: NumericType> Vec4<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }

    pub fn from_collection<Z: NumericCollectionType<T>>(collection: Z) -> Result<Self, String> {
        if collection.len() != Some(4) {
            Err(String::from(
                "invalid collection length. cannot convert to Vec4",
            ))
        } else {
            Ok(Self {
                x: collection.get(0).unwrap(),
                y: collection.get(1).unwrap(),
                z: collection.get(2).unwrap(),
                w: collection.get(3).unwrap(),
            })
        }
    }
}
