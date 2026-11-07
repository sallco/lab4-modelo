use crate::framebuffer::Framebuffer;
use raylib::prelude::*;

// Bresenham del laboratorio anterior, adaptado a Vector2 y al framebuffer.
pub fn line(framebuffer: &mut Framebuffer, start: Vector2, end: Vector2, color: Color) {
    let mut x0 = start.x.round() as i32;
    let mut y0 = start.y.round() as i32;
    let x1 = end.x.round() as i32;
    let y1 = end.y.round() as i32;
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;

    loop {
        framebuffer.point(Vector2::new(x0 as f32, y0 as f32), color);
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x0 += sx;
        }
        if e2 < dx {
            err += dx;
            y0 += sy;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draws_endpoints_in_every_octant_and_single_points() {
        let origin = Vector2::new(5.0, 5.0);
        for (x, y) in [
            (9, 7),
            (7, 9),
            (3, 9),
            (1, 7),
            (1, 3),
            (3, 1),
            (7, 1),
            (9, 3),
            (5, 5),
            (5, 1),
            (9, 5),
        ] {
            let mut fb = Framebuffer::new(11, 11);
            line(
                &mut fb,
                origin,
                Vector2::new(x as f32, y as f32),
                Color::WHITE,
            );
            assert_eq!(fb.image.get_color(5, 5), Color::WHITE);
            assert_eq!(fb.image.get_color(x, y), Color::WHITE);
            let pixels = fb.image.get_image_data();
            let count = pixels.iter().filter(|&&c| c == Color::WHITE).count();
            assert_eq!(count, (x - 5).abs().max((y - 5).abs()) as usize + 1);
        }
    }
}
