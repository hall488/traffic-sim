use crate::stop_light::StopLight;
use std::time::Instant;
use ndarray::Array1;

pub struct IntersectionManager {
    pub intersection_volume: [u32; 4],
    pub stop_lights: [StopLight; 4]
}

impl IntersectionManager {

    pub fn new() -> Self {
        let stop_lights = [StopLight::new(0), StopLight::new(1), StopLight::new(2), StopLight::new(3)];
        Self {
            intersection_volume: [0,0,0,0],
            stop_lights,
        }
    }
    pub fn update_from_action(&mut self, action: usize) {
        if action < 4 {
            self.stop_lights[action].flip();
            self.stop_lights[action].time_since_flip = Instant::now();
        }
        // If action is 4, do nothing
    }

    pub fn get_state(&self) -> Array1<f64> {
        Array1::from(vec![
            self.intersection_volume[0] as f64,
            self.intersection_volume[1] as f64,
            self.intersection_volume[2] as f64,
            self.intersection_volume[3] as f64,
            self.stop_lights[0].active as u32 as f64,
            self.stop_lights[1].active as u32 as f64,
            self.stop_lights[2].active as u32 as f64,
            self.stop_lights[3].active as u32 as f64,
            self.stop_lights[0].time_since_flip.elapsed().as_secs_f64(),
        ])
    }
}


