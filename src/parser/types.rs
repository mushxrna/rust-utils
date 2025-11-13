use crate::generics::Byteable;

pub enum PrimitiveType {
    Float32,
    Int32,
}

impl PrimitiveType {
    pub fn byte_length(&self) -> usize {
        match &self {
            PrimitiveType::Float32 | PrimitiveType::Int32 => 4,
        }
    }
}
