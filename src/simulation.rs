use rand::Rng;
use winit::window::Window;
use crate::collision::rectangles_intersect;
use crate::config::{DASH_LENGTH, GAP_LENGTH, HEIGHT, WIDTH};
use pixels::{Pixels, SurfaceTexture};
use winit::event_loop::{ActiveEventLoop, ControlFlow};
use crate::vehicle::Vehicle;
use std::time::{Duration, Instant};
use crate::intersection_manager::IntersectionManager;
use crate::qlearning::QLearning;

pub struct Simulation {
    pixels: Option<Pixels>,
    vehicles: Vec<Vehicle>,
    window_width: u32,
    window_height: u32,
    background: Option<Vec<u8>>,
    time_since_last_spawn: Instant,
    id_counter: usize,
    pub intersection_manager: IntersectionManager,
    release_queue: [Vec<Vehicle>; 8],
    pub qlearning: QLearning,
    last_light_change: Instant,
    min_volume: f64,
    max_volume: f64,
}

impl Simulation {
    pub fn new(window: Option<&Window>) -> Self {
        let (pixels, background, window_width, window_height) = if let Some(window) = window {
            let window_size = window.inner_size();
            let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, window);
            let mut pixels = Pixels::new(WIDTH, HEIGHT, surface_texture)
                .expect("Failed to create pixels");
            let frame = pixels.frame_mut();
            let background = Some(load_background_frame(frame));

            (Some(pixels), background, window_size.width, window_size.height)
        } else {
            (None, None, WIDTH, HEIGHT)
        };

        let vehicles: Vec<Vehicle> = Vec::new();
        let intersection_manager = IntersectionManager::new();

        Self {
            pixels,
            vehicles,
            window_width,
            window_height,
            background,
            time_since_last_spawn: Instant::now(),
            id_counter: 0,
            intersection_manager,
            release_queue: [Vec::new(), Vec::new(), Vec::new(), Vec::new(),
                            Vec::new(), Vec::new(), Vec::new(), Vec::new()],
            qlearning: QLearning::new(9, 5),
            last_light_change: Instant::now(),
            min_volume: f64::MAX,
            max_volume: f64::MIN,
        }
    }

    pub fn update(&mut self, dt: Duration) {
        for queue in &mut self.release_queue {
            if !queue.is_empty() {
                let v1 = queue.swap_remove(0);
                let mut passed = true;

                for v2 in &self.vehicles {
                    if rectangles_intersect(&v1.bounds, &v2.bounds) {
                        passed = false;
                        break;
                    }
                }

                if passed {
                    self.vehicles.push(v1);
                } else {
                    queue.push(v1);
                }
            }
        }

        for vehicle in &mut self.vehicles {
            vehicle.update(dt);
        }

        self.vehicles.retain(|vehicle| {
            if !vehicle.check_bounds() {
                return true;
            } else {
                self.intersection_manager.intersection_volume[(vehicle.entrance / 2) as usize] -= 1;
                return false;
            }
        });
        self.spawn_on_timer(0.02);
        self.handle_vehicle_collisions();
        self.handle_vehicle_stops();

        for stop_light in &mut self.intersection_manager.stop_lights {
            stop_light.update();
        }

        if self.last_light_change.elapsed() >= Duration::from_millis(100) {
            let state = self.intersection_manager.get_state();
            let action = self.qlearning.choose_action(&state);
            self.intersection_manager.update_from_action(action);
            let next_state = self.intersection_manager.get_state();
            let total_volume: u32 = self.intersection_manager.intersection_volume.iter().sum();
            let reward = self.calculate_reward(total_volume);
            self.qlearning.update(&state, action, reward, &next_state);
            self.last_light_change = Instant::now();
        }
    }

    fn calculate_reward(&mut self, total_volume: u32) -> f64 {
        let total_volume = total_volume as f64;

        // Update min and max volumes observed
        if total_volume < self.min_volume {
            self.min_volume = total_volume;
        }
        if total_volume > self.max_volume {
            self.max_volume = total_volume;
        }

        // Normalize volume
        let normalized_volume = if self.max_volume != self.min_volume {
            (total_volume - self.min_volume) / (self.max_volume - self.min_volume)
        } else {
            0.0
        };
        -normalized_volume
    }

    pub fn spawn_on_timer(&mut self, time: f32) {
        let now = Instant::now();
        let spawn_timer = now.duration_since(self.time_since_last_spawn);

        if spawn_timer.as_secs_f32() > time {
            let mut rng = rand::thread_rng();
            let entrance = rng.gen_range(0..8);
            let vehicle = Vehicle::new(self.id_counter, 50, 10, 10, entrance);

            self.id_counter += 1;

            self.intersection_manager.intersection_volume[(entrance / 2) as usize] += 1;
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
                    break;
                }
            }
        }
    }

    fn handle_vehicle_stops(&mut self) {
        for vehicle in &mut self.vehicles {
            for stop_light in &self.intersection_manager.stop_lights {
                if rectangles_intersect(&vehicle.vision, &stop_light.line) && !stop_light.active {
                    vehicle.speed = 0;
                    break;
                }
            }
        }
    }

    pub fn draw(&mut self, event_loop: &ActiveEventLoop) {
        if let Some(pixels) = &mut self.pixels {
            let frame = pixels.frame_mut();

            frame.copy_from_slice(self.background.as_ref().expect("Background should be set"));

            for vehicle in &self.vehicles {
                vehicle.draw(frame, self.window_width, self.window_height);
            }

            for stop_light in &self.intersection_manager.stop_lights {
                stop_light.draw(frame, self.window_width, self.window_height);
            }

            if let Err(err) = pixels.render() {
                println!("Error during rendering: {:?}", err);
                event_loop.set_control_flow(ControlFlow::Wait);
                return;
            }
        } else {
            println!("No window context available for drawing");
        }
    }
}

fn load_background_frame(frame: &mut [u8]) -> Vec<u8> {
    for (index, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let mut color = &[0x48, 0xb2, 0xe8, 0xff];

        let i = index % WIDTH as usize;
        let j = index / WIDTH as usize;

        if i < WIDTH as usize / 2 + 50 && i > WIDTH as usize / 2 - 50 || j < HEIGHT as usize / 2 + 50 && j > HEIGHT as usize / 2 - 50 {
            color = &[0xa0, 0xa0, 0xa0, 0xff];
        }

        if !(i < WIDTH as usize / 2 + 50 && i > WIDTH as usize / 2 - 50 && j < HEIGHT as usize / 2 + 50 && j > HEIGHT as usize / 2 - 50) {
            if (i < WIDTH as usize / 2 + 5 && i > WIDTH as usize / 2 - 5) ^ (j < HEIGHT as usize / 2 + 5 && j > HEIGHT as usize / 2 - 5) {
                color = &[0xff, 0xff, 0x00, 0xff];
            } else if (i == WIDTH as usize / 2 - 25 || i == WIDTH as usize / 2 + 25) && (j / (DASH_LENGTH + GAP_LENGTH) % 2 == 0) {
                color = &[0xff, 0xff, 0x00, 0xff];
            } else if (j == HEIGHT as usize / 2 - 25 || j == HEIGHT as usize / 2 + 25) && (i / (DASH_LENGTH + GAP_LENGTH) % 2 == 0) {
                color = &[0xff, 0xff, 0x00, 0xff];
            }
        }

        pixel.copy_from_slice(color);
    }

    return frame.to_vec();
}

