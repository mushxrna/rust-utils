use crate::generics::NumericType;

pub trait VectorType<T: NumericType> {
    fn len(&self) -> usize;
    fn get(&self, index: usize) -> Option<T>;
    fn to_collection(&self) -> Vec<T> {
        let mut newvec: Vec<T> = vec![];
        for i in 0..self.len() {
            newvec.push(self.get(i).unwrap());
        }
        return newvec;
    }
}
