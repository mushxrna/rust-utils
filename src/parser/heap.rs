use crate::generics::Byteable;

pub struct BytePointer<T: Byteable> {
    pub index: usize,
    pub primitive: T,
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

    pub fn insert<T: Byteable>(&mut self, obj: T) {
        let bytes = obj.to_raw_bytes();

        bytes
            .iter()
            .enumerate()
            .for_each(|(i, b)| self.heap[self.last_occupied_index + i] = *b);

        self.last_occupied_index += bytes.len();
    }

    pub fn get_occupied_slice(&self) -> &[u8] {
        &self.heap[0..self.last_occupied_index]
    }
}
