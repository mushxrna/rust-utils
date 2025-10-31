use crate::generics::VectorType;

struct ManagedBytes {
    bytes: Vec<u8>,
}

pub trait Byteable {
    fn to_raw_bytes(self) -> Vec<u8>;
    fn to_managed_bytes(self) -> ManagedBytes;
}

impl<T: VectorType<f32>> Byteable for T {
    fn to_raw_bytes(self) -> Vec<u8> {
        let bytes: Vec<u8> = bytemuck::cast_slice(&self.to_collection()).into();
        bytes
    }
    fn to_managed_bytes(self) -> ManagedBytes {
        ManagedBytes {
            bytes: bytemuck::cast_slice(&self.to_collection()).into(),
        }
    }
}
