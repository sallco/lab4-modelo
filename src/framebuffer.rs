use raylib::prelude::*;

const BACKGROUND: Color = Color::new(13, 13, 20, 255);

pub struct Framebuffer {
    pub image: Image,
}

impl Framebuffer {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            image: Image::gen_image_color(width, height, BACKGROUND),
        }
    }

    pub fn clear(&mut self) {
        self.image.clear_background(BACKGROUND);
    }

    pub fn point(&mut self, position: Vector2, color: Color) {
        let x = position.x.round() as i32;
        let y = position.y.round() as i32;
        if x >= 0 && x < self.image.width && y >= 0 && y < self.image.height {
            self.image.draw_pixel(x, y, color);
        }
    }
}
