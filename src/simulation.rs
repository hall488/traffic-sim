use winit::window::Window;
use crate::world::World;
use crate::config::{WIDTH, HEIGHT};
use pixels::{Pixels, SurfaceTexture};
use winit::event_loop::{ActiveEventLoop, ControlFlow};

pub struct Simulation {
    world: World,
    pixels: Pixels,
}


impl Simulation {

    pub fn new(window: &Window) -> Self {
        let pixels = {
            let window_size = window.inner_size();
            let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(WIDTH, HEIGHT, surface_texture)
        };
        let world = World::new();

        Self {
            world,
            pixels : pixels.expect("_")
        }

    }


    pub fn update(&mut self) {

        self.world.update();

    }

    pub fn draw(&mut self, event_loop: &ActiveEventLoop) {

        self.world.draw(self.pixels.frame_mut());

        if let Err(err) = self.pixels.render() {
            println!("Error during rendering: {:?}", err);
            event_loop.set_control_flow(ControlFlow::Wait);
            return;
        }


    }

}
