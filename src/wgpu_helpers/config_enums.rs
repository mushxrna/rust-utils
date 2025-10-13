use crate::vectors::*;
use crate::wgpu_helpers::{pod_types::*, texturemanager::*};

#[derive(Clone, Copy, Debug)]
pub enum AccessMode {
    Read,
    Write,
    ReadWrite,
}

impl AccessMode {
    pub fn into_buffer_mode(self) -> bool {
        match self {
            AccessMode::Read => true,
            AccessMode::Write => false,
            AccessMode::ReadWrite => false,
        }
    }
}

pub enum ShaderSource {
    Compute(&'static str),
    Fragment(&'static str),
    Vertex(&'static str),
}

pub enum BufferType {
    Vertex(&'static [Vertex]),
    Index(&'static [u16]),
    Uniform(UniformsArray),
    Storage(Vec<u8>, AccessMode),
    ImageBuffer(Vec2<u32>),
}
