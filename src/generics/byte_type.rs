use bytemuck::NoUninit;
use bytemuck::Pod;
use bytemuck::cast_slice;
use std::any::TypeId;

use crate::generics::{NumericType, VectorType};

pub trait Byteable {
    fn to_raw_bytes(self) -> Vec<u8>;
    fn as_raw_bytes(&self) -> Vec<u8>;
}
/*
impl<T: Pod> Byteable for T {
    fn to_raw_bytes(self) -> Vec<u8> {
        bytemuck::cast_slice(&[self]).into()
    }
    fn as_raw_bytes(&self) -> Vec<u8> {
        self.clone().to_raw_bytes()
    }
}
*/

impl<T: NumericType> Byteable for T {
    fn to_raw_bytes(self) -> Vec<u8> {
        bytemuck::cast_slice(&[self]).into()
    }
    fn as_raw_bytes(&self) -> Vec<u8> {
        self.to_owned().to_raw_bytes()
    }
}

impl<T: Pod> Byteable for &[T] {
    fn to_raw_bytes(self) -> Vec<u8> {
        bytemuck::cast_slice(self).into()
    }
    fn as_raw_bytes(&self) -> Vec<u8> {
        self.to_owned().to_raw_bytes()
    }
}

impl<T: Pod> Byteable for Vec<T> {
    fn to_raw_bytes(self) -> Vec<u8> {
        bytemuck::cast_slice(&self).into()
    }

    fn as_raw_bytes(&self) -> Vec<u8> {
        self.clone().to_raw_bytes()
    }
}
