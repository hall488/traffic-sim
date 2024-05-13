use winit::window::Window;
use crate::config::{WIDTH, HEIGHT, DASH_LENGTH, GAP_LENGTH};
use pixels::{Pixels, SurfaceTexture};
use winit::event_loop::{ActiveEventLoop, ControlFlow};
use crate::vehicle::{Vehicle,Lane};
use std::time::Duration;

pub struct Simulation {
    pixels: Pixels,
    vehicles: Vec<Vehicle>,
    window_width: u32,
    window_height: u32,
}


impl Simulation {

    pub fn new(window: &Window) -> Self {

        let window_size = window.inner_size();
        let pixels = {
            let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(WIDTH, HEIGHT, surface_texture)
        };

        let mut vehicles: Vec<Vehicle> = Vec::new();

        let my_vehicle = Vehicle::new(50, 10, 10, 0.0, HEIGHT as f64 /2.0 + 12.5, 0.0, Lane::Left);
        let my_vehicle_2 = Vehicle::new(50, 10, 10, 0.0, HEIGHT as f64 /2.0 + 37.5, 0.0, Lane::Right);

        // Append the vehicle to the vector
        vehicles.push(my_vehicle);
        vehicles.push(my_vehicle_2);

        Self {
            pixels : pixels.expect("_"),
            vehicles,
            window_width: window_size.width,
            window_height: window_size.height,
        }

    }


    pub fn update(&mut self, dt: Duration ) {

        for vehicle in &mut self.vehicles {
            vehicle.update(dt);
        }

        self.vehicles.retain(|vehicle| !vehicle.check_bounds())

    }

    pub fn draw(&mut self, event_loop: &ActiveEventLoop) {

        let frame = self.pixels.frame_mut();

        for (index, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let mut color =  &[0x48, 0xb2, 0xe8, 0xff];

            let i = index % WIDTH as usize; // Current pixel's x-coordinate
            let j = index / WIDTH as usize;

            if  i < WIDTH as usize / 2 + 50 &&
            i > WIDTH as usize / 2 - 50 ||
            j < HEIGHT as usize / 2 + 50 &&
            j > HEIGHT as usize / 2 - 50 {
                color = &[0xa0, 0xa0, 0xa0, 0xff];
            }

            if !(i < WIDTH as usize / 2 + 50 &&
            i > WIDTH as usize / 2 - 50 &&
            j < HEIGHT as usize / 2 + 50 &&
            j > HEIGHT as usize / 2 - 50 ) {

                if (i < WIDTH as usize / 2 + 5 &&
                i > WIDTH as usize / 2 - 5) ^
                (j < HEIGHT as usize / 2 + 5 &&
                j > HEIGHT as usize / 2 - 5) {
                    color = &[0xff, 0xff, 0x00, 0xff];
                } else if (i == WIDTH as usize / 2 - 25 || i == WIDTH as usize / 2 + 25) && // Positions for lane dividers
                (j / (DASH_LENGTH + GAP_LENGTH) % 2 == 0) { // Dashed pattern
                    color = &[0xff, 0xff, 0x00, 0xff]; // Yellow dashes
                } else if (j == HEIGHT as usize / 2 - 25 || j == HEIGHT as usize / 2 + 25) && // Positions for lane dividers
                (i / (DASH_LENGTH + GAP_LENGTH) % 2 == 0) { // Dashed pattern
                    color = &[0xff, 0xff, 0x00, 0xff]; // Yellow dashes
                }

            }

            pixel.copy_from_slice(color);
        }

        for vehicle in &self.vehicles {
            vehicle.draw(frame, self.window_width, self.window_height);
        }

        if let Err(err) = self.pixels.render() {
            println!("Error during rendering: {:?}", err);
            event_loop.set_control_flow(ControlFlow::Wait);
            return;
        }


    }

}
