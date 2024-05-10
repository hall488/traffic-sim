use crate::config::{WIDTH, HEIGHT};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};
use crate::simulation::Simulation;

#[derive(Default)]
pub struct App {
    window: Option<Window>,
    simulation: Option<Simulation>,
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
   }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {

                if let Some(simulation) = self.simulation.as_mut() {
                    simulation.update();
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
