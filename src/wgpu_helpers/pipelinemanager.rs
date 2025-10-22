use crate::{
    vectors::*,
    wgpu_helpers::{errors::PipelineError, *},
};
use wgpu::{BindGroup, BindGroupLayout};

pub struct PipelineManager {
    pipeline: ActivePipeline,
    bind_groups: Vec<wgpu::BindGroup>,
    bind_group_layouts: Vec<BindGroupLayout>,
}

impl PipelineManager {
    pub fn do_compute_pass(
        &mut self,
        context: &WgpuContextManager,
        size: Vec2<u32>,
    ) -> Result<(), PipelineError> {
        let mut encoder = context
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("TODO."),
            });

        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("compute pass"),
                timestamp_writes: None,
            });

            match self.pipeline.clone() {
                ActivePipeline::Compute(pipe) => compute_pass.set_pipeline(&pipe),
                _ => {
                    return Err(PipelineError::PassError(
                        PipelineError::Specific(String::from(
                            "Compute pass attempted on Render Pipeline.",
                        ))
                        .into(),
                    ));
                }
            }

            for i in 0..self.bind_groups.len() {
                compute_pass.set_bind_group(i as u32, &self.bind_groups[i], &[]);
            }

            let workgroup_size = 16;
            let dispatch_x = (size.x + workgroup_size - 1) / workgroup_size;
            let dispatch_y = (size.y + workgroup_size - 1) / workgroup_size;

            compute_pass.dispatch_workgroups(dispatch_x, dispatch_y, 1);
        }
        context.queue.submit(std::iter::once(encoder.finish()));
        Ok(())
    }

    fn do_render_pass(&mut self, context: &WgpuContextManager, size: Vec2<u32>) {}

    pub fn new(
        shader: wgpu::ShaderModule,
        bind_groups: Vec<BindGroup>,
        bind_group_layouts: Vec<BindGroupLayout>,
        context: &WgpuContextManager,
    ) -> Self {
        let layout_refs: Vec<&wgpu::BindGroupLayout> = bind_group_layouts.iter().collect();
        let pipeline = context
            .device
            .create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some("Compute Pipeline"),
                layout: Some(&context.device.create_pipeline_layout(
                    &wgpu::PipelineLayoutDescriptor {
                        label: Some("Compute Pipeline Layout"),
                        bind_group_layouts: &layout_refs,
                        push_constant_ranges: &[],
                    },
                )),
                module: &shader,
                entry_point: Some("cs_main"),
                compilation_options: Default::default(),
                cache: Default::default(),
            });

        Self {
            pipeline: ActivePipeline::Compute(pipeline),
            bind_groups,
            bind_group_layouts,
        }
    }
}
