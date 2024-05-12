pub struct Vehicle {
    speed: u32,
    width: u32,
    height: u32,
    x: u32,
    y: u32,
    direction: f32,
}

impl Vehicle {

    pub fn new(speed: u32, width: u32, height: u32, x: u32, y: u32, direction: f32) -> Self {
        Self {
            speed,
            width,
            height,
            x,
            y,
            direction,
        }
    }

    pub fn update(&mut self) {
       //vroom vroom
        self.x += self.speed;
    }

    pub fn draw(&self, frame: &mut [u8], frame_width: u32, frame_height: u32) {
        //vehcile draw code

        self.modify_frame(frame, frame_width, frame_height);
    }

    pub fn modify_frame(&self, frame: &mut [u8], frame_width: u32, frame_height: u32) {
        let half_width = self.width / 2;
        let half_height = self.height / 2;

        // Calculate the start and end coordinates, clamped to the frame's dimensions
        let start_x = self.x.saturating_sub(half_width).max(0).min(frame_width - 1);
        let end_x = (self.x + half_width).min(frame_width - 1);
        let start_y = self.y.saturating_sub(half_height).max(0).min(frame_height - 1);
        let end_y = (self.y + half_height).min(frame_height - 1);

        // Loop over the pixel coordinates that fall within the vehicle's area
        for y in start_y..end_y {
            for x in start_x..end_x {
                let index = ((y * frame_width + x) * 4) as usize; // Assuming 4 bytes per pixel (RGBA)

                // Modify the pixel's color
                frame[index] = 255;      // R
                frame[index + 1] = 0;    // G
                frame[index + 2] = 0;    // B
                frame[index + 3] = 255;  // A
            }
        }
    }
}
