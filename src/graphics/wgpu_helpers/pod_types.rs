use crate::graphics::vectors::Vec3;
use crate::graphics::wgpu_helpers::config_enums;

pub trait PodType: Copy + Clone + bytemuck::Pod + bytemuck::Zeroable {
    fn to_bytes(self) -> Vec<u8> {
        bytemuck::cast_slice(&[self]).to_vec()
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 4],
}

impl Vertex {
    pub fn new(position: Vec3<f32>) -> Vertex {
        Vertex {
            position: [position.x, position.y, position.z, 1.0],
        }
    }

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[wgpu::VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float32x3,
            }],
        }
    }
}

impl PodType for Vertex {}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct UniformsArray {
    pub time: f32,
    pub voxel_dim: f32,
    pub prime_index: f32,
    pub _padding: f32,
    pub camera_position: [f32; 4],
    pub mouse_movement: [f32; 4],
}

impl UniformsArray {
    pub fn new() -> Self {
        Self {
            time: 0.0,
            voxel_dim: 0.0,
            prime_index: 0.0,
            _padding: 0.0,
            camera_position: [0.0; 4],
            mouse_movement: [0.0; 4],
        }
    }
}

impl PodType for UniformsArray {}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RayHitInfo {
    pub position: [f32; 4],
    pub normal: [f32; 4],
    pub color: [f32; 4],
}

impl RayHitInfo {
    pub fn new_empty() -> Self {
        Self {
            position: [0.0; 4],
            normal: [0.0; 4],
            color: [0.0; 4],
        }
    }
}

impl PodType for RayHitInfo {}
