use std::sync::Arc;

use winit::{
    dpi::PhysicalSize,
    event::WindowEvent,
    window::Window,
};

use crate::ui::Ui;

pub struct Renderer {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface_config: wgpu::SurfaceConfiguration,

    egui_ctx: egui::Context,
    egui_state: egui_winit::State,
    egui_renderer: egui_wgpu::Renderer,
}

impl Renderer {
    pub fn new(
        window: Arc<Window>,
    ) -> Self {
        let instance = wgpu::Instance::default();

        let surface = instance
            .create_surface(window.clone())
            .unwrap();

        let adapter = pollster::block_on(
            instance.request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
        ).unwrap();

        let (device, queue) = pollster::block_on(
            adapter.request_device(&wgpu::DeviceDescriptor::default())
        ).unwrap();

        let size = window.inner_size();

        let surface_config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();

        surface.configure(&device, &surface_config);

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
            &device,
            surface_config.format,
            egui_wgpu::RendererOptions::default(),
        );

        Self {
            surface,
            device,
            queue,
            surface_config,
            egui_ctx,
            egui_state,
            egui_renderer,
        }
    }

    pub fn handle_window_event(
        &mut self,
        window: &Window,
        event: &WindowEvent,
    ) {
        self.egui_state.on_window_event(window, event);
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        if size.width == 0 || size.height == 0 {
            return;
        }

        self.surface_config.width = size.width;
        self.surface_config.height = size.height;

        self.surface.configure(&self.device,
            &self.surface_config,
        );
    }

    pub fn render(
        &mut self,
        window: &Window,
        ui: &mut Ui,
    ) {
        let raw_input = self.egui_state.take_egui_input(window);

        let mut full_output = self.egui_ctx.run_ui(raw_input, |egui_ui| {
            ui.show(egui_ui);
        });

        self.egui_state
            .handle_platform_output(
                window,
                full_output.platform_output,
            );

        let paint_jobs = self.egui_ctx.tessellate(
            full_output.shapes,
            full_output.pixels_per_point,
        );

        let screen_descriptor = egui_wgpu::ScreenDescriptor {
            size_in_pixels: [
                self.surface_config.width,
                self.surface_config.height,
            ],
            pixels_per_point: full_output.pixels_per_point,
        };

        let output = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(output) => output,

            wgpu::CurrentSurfaceTexture::Suboptimal(output) => {
                self.surface.configure(
                    &self.device,
                    &self.surface_config,
                );
                output
            }

            wgpu::CurrentSurfaceTexture::Lost
            | wgpu::CurrentSurfaceTexture::Outdated => {
                self.surface.configure(
                    &self.device,
                    &self.surface_config,
                );
                return;
            }

            wgpu::CurrentSurfaceTexture::Timeout
            | wgpu::CurrentSurfaceTexture::Occluded
            |wgpu::CurrentSurfaceTexture::Validation => {
                return;
            }
        };

        let view = output.texture.create_view(
            &wgpu::TextureViewDescriptor::default(),
        );

        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("egui encoder"),
            },
        );

        for (id, image_deltas) in full_output.textures_delta.set.drain() {
            for image_delta in image_deltas {
                self.egui_renderer.update_texture(
                    &self.device,
                    &self.queue,
                    id,
                    &image_delta,
                );
            }
        }

        let user_cmd_bufs = self.egui_renderer.update_buffers(
            &self.device,
            &self.queue,
            &mut encoder,
            &paint_jobs,
            &screen_descriptor,
        );

        {
            let render_pass = encoder.begin_render_pass(
                &wgpu::RenderPassDescriptor {
                    label: Some("egui render pass"),
                    color_attachments: &[Some(
                        wgpu::RenderPassColorAttachment {
                            view: &view,
                            depth_slice: None,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(
                                          wgpu::Color::BLACK,
                                      ),
                                      store: wgpu::StoreOp::Store,
                            },
                        },
                    )],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                    multiview_mask: None,
                },
            );

            self.egui_renderer.render(
                &mut render_pass.forget_lifetime(),
                &paint_jobs,
                &screen_descriptor,
            );
        }

        self.queue.submit(
            user_cmd_bufs
            .into_iter()
            .chain(std::iter::once(encoder.finish())),
        );

        self.queue.present(output);

        for id in &full_output.textures_delta.free {
            self.egui_renderer.free_texture(id);
        }
    }
}
