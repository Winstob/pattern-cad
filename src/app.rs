use std::sync::Arc;

use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

use crate::cad::CadEngine;
use crate::commands::UiCommand;
use crate::renderer::Renderer;
use crate::ui::{Ui, UiContext};

pub struct App {
    window: Option<Arc<Window>>,
    app_ui: Ui,
    cad_engine: CadEngine,
    renderer: Option<Renderer>,
}

impl App {
    pub fn new() -> Self {
        Self {
            window: None,
            app_ui: Ui::new(),
            cad_engine: CadEngine::new(),
            renderer: None,
        }
    }

    fn render(&mut self) {
        for command in self.app_ui.take_pending_commands() {
            self.handle_ui_command(command);
        }

        let Some(window) = &self.window else {
            return;
        };

        let Some(renderer) = &mut self.renderer else {
            return;
        };

        let ui_context = UiContext {
            model_available: self.cad_engine.model_loaded(),
        };

        renderer.prepare_ui(window, &mut self.app_ui, &ui_context);
        //println!("{:?}", self.app_ui.cad_viewport_input.pointer_pos);
        self.cad_engine
            .handle_input(&mut self.app_ui.cad_viewport_input);

        renderer.render(window);
    }

    fn handle_ui_command(&mut self, cmd: UiCommand) {
        self.cad_engine.handle_ui_command(cmd);
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );

        let renderer = Renderer::new(window.clone());

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
