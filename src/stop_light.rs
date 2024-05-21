use crate::collision::Rectangle;
use crate::config::{WIDTH, HEIGHT};
use crate::drawing_util::draw_rectangle;
use std::time::{Duration, Instant};

pub struct StopLight {
    pub line: Rectangle,
    time_since_flip: Instant,
    pub active: bool,
}

impl StopLight {

    pub fn new(lane: u32) -> Self {

        let (line, timer_offset) = match lane {
            0 => (Rectangle::new(WIDTH as f64 / 2.0 - 50.0, HEIGHT as f64 / 2.0 + 25.0, 1, 50, 0.0), 0.0),
            1 => (Rectangle::new(WIDTH as f64 / 2.0 - 25.0, HEIGHT as f64 / 2.0 - 50.0, 50, 1, 0.0), 4.0),
            2 => (Rectangle::new(WIDTH as f64 / 2.0 + 50.0, HEIGHT as f64 / 2.0 - 25.0, 1, 50, 0.0), 8.0),
            3 => (Rectangle::new(WIDTH as f64 / 2.0 + 25.0, HEIGHT as f64 / 2.0 + 50.0, 50, 1, 0.0), 12.0),
            _ => unreachable!(),
        };



        Self {
            line,
            time_since_flip: Instant::now() - Duration::from_secs_f32(timer_offset),
            active: false,
        }
    }

    pub fn update(&mut self) {

        self.flip_on_timer(13.0, 3.0);
    }

    pub fn flip_on_timer(&mut self, off_time: f32, on_time: f32) {
        let now = Instant::now();
        let time_since_flip = now.duration_since(self.time_since_flip);

        if self.active {
            if time_since_flip.as_secs_f32() > on_time {
                self.active = !self.active;
                self.time_since_flip = now;
            }
        } else {
            if time_since_flip.as_secs_f32() > off_time {
                self.active = !self.active;
                self.time_since_flip = now;
            }
        }
    }

    pub fn draw(&self, frame: &mut [u8], frame_width: u32, frame_height: u32) {

        let mut color = [255, 0, 0, 255];

        if self.active {
            color = [0, 255, 0, 255];
        }

        draw_rectangle(frame, frame_width, frame_height, &self.line, color, true);
        //self.draw_rectangle(frame, frame_width, frame_height, &self.vision, [0,255,0,255], false);
    }

}
