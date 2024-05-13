use crate::config::{WIDTH, HEIGHT};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};
use crate::simulation::Simulation;
use std::time::{Duration, Instant};

#[derive(Default)]
pub struct App {
    window: Option<Window>,
    simulation: Option<Simulation>,
    last_redraw: Option<Instant>,
}

impl ApplicationHandler for App {

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes()
            .with_title("Fantastic window number one!")
            .with_inner_size(winit::dpi::LogicalSize::new(WIDTH, HEIGHT));
        let window = event_loop.create_window(window_attributes).unwrap();

        let simulation = Simulation::new(&window);

        self.simulation = Some(simulation);
        self.window = Some(window);
        self.last_redraw = Some(Instant::now());
   }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {

                if let (Some(simulation), Some(last_redraw)) = (&mut self.simulation, &mut self.last_redraw) {
                    let now = Instant::now();
                    let dt = now.duration_since(*last_redraw);
                    *last_redraw = now;

                    simulation.update(dt);
                    simulation.draw(event_loop);
                }

                if let Some(window) = self.window.as_ref() {
                    window.request_redraw();
                }
            }
            _ => (),
        }
    }
}
