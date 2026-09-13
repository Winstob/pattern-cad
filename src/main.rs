mod app;

use tokio::runtime::Runtime;
use winit::event_loop::{EventLoop, ControlFlow};
use app::App;

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
