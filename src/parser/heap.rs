use bytemuck::{AnyBitPattern, NoUninit};
use std::marker::PhantomData;

use crate::generics::{Byteable, NumericType};

pub struct BytePointer<T: Byteable> {
    index: usize,
    byte_len: usize,
    primitive: PhantomData<T>,
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

    pub fn insert<T: Byteable>(&mut self, obj: T) -> BytePointer<T> {
        let bytes = obj.to_raw_bytes();
        /*
        let bytes = obj
            .iter()
            .map(|e| -> Vec<u8> { bytemuck::cast_slice(&[*e]).to_owned() })
            .collect::<Vec<Vec<u8>>>()
            .concat();
        */
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
