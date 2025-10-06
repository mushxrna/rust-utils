use crate::vectors::Vec3;
use crate::wgpu_helpers::config_enums;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
}

impl Vertex {
    pub fn new(position: Vec3<f32>) -> Vertex {
        Vertex {
            position: [position.x, position.y, position.z],
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

#[derive(Clone)]
pub struct VoxelArray3d {
    pub size: usize,
    pub flattened: Vec<f32>,
}

impl VoxelArray3d {
    pub fn new(size: usize) -> Self {
        let flattened = vec![0.0; size * size * size];

        Self { size, flattened }
    }

    pub fn index_to_coords(&self, index: usize) -> (usize, usize, usize) {
        let z = index / (self.size * self.size);
        let remainder = index % (self.size * self.size);
        let y = remainder / self.size;
        let x = remainder % self.size;

        (x, y, z)
    }

    pub fn coords_to_index(&self, coord: (usize, usize, usize)) -> usize {
        coord.0 + coord.1 * self.size + coord.2 * self.size * self.size
    }
}
