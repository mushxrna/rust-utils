use bytemuck::{AnyBitPattern, NoUninit};
use std::{fmt::Debug, hash::Hash, marker::PhantomData};

use crate::generics::{Byteable, NumericType};

pub trait BytePtr: Debug {
    fn as_raw_ptr(&self) -> u64;
    fn from_raw_ptr(raw_ptr: u64) -> Self
    where
        Self: Sized;

    fn into_f32_byte_pointer(self) -> BytePointer<f32>;
    fn into_i32_byte_pointer(self) -> BytePointer<i32>;
    fn as_f32_byte_pointer(&self) -> BytePointer<f32>;
    fn as_i32_byte_pointer(&self) -> BytePointer<i32>;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BytePointer<T: Byteable> {
    pub index: usize,
    pub byte_len: usize,
    primitive: PhantomData<T>,
}

impl<T: Byteable + Debug> BytePtr for BytePointer<T> {
    fn as_raw_ptr(&self) -> u64 {
        let ptr: u64 = ((self.index as u64) << 32) | (self.byte_len as u64);
        ptr
    }

    fn from_raw_ptr(raw_ptr: u64) -> Self {
        let index = (raw_ptr >> 32) as u32 as usize;
        let byte_len = (raw_ptr & 0xFFFFFFFF) as u32 as usize;

        Self {
            index,
            byte_len,
            primitive: PhantomData,
        }
    }

    fn into_f32_byte_pointer(self) -> BytePointer<f32> {
        BytePointer {
            index: self.index,
            byte_len: self.byte_len,
            primitive: PhantomData,
        }
    }

    fn into_i32_byte_pointer(self) -> BytePointer<i32> {
        BytePointer {
            index: self.index,
            byte_len: self.byte_len,
            primitive: PhantomData,
        }
    }

    fn as_f32_byte_pointer(&self) -> BytePointer<f32> {
        BytePointer {
            index: self.index,
            byte_len: self.byte_len,
            primitive: PhantomData,
        }
    }

    fn as_i32_byte_pointer(&self) -> BytePointer<i32> {
        BytePointer {
            index: self.index,
            byte_len: self.byte_len,
            primitive: PhantomData,
        }
    }
}

pub struct ByteHeap {
    heap: Box<[u8]>,
    last_occupied_index: usize,
}

impl ByteHeap {
    pub fn new(heapsize: u32) -> Self {
        Self {
            heap: vec![0; heapsize as usize].into_boxed_slice(),
            last_occupied_index: 0,
        }
    }

    pub fn insert<T: Byteable + NoUninit>(&mut self, obj: &[T]) -> BytePointer<T> {
        let bytes = obj
            .iter()
            .map(|e| -> Vec<u8> { bytemuck::cast_slice(&[*e]).to_owned() })
            .collect::<Vec<Vec<u8>>>()
            .concat();

        let ptr = BytePointer {
            index: self.last_occupied_index,
            byte_len: bytes.len(),
            primitive: PhantomData,
        };

        bytes
            .iter()
            .enumerate()
            .for_each(|(i, b)| self.heap[self.last_occupied_index + i] = *b);

        self.last_occupied_index += bytes.len();

        ptr
    }

    pub fn view<T: Byteable + AnyBitPattern>(&self, ptr: BytePointer<T>) -> Vec<T> {
        let range = &self.heap[ptr.index..ptr.index + ptr.byte_len];
        bytemuck::cast_slice(range).to_owned()
    }

    pub fn get_occupied_slice(&self) -> &[u8] {
        &self.heap[0..self.last_occupied_index]
    }
}
