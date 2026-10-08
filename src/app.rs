use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

use crate::{
    renderer::Renderer,
    ui::Ui,
};

pub struct App {
    window: Option<Arc<Window>>,
    renderer: Option<Renderer>,
    ui: Ui,
}

impl App {
    pub fn new() -> Self {
        Self {
            window: None,
            renderer: None,
            ui: Ui::new(),
        }
    }

    fn render(&mut self) {
        let Some(window) = &self.window else {
            return;
        };

        let Some(renderer) = &mut self.renderer else {
            return;
        };
        
        renderer.render(
            window,
            &mut self.ui,
        );
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap()
        );

        let renderer = Renderer::new(
            window.clone(),
        );

        self.window = Some(window.clone());
        self.renderer = Some(renderer);

        window.request_redraw();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: WindowId,
        event: WindowEvent,
    ) {
        let Some(window) = self.window.clone() else {
            return;
        };

        if window.id() != window_id {
            return;
        }

        match event {
            WindowEvent::CloseRequested => {
                self.renderer = None; // for some reason we get a segfault here
                                      // on wayland if we don't explicitly
                                      // drop this
                event_loop.exit();
                return;
            }

            WindowEvent::RedrawRequested => {
                self.render();
            }

            WindowEvent::Resized(size) => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.resize(size);
                }
                window.request_redraw();
            }

            _ => {}
        }

        if let Some(renderer) = &mut self.renderer {
            renderer.handle_window_event(&window, &event);
        }
    }
}
