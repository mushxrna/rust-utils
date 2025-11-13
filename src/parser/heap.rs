use bytemuck::AnyBitPattern;

use crate::generics::Byteable;
use crate::parser::types::*;

pub struct BytePointer {
    index: usize,
    primitive: PrimitiveType,
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

    pub fn insert<T: Byteable>(&mut self, obj: T, primitive: PrimitiveType) -> BytePointer {
        let bytes = obj.to_raw_bytes();

        let ptr = BytePointer {
            index: self.last_occupied_index,
            primitive,
        };

        bytes
            .iter()
            .enumerate()
            .for_each(|(i, b)| self.heap[self.last_occupied_index + i] = *b);

        self.last_occupied_index += bytes.len();

        ptr
    }

    pub fn view<T: Byteable + AnyBitPattern>(&self, ptr: BytePointer) -> Vec<T> {
        let range = &self.heap[ptr.index..ptr.index + ptr.primitive.byte_length()];
        bytemuck::cast_slice(range).to_owned()
    }

    pub fn get_occupied_slice(&self) -> &[u8] {
        &self.heap[0..self.last_occupied_index]
    }
}
