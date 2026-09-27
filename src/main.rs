#![allow(dead_code)]
#![allow(unused)]
mod app;
mod renderer;
mod ui;
mod app_ui;

use app::App;
use tokio::runtime::Runtime;
use ui::Ui;
use winit::event_loop::{ControlFlow, EventLoop};

fn run() {
    env_logger::init();

    let runtime = Runtime::new().expect("Failed to create Tokio runtime");

    let event_loop = EventLoop::new().expect("Failed to create event loop");

    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = App::new(runtime);

    event_loop
        .run_app(&mut app)
        .expect("Event loop terminated with an error");
}

fn main() {
    run();
}
