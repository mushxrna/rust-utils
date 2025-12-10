use thiserror::Error;

use crate::generics::Byteable;

const MANAGED_FLAG: u8 = 0b1000_0000;

macro_rules! flag_check {
    ($flag:expr, $item:expr) => {
        ($item & $flag == $flag)
    };
}

#[derive(Debug, Error)]
pub enum ByteHeapError {
    #[error("Could not insert item.")]
    InsertError,
    #[error("No avalaible space in the heap.")]
    AllocError,
}

use ByteHeapError as E;

pub struct ByteHeap<const SIZE: usize> {
    bytes: Box<[u8; SIZE]>,
    allocation_flags: Box<[u8; SIZE]>,
}

impl<const S: usize> ByteHeap<S> {
    fn find_available(&self, space_in_bytes: usize) -> Result<usize, E> {
        let mut consecutive = 0;
        let mut start = 0;

        for (i) in 0..self.bytes.len() {
            if !flag_check!(self.allocation_flags[i], MANAGED_FLAG) {
                consecutive += 1;
            }

            if flag_check!(self.allocation_flags[i], MANAGED_FLAG) {
                consecutive = 0;
            }

            if consecutive == space_in_bytes {
                return Ok(i - consecutive);
            }
        }

        Err(E::AllocError)
    }

    pub fn insert<A: Byteable>(&mut self, item: A) -> Result<usize, E> {
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

    pub fn new() -> ByteHeap<S> {
        ByteHeap {
            bytes: Box::new([0; S]),
            allocation_flags: Box::new([0; S]),
        }
    }
}
