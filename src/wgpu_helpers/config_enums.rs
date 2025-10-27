use crate::vectors::*;
use crate::wgpu_helpers::{WgpuContextManager, contextmanager};
use crate::wgpu_helpers::{
    errors::{self, *},
    pod_types::*,
    texturemanager::*,
};

trait PipelineType {}
impl PipelineType for wgpu::ComputePipeline {}
impl PipelineType for wgpu::RenderPipeline {}

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

#[derive(Debug, Clone)]
pub enum ActivePipeline {
    Compute(wgpu::ComputePipeline),
    Render(wgpu::RenderPipeline),
}

impl ActivePipeline {
    pub fn into_compute_pipeline(self) -> Result<wgpu::ComputePipeline, PipelineError> {
        match self {
            ActivePipeline::Compute(pipe) => Ok(pipe),
            _ => Err(PipelineError::WrongType(self)),
        }
    }
    pub fn into_render_pipeline(self) -> Result<wgpu::RenderPipeline, PipelineError> {
        match self {
            ActivePipeline::Render(pipe) => Ok(pipe),
            _ => Err(PipelineError::WrongType(self)),
        }
    }
}

pub enum ShaderSource {
    Compute(&'static str, String),
    Fragment(&'static str, String),
    Vertex(&'static str, String),
}

impl ShaderSource {
    pub fn into_module(
        self,
        context: &WgpuContextManager,
    ) -> Result<wgpu::ShaderModule, errors::ProcessError> {
        let shader = match &self {
            ShaderSource::Compute(src, _) => Ok(src),
            _ => Err(ProcessError::NotImplemented()),
        };

        let descriptor = wgpu::ShaderModuleDescriptor {
            label: match &self {
                ShaderSource::Compute(_, label) => Some(label),
                _ => None,
            },
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::from(*shader?)),
        };

        Ok(context.device.create_shader_module(descriptor))
    }
}

pub enum BufferType {
    Vertex(&'static [Vertex]),
    Index(&'static [u16]),
    Uniform(UniformsArray),
    Storage(Vec<u8>, AccessMode),
    ImageBuffer(Vec2<u32>),
}
