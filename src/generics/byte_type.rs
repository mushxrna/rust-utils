use bytemuck::NoUninit;
use bytemuck::Pod;
use bytemuck::cast_slice;
use std::any::TypeId;

use crate::generics::{NumericType, VectorType};

pub trait Byteable {
    fn to_bytes(self) -> Vec<u8>;
    fn copy_bytes(&self) -> Vec<u8>;
}

impl<T: NumericType> Byteable for T {
    fn to_bytes(self) -> Vec<u8> {
        bytemuck::cast_slice(&[self]).into()
    }
    fn copy_bytes(&self) -> Vec<u8> {
        self.to_owned().to_bytes()
    }
}

impl<T: Pod> Byteable for &[T] {
    fn to_bytes(self) -> Vec<u8> {
        bytemuck::cast_slice(self).into()
    }
    fn copy_bytes(&self) -> Vec<u8> {
        self.to_owned().to_bytes()
    }
}

impl<T: Pod> Byteable for Vec<T> {
    fn to_bytes(self) -> Vec<u8> {
        bytemuck::cast_slice(&self).into()
    }

    fn copy_bytes(&self) -> Vec<u8> {
        self.clone().to_bytes()
    }
}
