pub struct IntersectionManager {
    pub intersection_volume: [u32; 4],
    pub stoplight_states: [bool; 4],
}

impl IntersectionManager {

    pub fn new() -> Self {
        Self {
            intersection_volume: [0,0,0,0],
            stoplight_states: [false, false, false, false],
        }
    }
}
