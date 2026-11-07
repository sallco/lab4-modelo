use crate::{framebuffer::Framebuffer, obj::Obj, triangle::triangle};
use raylib::prelude::*;

pub const WIDTH: i32 = 1000;
pub const HEIGHT: i32 = 700;
pub const MARGIN: f32 = 60.0;

pub fn screen_vertices(obj: &Obj) -> Vec<Vector2> {
    let mut min = Vector2::new(f32::INFINITY, f32::INFINITY);
    let mut max = Vector2::new(f32::NEG_INFINITY, f32::NEG_INFINITY);
    // Los vértices sin caras no deben desplazar el encuadre visible.
    for &index in &obj.indices {
        let vertex = obj.vertices[index];
        min.x = min.x.min(vertex.x);
        min.y = min.y.min(vertex.y);
        max.x = max.x.max(vertex.x);
        max.y = max.y.max(vertex.y);
    }
    let extent = max - min;
    let scale_x = if extent.x > 0.0 {
        (WIDTH as f32 - 2.0 * MARGIN) / extent.x
    } else {
        f32::INFINITY
    };
    let scale_y = if extent.y > 0.0 {
        (HEIGHT as f32 - 2.0 * MARGIN) / extent.y
    } else {
        f32::INFINITY
    };
    let scale = scale_x.min(scale_y);
    let scale = if scale.is_finite() { scale } else { 1.0 };
    let center = (min + max) * 0.5;

    obj.vertices
        .iter()
        .map(|vertex| {
            Vector2::new(
                (vertex.x - center.x) * scale + WIDTH as f32 * 0.5,
                HEIGHT as f32 * 0.5 - (vertex.y - center.y) * scale,
            )
        })
        .collect()
}

pub fn render(framebuffer: &mut Framebuffer, obj: &Obj) -> usize {
    let vertices = screen_vertices(obj);
    let mut count = 0;
    for indices in obj.indices.as_chunks::<3>().0 {
        let a = vertices[indices[0]];
        let b = vertices[indices[1]];
        let c = vertices[indices[2]];
        triangle(framebuffer, a, b, c, Color::new(165, 210, 235, 255));
        count += 1;
    }
    count
}

