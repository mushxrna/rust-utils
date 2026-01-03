use crate::graphics::wgpu_helpers::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WpguContextError {
    #[error("Couldn't create surface.")]
    SurfaceCreationError(#[from] wgpu::CreateSurfaceError),
    #[error("Couldn't create adapter.")]
    AdapterCreationError(#[from] wgpu::RequestAdapterError),
    #[error("Couldn't create device.")]
    DeviceCreationError(#[from] wgpu::RequestDeviceError),
}
