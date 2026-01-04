pub mod common {
    pub use crate::graphics::wgpu_helpers::errors::*;
    pub use wgpu::{
        Adapter, Device, FragmentState, Instance, Queue, RenderPipeline, RenderPipelineDescriptor,
        ShaderModuleDescriptor, ShaderSource, Surface, SurfaceConfiguration, Texture,
        TextureFormat, VertexState,
    };
}

mod contextmanager;
mod errors;
mod pipelinemanager;
pub use contextmanager::WgpuContextManager;
pub use pipelinemanager::RenderPipelineManager;
