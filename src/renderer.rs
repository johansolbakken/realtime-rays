use glm::Vector2;

use crate::image::{Color, Image};

pub struct Renderer {
    image: Image,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            image: Image::new(0, 0),
        }
    }

    pub fn final_image(&self) -> &Image {
        &self.image
    }

    pub fn on_resize(&mut self, width: usize, height: usize) {
        self.image.resize(width, height);
    }

    pub fn render(&mut self) {
        for y in 0..self.image.get_height() {
            for x in 0..self.image.get_width() {
                let coord = glm::vec2(
                    x as f32 / self.image.get_width() as f32,
                    y as f32 / self.image.get_height() as f32,
                );

                let color = self.per_pixel(coord);

                self.image.set_pixel(x, y, color); // ABGR
            }
        }
    }

    fn per_pixel(&mut self, coord: Vector2<f32>) -> Color {
        let r = (coord.x * 255.0) as u8 as u32;
        let g = (coord.y * 255.0) as u8 as u32;
        0xff000000 | (g << 8) | r
    }
}
