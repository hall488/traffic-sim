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

    pub fn update(&self) {
       //vroom vroom
    }

    pub fn draw(&self) {
        //vehcile draw code
    }

}
