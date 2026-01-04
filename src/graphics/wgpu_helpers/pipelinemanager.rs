use crate::graphics::wgpu_helpers::{WgpuContextManager, common::*};
use std::borrow::Cow;

pub struct RenderPipelineManager {
    pipe: RenderPipeline,
}

impl RenderPipelineManager {
    pub fn pipe(&self) -> &RenderPipeline {
        &self.pipe
    }

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
                cache: None,
                multiview_mask: None,
            });

        Self { pipe }
    }

    pub fn render_pass(&self, ctx: &WgpuContextManager) {
        let surface = ctx
            .surface()
            .get_current_texture()
            .expect("Couldn't get surface texture!");
        let view = surface
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = ctx
            .device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut r_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            r_pass.set_pipeline(self.pipe());
            r_pass.draw(0..3, 0..1);
        }

        ctx.queue().submit(Some(encoder.finish()));
        surface.present();
    }
}
