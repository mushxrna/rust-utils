use std::collections::HashMap;
use thiserror::Error;
//
//      ERRORS
//
#[derive(Debug, Error)]
pub enum ByteMapError {
    #[error("No keys left.")]
    MaxCapacityError,
    #[error("Cast error.")]
    CastError,
}
use ByteMapError as E;
//
//      STRUCTS
//
pub struct ByteMap<const BYTELEN: usize, V> {
    map: HashMap<[u8; BYTELEN], V>,
    capacity: usize, //2^(BYTELEN * 8) - 1
    occupied_keys: usize,
}

impl<const BYTELEN: usize, V> ByteMap<BYTELEN, V> {
    //
    //      PRIVATE
    //
    fn generate_key(&mut self) -> Result<[u8; BYTELEN], E> {
        if self.occupied_keys < self.capacity {
            let key = self.occupied_keys + 1;
            let key_b: [u8; BYTELEN] = usize::to_ne_bytes(key)[0..BYTELEN]
                .try_into()
                .map_err(|z| E::CastError)?;
            self.occupied_keys += 1;
            Ok(key_b)
        } else {
            Err(E::MaxCapacityError)
        }
    }
    //
    //
    //
    pub fn insert(&mut self, item: V) -> Result<[u8; BYTELEN], E> {
        let key = self.generate_key()?;
        self.map.insert(key, item);
        Ok(key)
    }

    pub fn retrieve(&self, key: [u8; BYTELEN]) -> Option<&V> {
        self.map.get(&key)
    }
    //
    //      CONSTRUCTOR
    //
    pub fn new() -> Self {
        let cap = (2 as usize).pow(BYTELEN as u32 * 8) - 1;
        Self {
            map: HashMap::new(),
            capacity: cap,
            occupied_keys: 0,
        }
    }
}
