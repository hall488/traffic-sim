use std::time::Duration;
use crate::config::{WIDTH, HEIGHT};
use crate::collision::{Rectangle, create_vehicle_vision};
use crate::drawing_util::draw_rectangle;
use rand::Rng;

pub struct Vehicle {
    pub id: usize,
    pub speed: u32,
    pub bounds: Rectangle,
    pub vision: Rectangle,
    pub direction: f64,
    state: State,
    pub lane: Lane,
    turn: TurnDirection,
    pub entrance: u32,
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

#[derive(PartialEq)]
pub enum Lane {
    Left,
    Right,
}

impl Vehicle {

    pub fn new(id: usize, speed: u32, width: u32, height: u32, entrance: u32) -> Self {

        let (x,y,direction,lane) = match entrance {
            0 => (0.0, HEIGHT as f64 /2.0 + 12.5, 0.0, Lane::Left),
            1 => (0.0, HEIGHT as f64 /2.0 + 37.5, 0.0, Lane::Right),
            2 => (WIDTH as f64 /2.0 - 12.5, 0.0, std::f64::consts::PI/2.0, Lane::Left),
            3 => (WIDTH as f64 /2.0 - 37.5, 0.0, std::f64::consts::PI/2.0, Lane::Right),
            4 => (WIDTH as f64, HEIGHT as f64 /2.0 - 12.5, std::f64::consts::PI, Lane::Left),
            5 => (WIDTH as f64, HEIGHT as f64 /2.0 - 37.5, std::f64::consts::PI, Lane::Right),
            6 => (WIDTH as f64 /2.0 + 12.5, HEIGHT as f64, std::f64::consts::PI*1.5, Lane::Left),
            7 => (WIDTH as f64 /2.0 + 37.5, HEIGHT as f64, std::f64::consts::PI*1.5, Lane::Right),
            _ => unreachable!(),
        };


        let mut rng = rand::thread_rng();

        let turn = match rng.gen_range(0..3) {
            0 => if lane == Lane::Left {
                TurnDirection::Left
            } else {
                TurnDirection:: Right
            },
            1 => if lane == Lane::Left {
                TurnDirection::Left
            } else {
                TurnDirection:: Right
            },
            2 => TurnDirection::Straight,
            _ => unreachable!(),
        };


        let bounds = Rectangle::new(x,y,width,height,direction);
        let vision = create_vehicle_vision((x,y), direction, 20, 10);

        Self {
            id,
            speed,
            bounds,
            vision,
            direction,
            state: State::Driving,
            lane,
            turn,
            entrance,
        }
    }

    pub fn update(&mut self, dt: Duration ) {
        //vroom vroom
        //println!("state {:?}", self.state);
        //println!("x {0} y {1} d {2} dt {3}", self.x, self.y, self.direction, dt.as_secs_f64());
        match self.state {
            State::Driving => {
                self.bounds.x += self.speed as f64 * dt.as_secs_f64() * self.direction.cos();
                self.bounds.y += self.speed as f64 * dt.as_secs_f64() * self.direction.sin();

                self.direction = (self.direction + 2.0 * std::f64::consts::PI) % (2.0 * std::f64::consts::PI);
                self.bounds.direction = self.direction;

                self.vision = create_vehicle_vision((self.bounds.x, self.bounds.y), self.direction, 20, 10);

                if  self.bounds.x < WIDTH as f64 / 2.0 + 50.0 &&
                self.bounds.x > WIDTH as f64 / 2.0 - 50.0 &&
                self.bounds.y < HEIGHT as f64 / 2.0 + 50.0 &&
                self.bounds.y > HEIGHT as f64 / 2.0 - 50.0 {
                    self.state = State::Turning;
                }
            },
            State::Stop => {

            },
            State::Turning => {
                let radius = self.get_turn_radius();
                self.apply_turn(radius, dt.as_secs_f64());
                if  self.bounds.x >= WIDTH as f64 / 2.0 + 50.0 ||
                self.bounds.x <= WIDTH as f64 / 2.0 - 50.0 ||
                self.bounds.y >= HEIGHT as f64 / 2.0 + 50.0 ||
                self.bounds.y <= HEIGHT as f64 / 2.0 - 50.0 {
                    self.quantize_direction();
                    self.state = State::Driving;
                }
            },
        }
        self.speed = 500;
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

    pub fn apply_turn(&mut self, radius: f64, delta_time: f64) {
        let angular_velocity = self.speed as f64 / radius; // Angular velocity = speed / radius
        let angular_change = angular_velocity * delta_time; // Change in angle is angular velocity * time

        match self.turn {
            TurnDirection::Left => self.direction -= angular_change,
            TurnDirection::Right => self.direction += angular_change,
            TurnDirection::Straight => (),
        }

        // Normalize direction to stay within the range [0, 2*pi)
        self.direction = (self.direction + 2.0 * std::f64::consts::PI) % (2.0 * std::f64::consts::PI);

        // Update the vehicle's position based on the new direction
        self.bounds.x += self.speed as f64 * delta_time * self.direction.cos();
        self.bounds.y += self.speed as f64 * delta_time * self.direction.sin();
        self.bounds.direction = self.direction;

        self.vision = create_vehicle_vision((self.bounds.x, self.bounds.y), self.direction, 20, 10);
    }

    fn quantize_direction(&mut self) {
        let pi = std::f64::consts::PI;
        let half_pi = pi / 2.0;
        self.direction = (self.direction / half_pi).round() * half_pi;
        self.direction = self.direction % (2.0 * pi);
    }


    pub fn check_bounds(&self) -> bool {
        if self.bounds.x > WIDTH as f64 || self.bounds.x < 0.0 || self.bounds.y > HEIGHT as f64 || self.bounds.y < 0.0 {

            return true;
        }

        return false;
    }

    pub fn draw(&self, frame: &mut [u8], frame_width: u32, frame_height: u32) {

        draw_rectangle(frame, frame_width, frame_height, &self.bounds, [0,0,255,255], true);
        //self.draw_rectangle(frame, frame_width, frame_height, &self.vision, [0,255,0,255], false);
    }

}
