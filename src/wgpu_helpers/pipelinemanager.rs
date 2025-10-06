use crate::wgpu_helpers::{BufferManager, TextureManager, config_enums::*, pod_types::*};

pub struct PipelineManager {
    kind: PipelineType,
    pipeline: Pipeline,
}

impl PipelineManager {
    pub fn get_compute_pipeline(&self) -> Option<&wgpu::ComputePipeline> {
        match &self.pipeline {
            Pipeline::Compute(pipeline) => Some(pipeline),
            _ => None,
        }
    }

    pub fn get_render_pipeline(&self) -> Option<&wgpu::RenderPipeline> {
        match &self.pipeline {
            Pipeline::Render(pipeline) => Some(pipeline),
            _ => None,
        }
    }

    pub fn do_compute_pass(
        &mut self,
        texture: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        buffers: &Vec<&BufferManager>,
        render_texture: &TextureManager,
    ) {
        let mut compute_pass = (*encoder).begin_compute_pass(&wgpu::ComputePassDescriptor {
            label: Some("compute pass"),
            timestamp_writes: None,
        });

        compute_pass.set_pipeline(self.get_compute_pipeline().unwrap());

        compute_pass.set_bind_group(0, &render_texture.cs_bg, &[]);

        for i in 0..buffers.len() {
            compute_pass.set_bind_group(i as u32 + 1, buffers[i].bind_group.as_ref().unwrap(), &[]);
        }

        let workgroup_size = 16;
        let dispatch_x = (render_texture.size.x + workgroup_size - 1) / workgroup_size;
        let dispatch_y = (render_texture.size.y + workgroup_size - 1) / workgroup_size;

        compute_pass.dispatch_workgroups(dispatch_x, dispatch_y, 1);
    }

    pub fn do_render_pass(
        &mut self,
        texture: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        buffers: &Vec<BufferManager>,
        render_texture: &TextureManager,
        indices_num: u32,
    ) {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                depth_slice: None,
                view: &texture,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });
        render_pass.set_pipeline(self.get_render_pipeline().unwrap());
        render_pass.set_bind_group(0, &render_texture.r_bg, &[]);
        render_pass.set_vertex_buffer(0, buffers[0].buffer.slice(..));
        render_pass.set_index_buffer(buffers[1].buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..indices_num, 0, 0..1)
    }

    pub fn new(kind: PipelineType, device: &wgpu::Device) -> Self {
        let pipeline = match &kind {
            PipelineType::Compute(shader, bindgroup_layouts) => {
                let layout_refs: Vec<&wgpu::BindGroupLayout> = bindgroup_layouts.iter().collect();
                Pipeline::Compute(device.create_compute_pipeline(
                    &wgpu::ComputePipelineDescriptor {
                        label: Some("Compute Pipeline"),
                        layout: Some(&device.create_pipeline_layout(
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
                    },
                ))
            }

            PipelineType::Render(shader, texture, config) => {
                let render_pipeline_layout =
                    device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                        label: Some("Render Pipeline Layout"),
                        bind_group_layouts: &[&texture.r_bg_layout],
                        push_constant_ranges: &[],
                    });

                Pipeline::Render(
                    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                        label: Some("Render Pipeline"),
                        layout: Some(&render_pipeline_layout),
                        vertex: wgpu::VertexState {
                            module: &shader,
                            entry_point: Some("vs_main"),
                            buffers: &[Vertex::desc()],
                            compilation_options: wgpu::PipelineCompilationOptions::default(),
                        },
                        fragment: Some(wgpu::FragmentState {
                            module: &shader,
                            entry_point: Some("fs_main"),
                            targets: &[Some(wgpu::ColorTargetState {
                                format: config.format,
                                blend: Some(wgpu::BlendState::REPLACE),
                                write_mask: wgpu::ColorWrites::ALL,
                            })],
                            compilation_options: wgpu::PipelineCompilationOptions::default(),
                        }),
                        primitive: wgpu::PrimitiveState {
                            topology: wgpu::PrimitiveTopology::TriangleList,
                            strip_index_format: None,
                            front_face: wgpu::FrontFace::Ccw,
                            cull_mode: None,
                            polygon_mode: wgpu::PolygonMode::Fill,
                            unclipped_depth: false,
                            conservative: false,
                        },
                        depth_stencil: None,
                        multisample: wgpu::MultisampleState {
                            count: 1,
                            mask: !0,
                            alpha_to_coverage_enabled: false,
                        },
                        multiview: None,
                        cache: None,
                    }),
                )
            }
        };

        Self { kind, pipeline }
    }
}
