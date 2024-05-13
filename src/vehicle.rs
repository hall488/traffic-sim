use std::time::Duration;
use crate::config::{WIDTH, HEIGHT};

pub struct Vehicle {
    speed: u32,
    width: u32,
    height: u32,
    x: f64,
    y: f64,
    direction: f64,
    state: State,
    lane: Lane,
    turn: TurnDirection,
}

#[derive(Debug)]
pub enum State {
    Driving,
    Turning,
    Stop,
}

pub enum TurnDirection {
    Left,
    Straight,
    Right,
}

pub enum Lane {
    Left,
    Right,
}

impl Vehicle {

    pub fn new(speed: u32, width: u32, height: u32, x: f64, y: f64, direction: f64, lane: Lane) -> Self {
        Self {
            speed,
            width,
            height,
            x,
            y,
            direction,
            state: State::Driving,
            lane,
            turn: TurnDirection::Right,
        }
    }

    pub fn update(&mut self, dt: Duration ) {
        //vroom vroom
        println!("state {:?}", self.state);
        println!("x {0} y {1} d {2} dt {3}", self.x, self.y, self.direction, dt.as_secs_f64());
        match self.state {
            State::Driving => {
                self.x += self.speed as f64 * dt.as_secs_f64() * self.direction.cos();
                self.y += self.speed as f64 * dt.as_secs_f64() * self.direction.sin();

                if  self.x < WIDTH as f64 / 2.0 + 50.0 &&
                self.x > WIDTH as f64 / 2.0 - 50.0 &&
                self.y < HEIGHT as f64 / 2.0 + 50.0 &&
                self.y > HEIGHT as f64 / 2.0 - 50.0 {
                    self.state = State::Turning;
                }
            },
            State::Stop => {

            },
            State::Turning => {
                let radius = self.get_turn_radius();
                self.apply_turn(radius, TurnDirection::Right, dt.as_secs_f64());
                if  self.x >= WIDTH as f64 / 2.0 + 50.0 ||
                self.x <= WIDTH as f64 / 2.0 - 50.0 ||
                self.y >= HEIGHT as f64 / 2.0 + 50.0 ||
                self.y <= HEIGHT as f64 / 2.0 - 50.0 {
                    self.quantize_direction();
                    self.state = State::Driving;
                }
            },
        }
    }

    pub fn get_turn_radius(&self) -> f64 {
        return match self.lane {
            Lane::Right => {
                match self.turn {
                    TurnDirection::Right => 12.5,
                    TurnDirection::Left => 87.5,
                    TurnDirection::Straight => 0.0,
                }
            }
            Lane::Left => {
                match self.turn {
                    TurnDirection::Right => 37.5,
                    TurnDirection::Left => 62.5,
                    TurnDirection::Straight => 0.0,
                }
            }
        }
    }

    pub fn apply_turn(&mut self, radius: f64, turn_direction: TurnDirection, delta_time: f64) {
        let angular_velocity = self.speed as f64 / radius; // Angular velocity = speed / radius
        let angular_change = angular_velocity * delta_time; // Change in angle is angular velocity * time

        match turn_direction {
            TurnDirection::Left => self.direction -= angular_change,
            TurnDirection::Right => self.direction += angular_change,
            TurnDirection::Straight => (),
        }

        // Normalize direction to stay within the range [0, 2*pi)
        self.direction = (self.direction + 2.0 * std::f64::consts::PI) % (2.0 * std::f64::consts::PI);

        // Update the vehicle's position based on the new direction
        self.x += self.speed as f64 * delta_time * self.direction.cos();
        self.y += self.speed as f64 * delta_time * self.direction.sin();
    }

    fn quantize_direction(&mut self) {
        let pi = std::f64::consts::PI;
        let half_pi = pi / 2.0;
        self.direction = (self.direction / half_pi).round() * half_pi;
        self.direction = self.direction % (2.0 * pi);
    }


    pub fn check_bounds(&self) -> bool {
        if self.x > WIDTH as f64 || self.x < 0.0 || self.y > HEIGHT as f64 || self.y < 0.0 {

            return true;
        }

        return false;
    }

    pub fn draw(&self, frame: &mut [u8], frame_width: u32, frame_height: u32) {
        //vehcile draw code

        self.modify_frame(frame, frame_width, frame_height);
    }

    pub fn modify_frame(&self, frame: &mut [u8], frame_width: u32, frame_height: u32) {
        let half_width = self.width / 2;
        let half_height = self.height / 2;

        let x_i32 = self.x.round() as i32;
        let y_i32 = self.y.round() as i32;

        // Calculate the start and end coordinates, clamped to the frame's dimensions
        let start_x = x_i32.saturating_sub(half_width as i32).max(0).min(frame_width as i32 - 1);
        let end_x = (x_i32 + half_width as i32).min(frame_width as i32 - 1);
        let start_y = y_i32.saturating_sub(half_height as i32).max(0).min(frame_height as i32 - 1);
        let end_y = (y_i32 + half_height as i32).min(frame_height as i32 - 1);

        // Loop over the pixel coordinates that fall within the vehicle's area
        for y in start_y..end_y {
            for x in start_x..end_x {
                let index = ((y * frame_width as i32 + x) * 4) as usize; // Assuming 4 bytes per pixel (RGBA)

                // Modify the pixel's color
                frame[index] = 255;      // R
                frame[index + 1] = 0;    // G
                frame[index + 2] = 0;    // B
                frame[index + 3] = 255;  // A
            }
        }
    }
}
