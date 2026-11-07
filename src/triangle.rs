use crate::{framebuffer::Framebuffer, line::line};
use raylib::prelude::*;

pub fn triangle(framebuffer: &mut Framebuffer, a: Vector2, b: Vector2, c: Vector2, color: Color) {
    line(framebuffer, a, b, color);
    line(framebuffer, b, c, color);
    line(framebuffer, c, a, color);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connects_three_edges_without_filling_the_interior() {
        let mut fb = Framebuffer::new(8, 8);
        triangle(
            &mut fb,
            Vector2::new(1.0, 1.0),
            Vector2::new(5.0, 1.0),
            Vector2::new(1.0, 5.0),
            Color::WHITE,
        );
        for (x, y) in [(1, 1), (5, 1), (1, 5), (3, 1), (1, 3), (3, 3)] {
            assert_eq!(fb.image.get_color(x, y), Color::WHITE);
        }
        assert_ne!(fb.image.get_color(2, 2), Color::WHITE);
    }
}
