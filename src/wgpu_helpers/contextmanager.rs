use crate::vectors::Vec2;

pub struct WgpuContextManager {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: Option<wgpu::SurfaceConfiguration>,
    pub surface: Option<wgpu::Surface<'static>>,
    pub surface_format: Option<wgpu::TextureFormat>,
    pub configured: bool,
}

impl WgpuContextManager {
    pub async fn new(
        size: Vec2<u32>,
        window: Option<std::sync::Arc<winit::window::Window>>,
    ) -> Self {
        let instance = {
            wgpu::Instance::new(&wgpu::InstanceDescriptor {
                backends: wgpu::Backends::VULKAN,
                ..Default::default()
            })
        };

        let surface = match window {
            Some(_window) => Some(instance.create_surface(_window).unwrap()),
            None => None,
        };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: match &surface {
                    Some(surface) => Some(&surface),
                    None => None,
                },
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

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
            .await
            .unwrap();

        let mut surface_format = None;

        let config = match &surface {
            Some(_surface) => {
                let surface_caps = surface.as_ref().unwrap().get_capabilities(&adapter);
                surface_format = Some(
                    surface_caps
                        .formats
                        .iter()
                        .find(|f| f.is_srgb())
                        .copied()
                        .unwrap_or(surface_caps.formats[0]),
                );

                let config = wgpu::SurfaceConfiguration {
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                    format: surface_format.unwrap(),
                    width: size.x,
                    height: size.y,
                    present_mode: surface_caps.present_modes[0],
                    alpha_mode: surface_caps.alpha_modes[0],
                    view_formats: vec![],
                    desired_maximum_frame_latency: 2,
                };
                Some(config)
            }
            None => None,
        };
        println!("context created.");
        Self {
            device,
            queue,
            config,
            surface,
            surface_format,
            configured: false,
        }
    }
}
