mod gui_renderer;
use gui_renderer::GuiRenderer;
mod cad_renderer;
use crate::ui::{Ui, UiContext};
use cad_renderer::CadRenderer;

use std::sync::Arc;

use winit::{dpi::PhysicalSize, event::WindowEvent, window::Window};

pub struct Renderer {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface_config: wgpu::SurfaceConfiguration,

    cad_renderer: CadRenderer,
    gui_renderer: GuiRenderer,
}

impl Renderer {
    pub fn new(window: Arc<Window>) -> Self {
        let instance = wgpu::Instance::default();

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        }))
        .unwrap();

        let (device, queue) =
            pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default())).unwrap();

        let size = window.inner_size();

        let surface_config = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();

        surface.configure(&device, &surface_config);

        let cad_renderer = CadRenderer::new(&device, surface_config.format);

        let gui_renderer = GuiRenderer::new(window.clone(), &device, &surface_config);

        Self {
            surface,
            device,
            queue,
            surface_config,
            cad_renderer,
            gui_renderer,
        }
    }

    pub fn handle_window_event(&mut self, window: &Window, event: &WindowEvent) {
        if self.gui_renderer.handle_window_event(window, event) {
            window.request_redraw();
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        if size.width == 0 || size.height == 0 {
            return;
        }

        self.surface_config.width = size.width;
        self.surface_config.height = size.height;

        self.surface.configure(&self.device, &self.surface_config);
    }

    pub fn prepare_ui(&mut self, window: &Window, app_ui: &mut Ui, ui_context: &UiContext) {
        self.gui_renderer.prepare(window, app_ui, ui_context);
    }

    pub fn render(&mut self, window: &Window, app_ui: &mut Ui, ui_context: &UiContext) {
        let output = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(output) => output,

            wgpu::CurrentSurfaceTexture::Suboptimal(output) => {
                self.surface.configure(&self.device, &self.surface_config);
                output
            }

            wgpu::CurrentSurfaceTexture::Lost | wgpu::CurrentSurfaceTexture::Outdated => {
                self.surface.configure(&self.device, &self.surface_config);
                return;
            }

            wgpu::CurrentSurfaceTexture::Timeout
            | wgpu::CurrentSurfaceTexture::Occluded
            | wgpu::CurrentSurfaceTexture::Validation => {
                return;
            }
        };

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("egui encoder"),
            });

        self.cad_renderer.render(
            &mut encoder,
            &view,
            self.gui_renderer.cad_viewport_rect.unwrap(),
        );

        self.gui_renderer
            .render(&self.device, &self.queue, window, &mut encoder, &view);

        self.queue.submit([encoder.finish()]);

        self.queue.present(output);
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ViewportRect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}
