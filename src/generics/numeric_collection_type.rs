use crate::{
    generics::{NumericType, VectorType},
    graphics::vectors::Vec2,
};

pub trait NumericCollectionType<T: NumericType> {
    fn len(&self) -> Option<usize>;
    fn get(&self, index: usize) -> Option<T>;
}

impl<T: NumericType> NumericCollectionType<T> for (T, T) {
    fn len(&self) -> Option<usize> {
        Some(2)
    }

    fn get(&self, index: usize) -> Option<T> {
        match index {
            0 => Some(self.0),
            1 => Some(self.1),
            _ => None,
        }
    }
}

impl<T: NumericType> NumericCollectionType<T> for (T, T, T) {
    fn len(&self) -> Option<usize> {
        Some(3)
    }

    fn get(&self, index: usize) -> Option<T> {
        match index {
            0 => Some(self.0),
            1 => Some(self.1),
            2 => Some(self.2),
            _ => None,
        }
    }
}

impl<T: NumericType> NumericCollectionType<T> for (T, T, T, T) {
    fn len(&self) -> Option<usize> {
        Some(4)
    }

    fn get(&self, index: usize) -> Option<T> {
        match index {
            0 => Some(self.0),
            1 => Some(self.1),
            2 => Some(self.2),
            3 => Some(self.3),
            _ => None,
        }
    }
}

impl<T: NumericType> NumericCollectionType<T> for [T; 2] {
    fn len(&self) -> Option<usize> {
        Some(2)
    }

    fn get(&self, index: usize) -> Option<T> {
        match index {
            0 => Some(self[0]),
            1 => Some(self[1]),
            _ => None,
        }
    }
}

impl<T: NumericType> NumericCollectionType<T> for [T; 3] {
    fn len(&self) -> Option<usize> {
        Some(3)
    }

    fn get(&self, index: usize) -> Option<T> {
        match index {
            0 => Some(self[0]),
            1 => Some(self[1]),
            2 => Some(self[2]),
            _ => None,
        }
    }
}

impl<T: NumericType> NumericCollectionType<T> for [T; 4] {
    fn len(&self) -> Option<usize> {
        Some(4)
    }

    fn get(&self, index: usize) -> Option<T> {
        match index {
            0 => Some(self[0]),
            1 => Some(self[1]),
            2 => Some(self[2]),
            3 => Some(self[3]),
            _ => None,
        }
    }
}
