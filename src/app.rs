use std::sync::Arc;

use tokio::runtime::Runtime;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

use crate::{renderer::Renderer, app_ui::AppUi};

pub struct App {
    runtime: Runtime,
    renderer: Option<Renderer>,
    app_ui: AppUi,
}

impl App {
    pub fn new(runtime: Runtime) -> Self {
        Self {
            runtime,
            renderer: None,
            app_ui: AppUi::new(),
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.renderer.is_some() {
            return;
        }

        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes().with_transparent(true))
                .expect("Failed to create window"),
        );

        let renderer = self.runtime.block_on(Renderer::new(window));

        self.renderer = Some(renderer);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(renderer) = self.renderer.as_mut() else {
            return;
        };

        if window_id != renderer.window().id() {
            return;
        }

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }

            WindowEvent::Resized(size) => {
                renderer.resize(size.width, size.height);
            }

            WindowEvent::RedrawRequested => {
                renderer.render(&self.app_ui.ui);
            }

            _ => {}
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(renderer) = self.renderer.as_ref() {
            renderer.request_redraw();
        }
    }
}
