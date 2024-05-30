use crate::config::{WIDTH, HEIGHT};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};
use crate::simulation::Simulation;
use std::time::{Instant, Duration};

#[derive(Default)]
pub struct App {
    window: Option<Window>,
    pub simulation: Option<Simulation>,
    last_redraw: Option<Instant>,
    frame_count: usize,
    last_fps_check: Option<Instant>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes()
            .with_title("Fantastic window number one!")
            .with_inner_size(winit::dpi::LogicalSize::new(WIDTH, HEIGHT));
        let window = event_loop.create_window(window_attributes).unwrap();

        let simulation = Simulation::new(Some(&window));

        self.simulation = Some(simulation);
        self.window = Some(window);
        self.last_redraw = Some(Instant::now());
        self.frame_count = 0;
        self.last_fps_check = Some(Instant::now());
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

                    self.frame_count += 1;
                    let elapsed = now.duration_since(self.last_fps_check.unwrap());
                    if elapsed >= Duration::from_secs(1) {
                        let fps = self.frame_count as f64 / elapsed.as_secs_f64();
                        println!("FPS: {:.2}", fps);
                        self.frame_count = 0;
                        self.last_fps_check = Some(now);
                    }
                }

                if let Some(window) = self.window.as_ref() {
                    window.request_redraw();
                }

                // Limit FPS to 60
                let frame_duration = Duration::from_secs_f64(1.0 / 60.0);
                let elapsed = self.last_redraw.unwrap().elapsed();
                if elapsed < frame_duration {
                    std::thread::sleep(frame_duration - elapsed);
                }
            }
            _ => (),
        }
    }
}
