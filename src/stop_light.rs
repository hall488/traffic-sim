use crate::collision::Rectangle;
use crate::config::{WIDTH, HEIGHT};
use crate::drawing_util::draw_rectangle;

pub struct StopLight {
    pub line: Rectangle,
    active: bool,
}

impl StopLight {

    pub fn new(lane: u32) -> Self {

        let line = match lane {
            0 => Rectangle::new(WIDTH as f64 / 2.0 - 50.0, HEIGHT as f64 / 2.0 + 25.0, 1, 50, 0.0),
            1 => Rectangle::new(WIDTH as f64 / 2.0 - 25.0, HEIGHT as f64 / 2.0 - 50.0, 50, 1, 0.0),
            2 => Rectangle::new(WIDTH as f64 / 2.0 + 50.0, HEIGHT as f64 / 2.0 - 25.0, 1, 50, 0.0),
            3 => Rectangle::new(WIDTH as f64 / 2.0 + 25.0, HEIGHT as f64 / 2.0 + 50.0, 50, 1, 0.0),
            _ => unreachable!(),
        };


        Self {
            line,
            active: false,
        }
    }


    pub fn draw(&self, frame: &mut [u8], frame_width: u32, frame_height: u32) {

        draw_rectangle(frame, frame_width, frame_height, &self.line, [255,0,0,255], true);
        //self.draw_rectangle(frame, frame_width, frame_height, &self.vision, [0,255,0,255], false);
    }

}
