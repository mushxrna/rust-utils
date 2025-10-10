use std::error::Error;

use wgpu::{TexelCopyBufferInfo, wgt::CommandEncoderDescriptor};

use crate::{
    vectors::Vec2,
    wgpu_helpers::{BufferManager, BufferType, WgpuContextManager},
};

#[derive(Clone, Copy, Debug)]
pub enum TextureAccessMode {
    ReadOnly,
    WriteOnly,
    ReadWrite,
}

#[derive(Clone)]
pub struct TextureManager {
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: wgpu::BindGroup,
    pub texture_view: wgpu::TextureView,
    pub texture_sampler: wgpu::Sampler,
    pub size: Vec2<u32>,
    pub texture: wgpu::Texture,
    pub access_mode: TextureAccessMode,
}

impl TextureManager {
    pub fn write_only(&self, device: &wgpu::Device) -> (wgpu::BindGroupLayout, wgpu::BindGroup) {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::StorageTexture {
                        access: wgpu::StorageTextureAccess::WriteOnly,
                        format: wgpu::TextureFormat::Rgba8Unorm,
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                },
            ],
            label: Some("Write-Only Storage Texture Bind Group Layout"),
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&self.texture_view),
                },
            ],
            label: Some("Write-Only Storage Texture Bind Group"),
        });

        (bind_group_layout, bind_group)
    }

    pub fn new(size: Vec2<u32>, context: &WgpuContextManager, access_mode: TextureAccessMode) -> Self {
        let texture = context.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Compute Output Texture"),
            size: wgpu::Extent3d {
                width: size.x,
                height: size.y,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::STORAGE_BINDING
                | wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let texture_sampler = context.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        // Create bind group layout based on access mode
        let (bind_group_layout, bind_group) = match access_mode {
            TextureAccessMode::WriteOnly => {
                let layout = context.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::StorageTexture {
                                access: wgpu::StorageTextureAccess::WriteOnly,
                                format: wgpu::TextureFormat::Rgba8Unorm,
                                view_dimension: wgpu::TextureViewDimension::D2,
                            },
                            count: None,
                        },
                    ],
                    label: Some("Write-Only Storage Texture Bind Group Layout"),
                });

                let group = context.device.create_bind_group(&wgpu::BindGroupDescriptor {
                    layout: &layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::TextureView(&texture_view),
                        },
                    ],
                    label: Some("Write-Only Storage Texture Bind Group"),
                });

                (layout, group)
            },
            TextureAccessMode::ReadOnly => {
                let layout = context.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::FRAGMENT | wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::FRAGMENT | wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Texture {
                                multisampled: false,
                                view_dimension: wgpu::TextureViewDimension::D2,
                                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            },
                            count: None,
                        },
                    ],
                    label: Some("Read-Only Texture Bind Group Layout"),
                });

                let group = context.device.create_bind_group(&wgpu::BindGroupDescriptor {
                    layout: &layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::Sampler(&texture_sampler),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::TextureView(&texture_view),
                        },
                    ],
                    label: Some("Read-Only Texture Bind Group"),
                });

                (layout, group)
            },
            TextureAccessMode::ReadWrite => {
                let layout = context.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::FRAGMENT | wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::FRAGMENT | wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::Texture {
                                multisampled: false,
                                view_dimension: wgpu::TextureViewDimension::D2,
                                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 2,
                            visibility: wgpu::ShaderStages::COMPUTE,
                            ty: wgpu::BindingType::StorageTexture {
                                access: wgpu::StorageTextureAccess::WriteOnly,
                                format: wgpu::TextureFormat::Rgba8Unorm,
                                view_dimension: wgpu::TextureViewDimension::D2,
                            },
                            count: None,
                        },
                    ],
                    label: Some("Read-Write Bind Group Layout"),
                });

                let group = context.device.create_bind_group(&wgpu::BindGroupDescriptor {
                    layout: &layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::Sampler(&texture_sampler),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::TextureView(&texture_view),
                        },
                        wgpu::BindGroupEntry {
                            binding: 2,
                            resource: wgpu::BindingResource::TextureView(&texture_view),
                        },
                    ],
                    label: Some("Read-Write Bind Group"),
                });

                (layout, group)
            }
        };

        Self {
            bind_group_layout,
            bind_group,
            texture_view,
            texture_sampler,
            size,
            texture,
            access_mode,
        }
    }

    pub async fn save_to_image(
        &self,
        filepath: &str,
        context: &WgpuContextManager,
    ) -> Result<(), wgpu::PollError> {
        let temp_buffer = BufferManager::new(BufferType::ImageBuffer(self.size), &context.device);
        let mut encoder = context
            .device
            .create_command_encoder(&CommandEncoderDescriptor {
                label: Some("write image to cpu encoder"),
            });
        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfoBase {
                texture: (&self.texture),
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &temp_buffer.buffer,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(4 * self.size.x),
                    rows_per_image: Some(self.size.y),
                },
            },
            wgpu::Extent3d {
                width: self.size.x,
                height: self.size.y,
                depth_or_array_layers: 1,
            },
        );

        context.queue.submit(Some(encoder.finish()));

        {
            let buffer_slice = temp_buffer.buffer.slice(..);

            let (tx, rx) = futures_intrusive::channel::shared::oneshot_channel();
            buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
                tx.send(result).unwrap();
            });
            context.device.poll(wgpu::PollType::wait_indefinitely())?;
            rx.receive().await.unwrap().unwrap();

            let data = buffer_slice.get_mapped_range();

            use image::{ImageBuffer, Rgba};
            let buffer =
                ImageBuffer::<Rgba<u8>, _>::from_raw(self.size.x, self.size.y, data).unwrap();
            buffer.save("image.png").unwrap();
        }
        temp_buffer.buffer.unmap();
        Ok(())
    }
}
