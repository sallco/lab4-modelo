use raylib::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct View {
    pub rotation: Vector3,
    pub zoom: f32,
    pub offset: Vector2,
}

impl Default for View {
    fn default() -> Self {
        Self {
            rotation: Vector3::zero(),
            zoom: 1.0,
            offset: Vector2::zero(),
        }
    }
}

impl View {
    pub fn update(&mut self, dt: f32, held: impl Fn(KeyboardKey) -> bool) -> bool {
        use KeyboardKey::*;
        let previous = *self;
        if held(KEY_R) {
            *self = Self::default();
            return *self != previous;
        }
        let dt = dt.clamp(0.0, 0.1);
        let axis =
            |positive, negative| (i32::from(held(positive)) - i32::from(held(negative))) as f32;
        self.rotation += Vector3::new(
            axis(KEY_DOWN, KEY_UP),
            axis(KEY_RIGHT, KEY_LEFT),
            axis(KEY_E, KEY_Q),
        ) * (1.5 * dt);
        for angle in [
            &mut self.rotation.x,
            &mut self.rotation.y,
            &mut self.rotation.z,
        ] {
            *angle = angle.rem_euclid(std::f32::consts::TAU);
        }
        self.offset += Vector2::new(axis(KEY_D, KEY_A), axis(KEY_S, KEY_W)) * (250.0 * dt);
        let zoom_in = held(KEY_EQUAL) || held(KEY_KP_ADD);
        let zoom_out = held(KEY_MINUS) || held(KEY_KP_SUBTRACT);
        let zoom_direction = (i32::from(zoom_in) - i32::from(zoom_out)) as f32;
        self.zoom = (self.zoom * (zoom_direction * dt).exp()).clamp(0.25, 4.0);
        *self != previous
    }

    pub fn rotate(&self, point: Vector3) -> Vector3 {
        let (sx, cx) = self.rotation.x.sin_cos();
        let (sy, cy) = self.rotation.y.sin_cos();
        let (sz, cz) = self.rotation.z.sin_cos();
        let y = point.y * cx - point.z * sx;
        let z = point.y * sx + point.z * cx;
        let x = point.x * cy + z * sy;
        let z = -point.x * sy + z * cy;
        Vector3::new(x * cz - y * sz, x * sz + y * cz, z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keyboard_controls_change_each_view_axis_and_reset() {
        use KeyboardKey::*;
        for key in [KEY_UP, KEY_DOWN, KEY_LEFT, KEY_RIGHT, KEY_Q, KEY_E] {
            let mut view = View::default();
            assert!(view.update(0.05, |held| held == key));
            assert_ne!(view.rotation, Vector3::zero());
            assert_eq!(view.offset, Vector2::zero());
            assert!(view.update(0.05, |held| held == KEY_R));
            assert_eq!(view, View::default());
        }
        for key in [KEY_W, KEY_A, KEY_S, KEY_D] {
            let mut view = View::default();
            assert!(view.update(0.05, |held| held == key));
            assert_ne!(view.offset, Vector2::zero());
            assert_eq!(view.rotation, Vector3::zero());
        }
        assert!(!View::default().update(0.05, |_| false));
        assert!(!View::default().update(0.05, |key| key == KEY_LEFT || key == KEY_RIGHT));
    }

    #[test]
    fn zoom_is_bounded_and_motion_uses_elapsed_time() {
        use KeyboardKey::*;
        for (key, expected) in [
            (KEY_EQUAL, 4.0),
            (KEY_KP_ADD, 4.0),
            (KEY_MINUS, 0.25),
            (KEY_KP_SUBTRACT, 0.25),
        ] {
            let mut view = View::default();
            for _ in 0..100 {
                view.update(0.1, |held| held == key);
            }
            assert_eq!(view.zoom, expected);
        }
        let mut first = View::default();
        first.update(0.1, |key| key == KEY_RIGHT || key == KEY_D);
        let mut second = View::default();
        for _ in 0..2 {
            second.update(0.05, |key| key == KEY_RIGHT || key == KEY_D);
        }
        assert!((first.rotation - second.rotation).length() < 0.0001);
        assert!((first.offset - second.offset).length() < 0.0001);
    }

    #[test]
    fn rotates_each_axis_and_preserves_distance() {
        use std::f32::consts::FRAC_PI_2;
        let cases = [
            (
                Vector3::new(FRAC_PI_2, 0.0, 0.0),
                Vector3::new(0.0, 1.0, 0.0),
                Vector3::new(0.0, 0.0, 1.0),
            ),
            (
                Vector3::new(0.0, FRAC_PI_2, 0.0),
                Vector3::new(0.0, 0.0, 1.0),
                Vector3::new(1.0, 0.0, 0.0),
            ),
            (
                Vector3::new(0.0, 0.0, FRAC_PI_2),
                Vector3::new(1.0, 0.0, 0.0),
                Vector3::new(0.0, 1.0, 0.0),
            ),
        ];
        for (rotation, point, expected) in cases {
            let view = View {
                rotation,
                ..View::default()
            };
            let rotated = view.rotate(point);
            assert!((rotated - expected).length() < 0.0001);
            assert!((rotated.length() - point.length()).abs() < 0.0001);
        }
        let point = Vector3::new(2.0, -3.0, 4.0);
        assert_eq!(View::default().rotate(point), point);
    }
}
