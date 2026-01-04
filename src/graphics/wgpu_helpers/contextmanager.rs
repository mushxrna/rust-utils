use crate::graphics::wgpu_helpers::common::*;
use pollster::block_on;
use std::sync::Arc;

pub struct WgpuContextManager {
    device: Device,
    queue: Queue,
    config: SurfaceConfiguration,
    surface: Surface<'static>,
}

impl WgpuContextManager {
    //----------------------------------------------------------- private utility
    fn primary_backend_instance() -> Instance {
        Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        })
    }

    async fn highperformance_adapter(
        instance: &Instance,
        surface: Option<&Surface<'static>>,
    ) -> Result<Adapter, WpguContextError> {
        Ok(instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: surface,
                force_fallback_adapter: false,
            })
            .await?)
    }

    fn configure_surface(
        surface: &Surface,
        adapter: &Adapter,
        size: (u32, u32),
    ) -> SurfaceConfiguration {
        let surface_caps = surface.get_capabilities(adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.0,
            height: size.1,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        }
    }

    async fn default_device(adapter: &Adapter) -> Result<(Device, Queue), WpguContextError> {
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits {
                    ..Default::default()
                },
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
            })
            .await?;

        Ok((device, queue))
    }
    //----------------------------------------------------------- accessors
    pub fn device(&self) -> &Device {
        &self.device
    }

    pub fn queue(&self) -> &Queue {
        &self.queue
    }

    pub fn surface(&self) -> &Surface {
        &self.surface
    }

    pub fn format(&self) -> &TextureFormat {
        &self.config.format
    }
    //----------------------------------------------------------- various constructors
    pub async fn new_with_winit(
        window: Arc<winit::window::Window>,
    ) -> Result<Self, WpguContextError> {
        let size = (window.inner_size().width, window.inner_size().height);

        let instance = Self::primary_backend_instance();
        let surface = instance.create_surface(window)?;
        let adapter = Self::highperformance_adapter(&instance, Some(&surface)).await?;
        let (device, queue) = Self::default_device(&adapter).await?;
        let config = Self::configure_surface(&surface, &adapter, size);

        surface.configure(&device, &config);
        Ok(Self {
            device,
            queue,
            config,
            surface,
        })
    }

    pub fn blocking_with_winit(
        window: Arc<winit::window::Window>,
    ) -> Result<Self, WpguContextError> {
        block_on(Self::new_with_winit(window))
    }
}
