use wgpu::{BindGroup, BindGroupLayout};

use crate::wgpu_helpers::{
    BufferManager, TextureManager, WgpuContextManager, config_enums::*, pod_types::*,
};

use crate::vectors::*;

pub struct PipelineManager {
    pipeline: wgpu::ComputePipeline,
    bind_groups: Vec<wgpu::BindGroup>,
    bind_group_layouts: Vec<BindGroupLayout>,
}

impl PipelineManager {
    pub fn do_compute_pass(&mut self, context: &WgpuContextManager, size: Vec2<u32>) {
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

            compute_pass.set_pipeline(&self.pipeline);

            for i in 0..self.bind_groups.len() {
                compute_pass.set_bind_group(i as u32, &self.bind_groups[i], &[]);
            }

            let workgroup_size = 16;
            let dispatch_x = (size.x + workgroup_size - 1) / workgroup_size;
            let dispatch_y = (size.y + workgroup_size - 1) / workgroup_size;

            compute_pass.dispatch_workgroups(dispatch_x, dispatch_y, 1);
        }
        context.queue.submit(std::iter::once(encoder.finish()));
    }

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
            pipeline,
            bind_groups,
            bind_group_layouts,
        }
    }
}
