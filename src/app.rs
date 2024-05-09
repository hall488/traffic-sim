use crate::world::World;
use crate::config::{WIDTH, HEIGHT};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow};
use winit::window::{Window, WindowId};
use pixels::{Pixels, SurfaceTexture};

#[derive(Default)]
pub struct App {
    window: Option<Window>,
    world: Option<World>,
    pixels: Option<Pixels>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes()
            .with_title("Fantastic window number one!")
            .with_inner_size(winit::dpi::LogicalSize::new(WIDTH, HEIGHT));
        let window = event_loop.create_window(window_attributes).unwrap();
        let pixels = {
            let window_size = window.inner_size();
            let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(WIDTH, HEIGHT, surface_texture)
        };
        let world = World::new();
        self.window = Some(window);
        self.world = Some(world);
        self.pixels = Some(pixels.expect("Fart"));
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {

                if let (Some(world), Some(pixels)) = (self.world.as_mut(), self.pixels.as_mut()) {

                    world.draw(pixels.frame_mut());
                    world.update();

                    if let Err(err) = pixels.render() {
                        println!("Error during rendering: {:?}", err);
                        event_loop.set_control_flow(ControlFlow::Wait);
                        return;
                    }

                    if let Some(window) = self.window.as_ref() {
                        window.request_redraw();
                    }
                }
            }
            _ => (),
        }
    }
}
