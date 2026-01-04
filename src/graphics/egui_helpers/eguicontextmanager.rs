use crate::graphics::egui_helpers::common::*;

pub struct EguiContextManager {
    state: State,
    renderer: Renderer,
}

impl EguiContextManager {
    //----------------------------------------------------------- accessors
    pub fn state(&mut self) -> &mut State {
        &mut self.state
    }
    pub fn renderer(&mut self) -> &mut Renderer {
        &mut self.renderer
    }
    //----------------------------------------------------------- constructor
    pub fn new(window: &Arc<Window>, wgpu_ctx: &WgpuContextManager) -> Self {
        let ctx = Context::default();
        let state = State::new(
            ctx,
            egui::ViewportId::ROOT,
            window,
            Some(window.scale_factor() as f32),
            None,
            None,
        );

        let renderer = Renderer::new(
            wgpu_ctx.device(),
            wgpu_ctx.format(),
            egui_wgpu::RendererOptions::default(),
        );

        Self { state, renderer }
    }

    //----------------------------------------------------------- public utilities
    //----------------------------------------------------------- render pass
    pub fn render_pass<F: FnMut(&Context)>(
        &mut self,
        wgpu_ctx: &WgpuContextManager,
        window: Arc<Window>,
        f: F,
    ) {
        let input = self.state.take_egui_input(&window);
        let full_output = self.state.egui_ctx().run(input, f);

        self.state
            .handle_platform_output(&window, full_output.platform_output);

        let tris = self
            .state
            .egui_ctx()
            .tessellate(full_output.shapes, full_output.pixels_per_point);

        for (id, image_delta) in &full_output.textures_delta.set {
            self.renderer
                .update_texture(wgpu_ctx.device(), wgpu_ctx.queue(), *id, image_delta);
        }

        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [wgpu_ctx.size().0, wgpu_ctx.size().1],
            pixels_per_point: window.scale_factor() as f32,
        };

        let output = wgpu_ctx.surface().get_current_texture().unwrap();
        let view = output.texture.create_view(&Default::default());

        let mut encoder = wgpu_ctx
            .device()
            .create_command_encoder(&Default::default());

        let new_cmd_bufs = self.renderer.update_buffers(
            wgpu_ctx.device(),
            wgpu_ctx.queue(),
            &mut encoder,
            &tris,
            &screen_descriptor,
        );

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("egui render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                ..Default::default()
            });

            self.renderer.render(
                &mut render_pass.forget_lifetime(),
                &tris,
                &screen_descriptor,
            );
        }

        wgpu_ctx.queue().submit(
            new_cmd_bufs
                .into_iter()
                .chain(std::iter::once(encoder.finish())),
        );

        output.present();

        for id in &full_output.textures_delta.free {
            self.renderer.free_texture(id);
        }
    }
}
