use std::collections::HashMap;
use crate::vehicle::Vehicle;

pub struct Grid {
    cell_size: f64,
    cells: HashMap<(i32,i32), Vec<usize>>,
}

impl Grid {
    pub fn new(cell_size: f64) -> Self {
        Grid {
            cell_size,
            cells: HashMap::new(),
        }
    }

    pub fn add_vehicle(&mut self, vehicle: &Vehicle) {
        let cell = self.get_cell(vehicle.bounds.x, vehicle.bounds.y);
        self.cells.entry(cell).or_default().push(vehicle.id);
    }

    fn get_cell(&self, _x: f64, _y: f64) -> (i32, i32) {
        let x = (_x / self.cell_size).floor() as i32;
        let y = (_y / self.cell_size).floor() as i32;

        (x,y)
    }

    pub fn get_neighbors(&self, _x: f64, _y: f64) -> Vec<usize> {
        let cell = self.get_cell(_x, _y);
        let mut neighbors = Vec::new();

        for x in -1..=1 {
            for y in -1..=1 {
                if let Some(cell_vehicles) = self.cells.get(&(cell.0 + x, cell.1 + y)) {
                    neighbors.extend(cell_vehicles);
                }
            }
        }

        neighbors
    }
}
