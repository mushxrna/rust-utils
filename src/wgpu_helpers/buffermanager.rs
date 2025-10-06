use crate::wgpu_helpers::config_enums::*;
use wgpu::util::DeviceExt;

pub struct BufferManager {
    pub kind: BufferType,
    pub buffer: wgpu::Buffer,
    pub bind_group_layout: Option<wgpu::BindGroupLayout>,
    pub bind_group: Option<wgpu::BindGroup>,
}

impl BufferManager {
    pub fn new(kind: BufferType, device: &wgpu::Device) -> Self {
        let buffer = match &kind {
            BufferType::Vertex(vertices) => {
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("vertex buffer"),
                    usage: wgpu::BufferUsages::VERTEX,
                    contents: bytemuck::cast_slice(vertices),
                })
            }
            BufferType::Index(indices) => {
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("index buffer"),
                    usage: wgpu::BufferUsages::INDEX,
                    contents: bytemuck::cast_slice(indices),
                })
            }
            BufferType::Uniform(uniform_array) => {
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("uniform buffer"),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                    contents: bytemuck::cast_slice(&[*uniform_array]),
                })
            }
            BufferType::VoxelStorage(voxel_array) => {
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("voxel storage buffer"),
                    usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                    contents: bytemuck::cast_slice(&voxel_array.flattened),
                })
            }
        };

        let bind_group_layout = match &kind {
            BufferType::Uniform(uniform_array) => Some(device.create_bind_group_layout(
                &wgpu::BindGroupLayoutDescriptor {
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX
                            | wgpu::ShaderStages::FRAGMENT
                            | wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                    label: Some("uniform bind group layout"),
                },
            )),
            BufferType::VoxelStorage(voxel_array) => Some(device.create_bind_group_layout(
                &wgpu::BindGroupLayoutDescriptor {
                    label: Some("Bind Group Layout"),
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                },
            )),
            _ => None,
        };

        let bind_group = match &kind {
            BufferType::Uniform(_) | BufferType::VoxelStorage(_) => {
                Some(device.create_bind_group(&wgpu::BindGroupDescriptor {
                    layout: bind_group_layout.as_ref().unwrap(),
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: buffer.as_entire_binding(),
                    }],
                    label: Some("Bind Group"),
                }))
            }
            _ => None,
        };

        Self {
            kind,
            buffer,
            bind_group_layout,
            bind_group,
        }
    }
}
