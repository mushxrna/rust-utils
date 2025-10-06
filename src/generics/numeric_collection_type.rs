use crate::NumericType;

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
