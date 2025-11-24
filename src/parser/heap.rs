use bytemuck::{AnyBitPattern, NoUninit};
use std::{
    cell::{Ref, RefCell},
    fmt::Debug,
    hash::Hash,
    ops::Deref,
};

use crate::generics::{Byteable, NumericType};

pub trait HeapPtr: Debug {
    fn raw(&self) -> usize;
    fn len(&self) -> usize;
}

pub trait TypedHeapPtr<T>: HeapPtr {}

pub trait ByteHeap {
    fn insert_bytes<T: Byteable>(&self, obj: T) -> Result<impl HeapPtr, String>;
    fn view_untyped(&self, ptr: impl HeapPtr) -> Result<impl Deref<Target = [u8]>, String>;

    //fn copy_slice_into<T>(&self, slice: T) -> Result<impl HeapPtr, String> {

    //}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct XPtr {
    index: usize,
    byte_len: usize,
}

impl XPtr {
    pub fn new(index: usize, byte_len: usize) -> Self {
        Self { index, byte_len }
    }
}

impl HeapPtr for XPtr {
    fn raw(&self) -> usize {
        self.index
    }
    fn len(&self) -> usize {
        self.byte_len
    }
}

impl HeapPtr for &dyn HeapPtr {
    fn raw(&self) -> usize {
        HeapPtr::raw(*self)
    }
    fn len(&self) -> usize {
        HeapPtr::len(*self)
    }
}

pub struct XHeap {
    heap: RefCell<Box<[u8]>>,
    last_occupied_index: RefCell<usize>,
}

impl XHeap {
    pub fn new(heapsize: u32) -> Self {
        Self {
            heap: RefCell::new(vec![0; heapsize as usize].into_boxed_slice()),
            last_occupied_index: RefCell::new(0),
        }
    }
}

impl ByteHeap for XHeap {
    fn insert_bytes<T: Byteable>(&self, obj: T) -> Result<impl HeapPtr, String> {
        let bytes = obj.to_raw_bytes();

        let start = *self.last_occupied_index.borrow();
        let len = bytes.len();

        for i in 0..len {
            let current = start + i;
            self.heap.borrow_mut()[current] = bytes[i];
        }

        self.last_occupied_index.replace(start + len);

        Ok(XPtr::new(start, bytes.len()))
    }

    fn view_untyped(&self, ptr: impl HeapPtr) -> Result<impl Deref<Target = [u8]>, String> {
        let heap = self.heap.borrow();
        Ok(Ref::map(heap, |b| &b[ptr.raw()..ptr.raw() + ptr.len()]))
    }
}
