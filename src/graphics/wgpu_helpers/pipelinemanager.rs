use crate::graphics::wgpu_helpers::{WgpuContextManager, common::*};
use std::borrow::Cow;

pub struct RenderPipelineManager {
    pipe: RenderPipeline,
}

impl RenderPipelineManager {
    pub fn new(ctx: &WgpuContextManager, shader_src: &str) -> Self {
        let shader = ctx.device().create_shader_module(ShaderModuleDescriptor {
            label: None,
            source: ShaderSource::Wgsl(Cow::Borrowed(shader_src)),
        });

        let pipe = ctx
            .device()
            .create_render_pipeline(&RenderPipelineDescriptor {
                label: None,
                layout: None,
                vertex: VertexState {
                    module: &shader,
                    entry_point: Some("vs_main"),
                    buffers: &[],
                    compilation_options: Default::default(),
                },
                fragment: Some(FragmentState {
                    module: &shader,
                    entry_point: Some("fs_main"),
                    targets: &[Some(ctx.format().clone().into())],
                    compilation_options: Default::default(),
                }),
                primitive: Default::default(),
                depth_stencil: None,
                multisample: Default::default(),
                multiview: None,
                cache: None,
            });

        Self { pipe }
    }
}
