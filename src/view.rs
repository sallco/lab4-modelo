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
