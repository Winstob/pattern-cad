use crate::ui::{Ui, UiContext};

use std::any::type_name;
use std::sync::Arc;
use winit::{event::WindowEvent, window::Window};

use super::ViewportRect;

pub struct GuiRenderer {
    egui_ctx: egui::Context,
    egui_state: egui_winit::State,
    egui_renderer: egui_wgpu::Renderer,

    curr_frame_output: Option<egui::FullOutput>,

    pub cad_draw_viewport: Option<ViewportRect>,
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
            curr_frame_output: None,
            cad_draw_viewport: None,
        }
    }

    pub fn prepare(&mut self, window: &Window, app_ui: &mut Ui, ui_context: &UiContext) {
        let raw_input = self.egui_state.take_egui_input(window);

        self.curr_frame_output = Some(self.egui_ctx.run_ui(raw_input, |egui_ui| {
            app_ui.show(egui_ui, ui_context);
        }));
        self.cad_draw_viewport = Some(self.get_cad_draw_viewport_rect(app_ui));
    }

    pub fn render(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        window: &Window,
        encoder: &mut wgpu::CommandEncoder,
        target: &wgpu::TextureView,
    ) {
        let mut full_output = match self.curr_frame_output.take() {
            Some(output) => output,
            None => panic!(
                "{}: must call prepare() before render()!",
                type_name::<Self>()
            ),
        };

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
                        //load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        load: wgpu::LoadOp::Load,
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

    fn get_cad_draw_viewport_rect(&self, app_ui: &Ui) -> ViewportRect {
        let full_output = match self.curr_frame_output.as_ref() {
            Some(output) => output,
            None => panic!(
                "{}: must call cad_draw_viewport_rect() between prepare() and render()!",
                type_name::<Self>()
            ),
        };

        let raw_rect = app_ui.cad_draw_viewport.as_ref().unwrap();
        let pixels_per_point = full_output.pixels_per_point;

        let min_x = (raw_rect.min.x * pixels_per_point).round() as u32;
        let min_y = (raw_rect.min.y * pixels_per_point).round() as u32;
        let max_x = (raw_rect.max.x * pixels_per_point).round() as u32;
        let max_y = (raw_rect.max.y * pixels_per_point).round() as u32;
        ViewportRect {
            x: min_x,
            y: min_y,
            width: max_x.saturating_sub(min_x),
            height: max_y.saturating_sub(min_y),
        }
    }
}
