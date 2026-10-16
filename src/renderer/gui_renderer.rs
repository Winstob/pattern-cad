use std::sync::Arc;
use winit::{event::WindowEvent, window::Window};

use crate::ui::Ui;

pub struct GuiRenderer {
    egui_ctx: egui::Context,
    egui_state: egui_winit::State,
    egui_renderer: egui_wgpu::Renderer,
}

impl GuiRenderer {
    pub fn new(
        window: Arc<Window>,
        device: &wgpu::Device,
        surface_config: &wgpu::SurfaceConfiguration,
    ) -> Self {
        let egui_ctx = egui::Context::default();

        let egui_state = egui_winit::State::new(
            egui_ctx.clone(),
            egui::ViewportId::ROOT,
            &window,
            Some(window.scale_factor() as f32),
            window.theme(),
            None,
        );

        let egui_renderer = egui_wgpu::Renderer::new(
            device,
            surface_config.format,
            egui_wgpu::RendererOptions::default(),
        );

        GuiRenderer {
            egui_ctx: egui_ctx,
            egui_state: egui_state,
            egui_renderer: egui_renderer,
        }
    }

    pub fn render(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        window: &Window,
        encoder: &mut wgpu::CommandEncoder,
        target: &wgpu::TextureView,
        ui: &mut Ui,
    ) {
        let raw_input = self.egui_state.take_egui_input(window);

        let mut full_output = self.egui_ctx.run_ui(raw_input, |egui_ui| {
            ui.show(egui_ui);
        });

        self.egui_state
            .handle_platform_output(window, full_output.platform_output);

        let paint_jobs = self
            .egui_ctx
            .tessellate(full_output.shapes, full_output.pixels_per_point);

        let size = window.inner_size();

        let screen_descriptor = egui_wgpu::ScreenDescriptor {
            size_in_pixels: [size.width, size.height],
            pixels_per_point: full_output.pixels_per_point,
        };

        for (id, image_deltas) in full_output.textures_delta.set.drain() {
            for image_delta in image_deltas {
                self.egui_renderer
                    .update_texture(device, queue, id, &image_delta);
            }
        }

        self.egui_renderer
            .update_buffers(device, queue, encoder, &paint_jobs, &screen_descriptor);

        {
            let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("egui render pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &target,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });

            self.egui_renderer.render(
                &mut render_pass.forget_lifetime(),
                &paint_jobs,
                &screen_descriptor,
            );
        }
    }

    pub fn handle_window_event(&mut self, window: &Window, event: &WindowEvent) -> bool {
        let response = self.egui_state.on_window_event(window, event);
        response.repaint
    }
}
