use crate::{framebuffer::Framebuffer, line::line};
use raylib::prelude::*;

pub fn triangle(framebuffer: &mut Framebuffer, a: Vector2, b: Vector2, c: Vector2, color: Color) {
    line(framebuffer, a, b, color);
    line(framebuffer, b, c, color);
    line(framebuffer, c, a, color);
}

