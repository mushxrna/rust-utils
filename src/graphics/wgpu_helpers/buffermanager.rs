use crate::graphics::wgpu_helpers::config_enums::*;
use wgpu::{Buffer, util::DeviceExt};

pub struct BufferManager {
    pub kind: BufferType,
    label: String,
    pub buffer: wgpu::Buffer,
    pub bind_group_layout: Option<wgpu::BindGroupLayout>,
    pub bind_group: Option<wgpu::BindGroup>,
}

impl BufferManager {
    pub fn new(kind: BufferType, device: &wgpu::Device, label: String) -> Self {
        let buffer = match &kind {
            BufferType::Vertex(vertices) => {
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(("vertex buffer: ".to_owned() + &label).as_str()),
                    usage: wgpu::BufferUsages::VERTEX,
                    contents: bytemuck::cast_slice(vertices),
                })
            }
            BufferType::Index(indices) => {
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(("index buffer: ".to_owned() + &label).as_str()),
                    usage: wgpu::BufferUsages::INDEX,
                    contents: bytemuck::cast_slice(indices),
                })
            }
            BufferType::Uniform(uniform_array) => {
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(("uniform buffer: ".to_owned() + &label).as_str()),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                    contents: bytemuck::cast_slice(&[*uniform_array]),
                })
            }
            BufferType::Storage(bytes, mode) => {
                device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(("hitinfo buffer: ".to_owned() + &label).as_str()),
                    usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
                    contents: bytemuck::cast_slice(bytes.as_slice()),
                })
            }
            BufferType::ImageBuffer(size) => {
                let buffer_size = (4 * size.x * size.y) as wgpu::BufferAddress;
                device.create_buffer(&wgpu::BufferDescriptor {
                    label: Some(("image buffer: ".to_owned() + &label).as_str()),
                    size: buffer_size,
                    usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
                    mapped_at_creation: false,
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
            BufferType::Storage(bytes, mode) => Some(device.create_bind_group_layout(
                &wgpu::BindGroupLayoutDescriptor {
                    label: Some("Bind Group Layout"),
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::COMPUTE,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage {
                                read_only: mode.into_buffer_mode(),
                            },
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
            BufferType::Uniform(_) | BufferType::Storage(_, _) => {
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
            label,
        }
    }
}
