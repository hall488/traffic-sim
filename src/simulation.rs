use rand::Rng;
use winit::window::Window;
use crate::{collision::rectangles_intersect, config::{DASH_LENGTH, GAP_LENGTH, HEIGHT, WIDTH}, intersection_manager, vehicle::TurnDirection};
use pixels::{Pixels, SurfaceTexture};
use winit::event_loop::{ActiveEventLoop, ControlFlow};
use crate::vehicle::Vehicle;
use std::time::{Duration, Instant};
use crate::stop_light::StopLight;
use crate::intersection_manager::IntersectionManager;

pub struct Simulation {
    pixels: Pixels,
    vehicles: Vec<Vehicle>,
    window_width: u32,
    window_height: u32,
    background: Vec<u8>,
    time_since_last_spawn: Instant,
    id_counter: usize,
    stop_lights: [StopLight; 4],
    intersection_manager: IntersectionManager ,
    release_queue: [Vec<Vehicle>; 8],
}

impl Simulation {

    pub fn new(window: &Window) -> Self {

        let window_size = window.inner_size();
        let mut pixels = {
            let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(WIDTH, HEIGHT, surface_texture)
        }.expect("_");

        let vehicles: Vec<Vehicle> = Vec::new();

        let frame = pixels.frame_mut();
        let background = load_background_frame(frame);

        let stop_lights = [StopLight::new(0), StopLight::new(1), StopLight::new(2), StopLight::new(3)];

        let intersection_manager = IntersectionManager::new();

        Self {
            pixels,
            vehicles,
            window_width: window_size.width,
            window_height: window_size.height,
            background,
            time_since_last_spawn: Instant::now(),
            id_counter: 0,
            stop_lights,
            intersection_manager,
            release_queue: [Vec::new(), Vec::new(), Vec::new(), Vec::new(),
                            Vec::new(), Vec::new(), Vec::new(), Vec::new()]
        }

    }


    pub fn update(&mut self, dt: Duration ) {

        for queue in &mut self.release_queue {
            if !queue.is_empty() {
                // Take the first element from the queue
                let v1 = queue.swap_remove(0);
                let mut passed = true;

                // Check for intersections
                for v2 in &self.vehicles {
                    if rectangles_intersect(&v1.bounds, &v2.bounds) {
                        passed = false;
                        break; // Exit the loop early if an intersection is found
                    }
                }

                // Move the vehicle into self.vehicles if the condition is met
                if passed {
                    self.vehicles.push(v1);
                } else {
                    // If not moved, put it back in the queue
                    queue.push(v1);
                }
            }
        }


        for vehicle in &mut self.vehicles {
            vehicle.update(dt);
        }

        self.vehicles.retain(|vehicle| {
            if !vehicle.check_bounds() {
                return true
            } else {
                self.intersection_manager.intersection_volume[(vehicle.entrance/2) as usize] -= 1;
                return false
            }
        });
        self.spawn_on_timer(0.1);
        self.handle_vehicle_collisions();
        self.handle_vehicle_stops();

        if let Some((max_index, &max_value)) = self.intersection_manager.intersection_volume.iter().enumerate().max_by_key(|&(_, &val)| val) {
            self.stop_lights[max_index].queued = true;
            // Perform a different action for all other indices
            for (index, &value) in self.intersection_manager.intersection_volume.iter().enumerate() {
                if index == max_index {
                    continue;
                }
                self.stop_lights[index].active = false;
            }
        }

        for stop_light in &mut self.stop_lights {
            stop_light.update();
        }
        //stoplight coooooooooode
        //stoplight is a line that is stop or go
        //line is a rectangle that appears and disappears
        //
        //stop light manager says when each stop light can switch
    }

    pub fn spawn_on_timer(&mut self, time: f32) {

        let now = Instant::now();
        let spawn_timer = now.duration_since(self.time_since_last_spawn);

        if spawn_timer.as_secs_f32() > time {
            let mut rng = rand::thread_rng();
            let entrance = rng.gen_range(0..8);
            let vehicle = Vehicle::new(self.id_counter, 50, 10, 10, entrance);

            self.id_counter += 1;

            self.intersection_manager.intersection_volume[(entrance/2) as usize] += 1;
            self.release_queue[entrance as usize].push(vehicle);
            self.time_since_last_spawn = now;
        }

    }

    fn handle_vehicle_collisions(&mut self) {
        for i in 0..self.vehicles.len() {

            let (before, rest) = self.vehicles.split_at_mut(i);
            let (v1, after) = rest.split_at_mut(1);
            let v1 = &mut v1[0];

            for v2 in before.iter().chain(after.iter()) {
                if rectangles_intersect(&v1.vision, &v2.bounds) {
                    v1.speed = 0;
                    break; // Stop checking further once an intersection is found
                }
            }
        }
    }

    fn handle_vehicle_stops(&mut self) {
        for vehicle in &mut self.vehicles {
            for stop_light in &self.stop_lights {
                if rectangles_intersect(&vehicle.vision, &stop_light.line) && !stop_light.active {
                    vehicle.speed = 0;
                    break;
                }
            }
        }
    }

    pub fn draw(&mut self, event_loop: &ActiveEventLoop) {

        let frame = self.pixels.frame_mut();

        frame.copy_from_slice(&self.background);

        for vehicle in &self.vehicles {
            vehicle.draw(frame, self.window_width, self.window_height);
        }

        for stop_light in &self.stop_lights {
            stop_light.draw(frame, self.window_width, self.window_height);
        }

        if let Err(err) = self.pixels.render() {
            println!("Error during rendering: {:?}", err);
            event_loop.set_control_flow(ControlFlow::Wait);
            return;
        }
    }
}

fn load_background_frame(frame: &mut [u8]) -> Vec<u8>{
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

    return frame.to_vec();
}

