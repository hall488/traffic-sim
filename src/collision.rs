
pub struct Rectangle {
    pub x: f64,
    pub y: f64,
    pub width: u32,
    pub height: u32,
    pub direction: f64, // Add direction to the rectangle
}

impl Rectangle {
    pub fn new(x: f64, y: f64, width: u32, height: u32, direction: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
            direction,
        }
    }
}

pub fn rectangles_intersect(rect1: &Rectangle, rect2: &Rectangle) -> bool {
    let rect1_corners = get_rotated_corners(rect1);
    let rect2_corners = get_rotated_corners(rect2);

    let axes = get_axes(&rect1_corners).into_iter().chain(get_axes(&rect2_corners).into_iter());

    for axis in axes {
        if !overlap_on_axis(&rect1_corners, &rect2_corners, &axis) {
            return false;
        }
    }
    true
}

fn get_rotated_corners(rect: &Rectangle) -> Vec<(f64, f64)> {
    let half_width = rect.width as f64 / 2.0;
    let half_height = rect.height as f64 / 2.0;

    let corners = [
        (-half_width, -half_height),
        (half_width, -half_height),
        (half_width, half_height),
        (-half_width, half_height),
    ];

    corners.iter().map(|&(x, y)| {
        let new_x = rect.x + x * rect.direction.cos() - y * rect.direction.sin();
        let new_y = rect.y + x * rect.direction.sin() + y * rect.direction.cos();
        (new_x, new_y)
    }).collect()
}

fn get_axes(corners: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let mut axes = Vec::new();
    for i in 0..corners.len() {
        let (x1, y1) = corners[i];
        let (x2, y2) = corners[(i + 1) % corners.len()];
        let edge = (x2 - x1, y2 - y1);
        let normal = (-edge.1, edge.0);
        axes.push(normal);
    }
    axes
}

fn project(corners: &[(f64, f64)], axis: &(f64, f64)) -> (f64, f64) {
    let mut min = (corners[0].0 * axis.0 + corners[0].1 * axis.1) /
                  (axis.0 * axis.0 + axis.1 * axis.1);
    let mut max = min;

    for &(x, y) in corners.iter().skip(1) {
        let projection = (x * axis.0 + y * axis.1) / (axis.0 * axis.0 + axis.1 * axis.1);
        if projection < min {
            min = projection;
        } else if projection > max {
            max = projection;
        }
    }
    (min, max)
}

fn overlap_on_axis(corners1: &[(f64, f64)], corners2: &[(f64, f64)], axis: &(f64, f64)) -> bool {
    let (min1, max1) = project(corners1, axis);
    let (min2, max2) = project(corners2, axis);

    max1 >= min2 && max2 >= min1
}


pub fn create_vehicle_vision(start: (f64, f64), direction: f64, width: u32, height: u32) -> Rectangle {
    let angle_cos = direction.cos();
    let angle_sin = direction.sin();

    // Calculating the center point based on the start point, direction, and length
    let x = start.0 + width as f64 / 2.0 * angle_cos;
    let y = start.1 + width as f64 / 2.0 * angle_sin;

    Rectangle {
        x,
        y,
        width,
        height,
        direction, // Set the direction
    }
}
