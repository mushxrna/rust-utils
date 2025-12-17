use std::{marker::PhantomData, mem};

use thiserror::Error;

use crate::generics::Byteable;

const MANAGED_FLAG: u8 = 0b1000_0000;

//
//      MACROS
//
macro_rules! flag_check {
    ($flag:expr, $item:expr) => {
        ($item & $flag == $flag)
    };
}
//
//      ERRORS
//
#[derive(Debug, Error)]
pub enum ByteHeapError {
    #[error("Could not insert item.")]
    InsertError,
    #[error("No avalaible space in the heap.")]
    AllocError,
}
use ByteHeapError as E;
//
//      STRUCTS
//
pub struct Accessor<'a, T> {
    value: &'a [u8],
    _p: PhantomData<T>,
}

pub struct ByteHeap<const SIZE: usize> {
    bytes: Box<[u8; SIZE]>,
    allocation_flags: Box<[u8; SIZE]>,
}
//
//      STRUCT IMPLS
//
impl<A: Byteable + bytemuck::Pod> Accessor<'_, A> {
    pub fn retrieve(&self) -> &A {
        let x: &[A] = bytemuck::cast_slice(self.value);
        &x[0]
    }
}

impl<const S: usize> ByteHeap<S> {
    //
    //      PRIVATE
    //
    fn find_available(&self, space_in_bytes: usize) -> Result<usize, E> {
        let mut consecutive = 0;
        let mut start = 0;

        for i in 0..self.bytes.len() {
            if flag_check!(MANAGED_FLAG, self.allocation_flags[i]) {
                consecutive = 0;
                start = i + 1;
            } else {
                consecutive += 1;
            }

            if consecutive == space_in_bytes {
                return Ok(start);
            }
        }

        Err(E::AllocError)
    }
    //
    //      RAW FNS
    //
    pub fn raw_insert<A: Byteable>(&mut self, item: A) -> Result<usize, E> {
        let bytes = item.to_bytes();
        let index = self.find_available(bytes.len())?;

        let insertion_region = self
            .bytes
            .get_mut(index..index + bytes.len())
            .ok_or(E::InsertError)?;

        for (x, i) in insertion_region.iter_mut().enumerate() {
            *i = bytes[x];
            self.allocation_flags[x] = 0b1000_0000
        }

        Ok(index)
    }

    pub fn raw_retrieve(&self, range: std::ops::Range<usize>) -> &[u8] {
        &self.bytes[range]
    }
    //
    //
    //
    pub fn insert<'a, A: Byteable>(&'a mut self, item: A) -> Result<Accessor<'a, A>, E> {
        let i = self.raw_insert(item)?;
        let l = mem::size_of::<A>();

        let r = self.raw_retrieve(i..l);

        Ok(Accessor {
            value: r,
            _p: PhantomData,
        })
    }
    //
    //      CONSTRUCTOR
    //
    pub fn new() -> ByteHeap<S> {
        ByteHeap {
            bytes: Box::new([0; S]),
            allocation_flags: Box::new([0; S]),
        }
    }
}
//
//      DEFAULT IMPLS
//
impl<const S: usize> Default for ByteHeap<S> {
    fn default() -> Self {
        Self::new()
    }
}
