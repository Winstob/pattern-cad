use std::sync::Arc;

use wgpu::CurrentSurfaceTexture;
use winit::window::Window;

use crate::ui::Ui;

pub struct Renderer {
    window: Arc<Window>,

    instance: wgpu::Instance,
    surface: wgpu::Surface<'static>,

    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,

    config: wgpu::SurfaceConfiguration,
}

impl Renderer {
    pub async fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::default();

        let surface = instance
            .create_surface(window.clone())
            .expect("Failed to create surface");

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
                apply_limit_buckets: false,
            })
            .await
            .expect("Failed to find a suitable GPU adapter");

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                memory_hints: wgpu::MemoryHints::default(),
                trace: wgpu::Trace::Off,
            })
            .await
            .expect("Failed to create GPU device");

        let mut config = surface
            .get_default_config(&adapter, size.width, size.height)
            .expect("Surface is not supported by this adapter");
        config.alpha_mode = surface
            .get_capabilities(&adapter)
            .alpha_modes
            .contains(&wgpu::CompositeAlphaMode::PreMultiplied)
            .then_some(wgpu::CompositeAlphaMode::PreMultiplied)
            .unwrap_or(wgpu::CompositeAlphaMode::Opaque);

        surface.configure(&device, &config);

        Self {
            window,
            instance,
            surface,
            adapter,
            device,
            queue,
            config,
        }
    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        if width == 0 || height == 0 {
            return;
        }

        self.config.width = width;
        self.config.height = height;

        self.surface.configure(&self.device, &self.config);
    }

    pub fn render(&mut self, ui: &Ui) {
        let output = match self.surface.get_current_texture() {
            CurrentSurfaceTexture::Success(output) => output,

            CurrentSurfaceTexture::Suboptimal(output) => output,

            CurrentSurfaceTexture::Timeout => return,

            CurrentSurfaceTexture::Occluded => return,

            CurrentSurfaceTexture::Outdated => {
                self.surface.configure(&self.device, &self.config);

                return;
            }

            CurrentSurfaceTexture::Lost => {
                self.surface.configure(&self.device, &self.config);

                return;
            }

            CurrentSurfaceTexture::Validation => return,
        };

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Frame Encoder"),
            });

        self.render_ui(&mut encoder, &view, ui);

        self.queue.submit(std::iter::once(encoder.finish()));

        self.queue.present(output);
    }

    fn render_ui(&self, encoder: &mut wgpu::CommandEncoder, target: &wgpu::TextureView, ui: &Ui) {
        let layout = ui
            .layout
            .organize(self.config.width as f32, self.config.height as f32);

        // Eventually:
        //
        // for widget in layout.widgets() {
        //     match widget {
        //         Widget::Button(...) => ...
        //         Widget::Label(...) => ...
        //         Widget::Viewport(...) => ...
        //     }
        // }
        //
        // For now, just clear the screen.

        let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("UI Render Pass"),

            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target,
                depth_slice: None,
                resolve_target: None,

                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                        a: 0.0,
                    }),

                    store: wgpu::StoreOp::Store,
                },
            })],

            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
    }
}
