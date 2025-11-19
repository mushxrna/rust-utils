use bytemuck::Pod;
use bytemuck::cast_slice;
use std::any::TypeId;

use crate::generics::{NumericType, VectorType};

#[derive(Clone)]
pub struct ManagedBytes {
    pub bytes: Vec<u8>,
    pub typename: String,
}

impl ManagedBytes {
    pub fn release<T: Pod>(self) -> Vec<T> {
        cast_slice(&self.bytes).into()
    }
}

impl Byteable for ManagedBytes {
    fn to_raw_bytes(self) -> Vec<u8> {
        self.bytes
    }

    fn to_managed_bytes(self) -> ManagedBytes {
        self
    }

    fn as_raw_bytes(&self) -> Vec<u8> {
        self.clone().to_raw_bytes()
    }
}

pub trait Byteable {
    fn to_raw_bytes(self) -> Vec<u8>;
    fn to_managed_bytes(self) -> ManagedBytes;
    fn as_raw_bytes(&self) -> Vec<u8>;
}

impl<T: NumericType> Byteable for T {
    fn to_raw_bytes(self) -> Vec<u8> {
        bytemuck::cast_slice(&[self]).into()
    }
    fn to_managed_bytes(self) -> ManagedBytes {
        ManagedBytes {
            bytes: self.to_raw_bytes(),
            typename: std::any::type_name::<T>().to_owned(),
        }
    }
    fn as_raw_bytes(&self) -> Vec<u8> {
        self.clone().to_raw_bytes()
    }
}

impl<T: NumericType> Byteable for &[T] {
    fn to_raw_bytes(self) -> Vec<u8> {
        bytemuck::cast_slice(self).into()
    }
    fn to_managed_bytes(self) -> ManagedBytes {
        ManagedBytes {
            bytes: self.to_raw_bytes(),
            typename: std::any::type_name::<T>().to_owned(),
        }
    }

    fn as_raw_bytes(&self) -> Vec<u8> {
        self.to_owned().to_raw_bytes()
    }
}

impl<T: NumericType> Byteable for Vec<T> {
    fn to_raw_bytes(self) -> Vec<u8> {
        bytemuck::cast_slice(&self).into()
    }
    fn to_managed_bytes(self) -> ManagedBytes {
        ManagedBytes {
            bytes: self.to_raw_bytes(),
            typename: std::any::type_name::<T>().to_owned(),
        }
    }
    fn as_raw_bytes(&self) -> Vec<u8> {
        self.clone().to_raw_bytes()
    }
}
