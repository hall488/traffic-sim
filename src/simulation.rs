use winit::window::Window;
use crate::config::{WIDTH, HEIGHT};
use pixels::{Pixels, SurfaceTexture};
use winit::event_loop::{ActiveEventLoop, ControlFlow};
use crate::vehicle::Vehicle;

pub struct Simulation {
    pixels: Pixels,
    vehicles: Vec<Vehicle>
}


impl Simulation {

    pub fn new(window: &Window) -> Self {
        let pixels = {
            let window_size = window.inner_size();
            let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(WIDTH, HEIGHT, surface_texture)
        };

        Self {
            pixels : pixels.expect("_"),
            vehicles : Vec::new(),
        }

    }


    pub fn update(&mut self) {

        for vehicle in &self.vehicles {
            vehicle.update();
        }

    }

    pub fn draw(&mut self, event_loop: &ActiveEventLoop) {

        let frame = self.pixels.frame_mut();

        for pixel in frame.chunks_exact_mut(4) {
            pixel.copy_from_slice(&[0x48, 0xb2, 0xe8, 0xff]);
        }

        for vehicle in &self.vehicles {
            vehicle.draw();
        }

        if let Err(err) = self.pixels.render() {
            println!("Error during rendering: {:?}", err);
            event_loop.set_control_flow(ControlFlow::Wait);
            return;
        }


    }

}
