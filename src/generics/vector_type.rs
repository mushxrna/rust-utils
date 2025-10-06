use crate::NumericType;

pub trait VectorType<T: NumericType> {
    fn len(&self) -> usize;
    fn get(&self, index: usize) -> Option<T>;
}
