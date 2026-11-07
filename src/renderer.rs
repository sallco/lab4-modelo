use crate::{framebuffer::Framebuffer, obj::Obj, triangle::triangle, view::View};
use raylib::prelude::*;

pub const WIDTH: i32 = 1000;
pub const HEIGHT: i32 = 700;
pub const MARGIN: f32 = 60.0;

pub fn screen_vertices(obj: &Obj, view: &View) -> Vec<Vector2> {
    let rotated: Vec<_> = obj
        .vertices
        .iter()
        .map(|&vertex| view.rotate(vertex))
        .collect();
    let mut min = Vector2::new(f32::INFINITY, f32::INFINITY);
    let mut max = Vector2::new(f32::NEG_INFINITY, f32::NEG_INFINITY);
    // Los vértices sin caras no deben desplazar el encuadre visible.
    for &index in &obj.indices {
        let vertex = rotated[index];
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

    rotated
        .iter()
        .map(|vertex| {
            Vector2::new(
                (vertex.x - center.x) * scale * view.zoom + WIDTH as f32 * 0.5 + view.offset.x,
                HEIGHT as f32 * 0.5 - (vertex.y - center.y) * scale * view.zoom + view.offset.y,
            )
        })
        .collect()
}

pub fn render(framebuffer: &mut Framebuffer, obj: &Obj, view: &View) -> usize {
    framebuffer.clear();
    let vertices = screen_vertices(obj, view);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn applies_zoom_and_pan_after_fitting_the_model() {
        let obj = Obj::load(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/nave1.obj")).unwrap();
        let initial = screen_vertices(&obj, &View::default());
        let view = View {
            zoom: 1.5,
            offset: Vector2::new(30.0, -20.0),
            ..View::default()
        };
        let moved = screen_vertices(&obj, &view);
        let center = Vector2::new(WIDTH as f32 * 0.5, HEIGHT as f32 * 0.5);
        for (before, after) in initial.iter().zip(moved) {
            assert!((after - ((*before - center) * 1.5 + center + view.offset)).length() < 0.001);
        }
    }

    #[test]
    fn rotation_changes_the_image_and_reset_removes_old_edges() {
        let obj = Obj::load(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/nave1.obj")).unwrap();
        let mut fb = Framebuffer::new(WIDTH, HEIGHT);
        render(&mut fb, &obj, &View::default());
        let initial = fb.image.get_image_data().to_vec();
        let view = View {
            rotation: Vector3::new(0.5, 0.7, 0.0),
            ..View::default()
        };
        assert_eq!(render(&mut fb, &obj, &view), 116_480);
        assert_ne!(initial, fb.image.get_image_data().to_vec());
        render(&mut fb, &obj, &View::default());
        assert_eq!(initial, fb.image.get_image_data().to_vec());
    }

    #[test]
    fn spaceship_is_centered_fits_and_draws_every_triangle() {
        let obj = Obj::load(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/nave1.obj")).unwrap();
        let projected = screen_vertices(&obj, &View::default());
        let vertices: Vec<_> = obj.indices.iter().map(|&index| projected[index]).collect();
        let min_x = vertices.iter().map(|v| v.x).fold(f32::INFINITY, f32::min);
        let max_x = vertices
            .iter()
            .map(|v| v.x)
            .fold(f32::NEG_INFINITY, f32::max);
        let min_y = vertices.iter().map(|v| v.y).fold(f32::INFINITY, f32::min);
        let max_y = vertices
            .iter()
            .map(|v| v.y)
            .fold(f32::NEG_INFINITY, f32::max);
        assert!(min_x >= MARGIN - 0.01 && max_x <= WIDTH as f32 - MARGIN + 0.01);
        assert!(min_y >= MARGIN - 0.01 && max_y <= HEIGHT as f32 - MARGIN + 0.01);
        assert!((min_x + max_x - WIDTH as f32).abs() < 0.01);
        assert!((min_y + max_y - HEIGHT as f32).abs() < 0.01);
        assert!((max_x - min_x).max(max_y - min_y) > 500.0);
        let mut fb = Framebuffer::new(WIDTH, HEIGHT);
        assert_eq!(render(&mut fb, &obj, &View::default()), 116_480);
        assert!(
            fb.image
                .get_image_data()
                .iter()
                .any(|&c| c == Color::new(165, 210, 235, 255))
        );
    }

    #[test]
    fn fits_flat_models_and_single_positions() {
        for vertices in [
            vec![Vector3::new(2.0, 3.0, 0.0)],
            vec![Vector3::new(2.0, -3.0, 0.0), Vector3::new(2.0, 3.0, 0.0)],
        ] {
            let obj = Obj {
                vertices,
                indices: vec![0, 0, 0],
            };
            assert!(
                screen_vertices(&obj, &View::default())
                    .iter()
                    .all(|v| v.x.is_finite() && v.y.is_finite())
            );
        }
    }
}
