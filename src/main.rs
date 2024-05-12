mod app;
mod config;
mod simulation;
mod vehicle;

use crate::app::App;
use winit::event_loop::{ControlFlow, EventLoop};


fn main() {

    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut app = App::default();
    let _ = event_loop.run_app(&mut app);

}

