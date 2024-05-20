use crate::collision::Rectangle;

pub fn draw_rectangle(frame: &mut [u8], frame_width: u32, frame_height: u32, rect: &Rectangle, color: [u8; 4], fill: bool) {
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
        fill_polygon(frame, frame_width, frame_height, &rotated_corners, color);
    } else {
        draw_polygon(frame, frame_width, frame_height, &rotated_corners, color);
    }
}

fn fill_polygon(frame: &mut [u8], frame_width: u32, frame_height: u32, corners: &[(f64, f64)], color: [u8; 4]) {
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
            if is_point_in_polygon((x as f64, y as f64), corners) {
                set_pixel(frame, frame_width, frame_height, x, y, color);
            }
        }
    }
}

fn is_point_in_polygon(point: (f64, f64), corners: &[(f64, f64)]) -> bool {
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

fn draw_polygon(frame: &mut [u8], frame_width: u32, frame_height: u32, corners: &[(f64, f64)], color: [u8; 4]) {
    for i in 0..corners.len() {
        let (x1, y1) = corners[i];
        let (x2, y2) = corners[(i + 1) % corners.len()];

        draw_line(frame, frame_width, frame_height, x1, y1, x2, y2, color);
    }
}

fn draw_line(frame: &mut [u8], frame_width: u32, frame_height: u32, x1: f64, y1: f64, x2: f64, y2: f64, color: [u8; 4]) {
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
        set_pixel(frame, frame_width, frame_height, x, y, color);

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

fn set_pixel(frame: &mut [u8], frame_width: u32, frame_height: u32, x: i32, y: i32, color: [u8; 4]) {
    if x >= 0 && x < frame_width as i32 && y >= 0 && y < frame_height as i32 {
        let index = ((y as u32 * frame_width + x as u32) * 4) as usize;
        frame[index] = color[0];
        frame[index + 1] = color[1];
        frame[index + 2] = color[2];
        frame[index + 3] = color[3];
    }
}

