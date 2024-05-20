use std::time::Duration;
use crate::config::{WIDTH, HEIGHT};
use crate::collision::{Rectangle, create_vehicle_vision};

pub struct Vehicle {
    pub id: usize,
    pub speed: u32,
    pub bounds: Rectangle,
    pub vision: Rectangle,
    pub direction: f64,
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

    pub fn new(id: usize, speed: u32, width: u32, height: u32, entrance: u32, turn: TurnDirection) -> Self {

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
        //vehcile draw code

        self.draw_rectangle(frame, frame_width, frame_height, &self.bounds, [255,0,0,255], true);
        //self.draw_rectangle(frame, frame_width, frame_height, &self.vision, [0,255,0,255], false);
    }

    pub fn draw_rectangle(&self, frame: &mut [u8], frame_width: u32, frame_height: u32, rect: &Rectangle, color: [u8; 4], fill: bool) {
        let half_width = rect.width as f64 / 2.0;
        let half_height = rect.height as f64 / 2.0;

        let corners = [
            (-half_width, -half_height),
            (half_width, -half_height),
            (half_width, half_height),
            (-half_width, half_height),
        ];

        let rotated_corners: Vec<(f64, f64)> = corners.iter().map(|&(x, y)| {
            let new_x = rect.x + x * rect.direction.cos() - y * rect.direction.sin();
            let new_y = rect.y + x * rect.direction.sin() + y * rect.direction.cos();
            (new_x, new_y)
        }).collect();

        if fill {
            self.fill_polygon(frame, frame_width, frame_height, &rotated_corners, color);
        } else {
            self.draw_polygon(frame, frame_width, frame_height, &rotated_corners, color);
        }
    }

    fn fill_polygon(&self, frame: &mut [u8], frame_width: u32, frame_height: u32, corners: &[(f64, f64)], color: [u8; 4]) {
        let mut min_x = frame_width as f64;
        let mut max_x = 0.0;
        let mut min_y = frame_height as f64;
        let mut max_y = 0.0;

        for &(x, y) in corners {
            if x < min_x { min_x = x; }
            if x > max_x { max_x = x; }
            if y < min_y { min_y = y; }
            if y > max_y { max_y = y; }
        }

        for y in (min_y.round() as i32)..=(max_y.round() as i32) {
            for x in (min_x.round() as i32)..=(max_x.round() as i32) {
                if self.is_point_in_polygon((x as f64, y as f64), corners) {
                    self.set_pixel(frame, frame_width, frame_height, x, y, color);
                }
            }
        }
    }

    fn is_point_in_polygon(&self, point: (f64, f64), corners: &[(f64, f64)]) -> bool {
        let mut inside = false;
        let (px, py) = point;
        let mut j = corners.len() - 1;

        for i in 0..corners.len() {
            let (ix, iy) = corners[i];
            let (jx, jy) = corners[j];

            if ((iy > py) != (jy > py)) &&
                (px < (jx - ix) * (py - iy) / (jy - iy) + ix) {
                inside = !inside;
            }

            j = i;
        }

        inside
    }

    fn draw_polygon(&self, frame: &mut [u8], frame_width: u32, frame_height: u32, corners: &[(f64, f64)], color: [u8; 4]) {
        for i in 0..corners.len() {
            let (x1, y1) = corners[i];
            let (x2, y2) = corners[(i + 1) % corners.len()];

            self.draw_line(frame, frame_width, frame_height, x1, y1, x2, y2, color);
        }
    }

    fn draw_line(&self, frame: &mut [u8], frame_width: u32, frame_height: u32, x1: f64, y1: f64, x2: f64, y2: f64, color: [u8; 4]) {
        let x1 = x1.round() as i32;
        let y1 = y1.round() as i32;
        let x2 = x2.round() as i32;
        let y2 = y2.round() as i32;

        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();

        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };

        let mut err = dx - dy;

        let mut x = x1;
        let mut y = y1;

        while x != x2 || y != y2 {
            self.set_pixel(frame, frame_width, frame_height, x, y, color);

            let e2 = 2 * err;

            if e2 > -dy {
                err -= dy;
                x += sx;
            }

            if e2 < dx {
                err += dx;
                y += sy;
            }
        }
    }

    fn set_pixel(&self, frame: &mut [u8], frame_width: u32, frame_height: u32, x: i32, y: i32, color: [u8; 4]) {
        if x >= 0 && x < frame_width as i32 && y >= 0 && y < frame_height as i32 {
            let index = ((y as u32 * frame_width + x as u32) * 4) as usize;
            frame[index] = color[0];
            frame[index + 1] = color[1];
            frame[index + 2] = color[2];
            frame[index + 3] = color[3];
        }
    }

}
