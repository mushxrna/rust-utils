use crate::vectors::*;
use crate::voxels::SerialOctreeNode;
use crate::wgpu_helpers::{pod_types::*, texturemanager::*};

pub enum ShaderSource {
    Compute(&'static str),
    Render(&'static str),
}

pub enum BufferType {
    Vertex(&'static [Vertex]),
    Index(&'static [u16]),
    Uniform(UniformsArray),
    VoxelStorage(VoxelArray3d),
    SvoStorage(Vec<SerialOctreeNode>),
    ImageBuffer(Vec2<u32>),
}

pub enum PipelineType {
    Render(
        wgpu::ShaderModule,
        TextureManager,
        wgpu::SurfaceConfiguration,
    ),
    Compute(wgpu::ShaderModule, Vec<wgpu::BindGroupLayout>),
}

pub enum Pipeline {
    Render(wgpu::RenderPipeline),
    Compute(wgpu::ComputePipeline),
}
