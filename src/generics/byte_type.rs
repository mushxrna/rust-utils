use bytemuck::Pod;
use bytemuck::cast_slice;

use crate::generics::{NumericType, VectorType};

pub struct ManagedBytes {
    pub bytes: Vec<u8>,
    pub typename: String,
}

impl ManagedBytes {
    pub fn release<T: Pod>(self) -> Vec<T> {
        cast_slice(&self.bytes).into()
    }
}

pub trait Byteable {
    fn to_raw_bytes(self) -> Vec<u8>;
    fn to_managed_bytes(self) -> ManagedBytes;
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
}

pub fn test() {
    let x = &[1, 2];
    x.to_raw_bytes();
}
