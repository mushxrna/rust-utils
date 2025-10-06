use crate::{NumericCollectionType, NumericType, VectorType};

#[derive(Clone, Copy)]
pub struct Vec2<T: NumericType> {
    pub x: T,
    pub y: T,
}

impl<T: NumericType> VectorType<T> for Vec2<T> {
    fn len(&self) -> usize {
        2
    }
    fn get(&self, index: usize) -> Option<T> {
        match index {
            0 => Some(self.x),
            1 => Some(self.y),
            _ => None,
        }
    }
}

impl<T: NumericType> NumericCollectionType<T> for Vec2<T> {
    fn len(&self) -> Option<usize> {
        Some(2)
    }
    fn get(&self, index: usize) -> Option<T> {
        match index {
            0 => Some(self.x),
            1 => Some(self.y),
            _ => None,
        }
    }
}

impl<T: NumericType> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn from_collection<Z: NumericCollectionType<T>>(collection: Z) -> Result<Self, String> {
        if collection.len() != Some(2) {
            Err(String::from(
                "invalid collection length. cannot convert to Vec2",
            ))
        } else {
            Ok(Self {
                x: collection.get(0).unwrap(),
                y: collection.get(1).unwrap(),
            })
        }
    }
}
