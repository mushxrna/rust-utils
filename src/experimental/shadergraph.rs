use crate::wgpu_helpers::*;
use std::rc::Rc;
use wgpu::ShaderModule;

pub struct ShaderNode {
    pub shader: ShaderModule,
    pub input_reads: Option<Vec<Rc<ShaderNode>>>,
    pub label: String,
    pub texture: TextureManager,
    pub pipeline: PipelineManager,
}

impl ShaderNode {
    pub fn new(
        shader: ShaderModule,
        context: &WgpuContextManager,
        label: String,
        texture: TextureManager,
        inputs: Option<Vec<Rc<ShaderNode>>>,
    ) -> Self {
        // Use the bind group layout from the texture which is already configured based on access mode
        let pipeline = PipelineManager::new(
            PipelineType::Compute(shader.clone(), vec![texture.bind_group_layout.clone()]),
            &context.device,
        );

        let input_reads = match inputs {
            None => None,
            _ => None,
        };

        Self {
            shader,
            input_reads,
            label,
            texture,
            pipeline,
        }
    }

    pub fn execute(&mut self, context: &WgpuContextManager) {
        let mut encoder = context
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some(&self.label),
            });

        // Use the bind group directly from the texture - it's already configured correctly
        self.pipeline
            .do_compute_pass(&mut encoder, &vec![], &self.texture);

        context.queue.submit(std::iter::once(encoder.finish()));
    }

    pub fn read_texture() {}
    pub fn read_image() {}
}
