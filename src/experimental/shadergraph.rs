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
        let pipeline = PipelineManager::new(
            PipelineType::Compute(shader.clone(), vec![texture.write_only(&context.device).0]),
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

        self.pipeline
            .do_compute_pass(&mut encoder, &vec![], &self.texture);

        context.queue.submit(std::iter::once(encoder.finish()));
    }

    pub fn read_texture() {}
    pub fn read_image() {}
}
