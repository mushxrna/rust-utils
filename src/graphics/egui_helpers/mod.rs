mod eguicontextmanager;

pub mod common {
    pub use egui::Context;
    pub use egui_wgpu::{Renderer, ScreenDescriptor};
    pub use egui_winit::State;

    pub use winit::window::Window;

    pub use std::sync::Arc;

    pub use crate::graphics::wgpu_helpers::*;
}

pub use eguicontextmanager::EguiContextManager;
