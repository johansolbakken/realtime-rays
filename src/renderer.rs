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
                let mut coord = glm::vec2(
                    x as f32 / self.image.get_width() as f32,
                    y as f32 / self.image.get_height() as f32,
                );
                coord = coord * 2.0 - 1.0;

                let color = self.per_pixel(coord);

                self.image.set_pixel(x, y, color); // ABGR
            }
        }
    }

    fn per_pixel(&mut self, coord: Vector2<f32>) -> Color {
        let r = (coord.x * 255.0) as u8 as u32;
        let g = (coord.y * 255.0) as u8 as u32;

        let ray_direction = glm::vec3(coord.x, coord.y, -1.0);
        let ray_origin = glm::vec3(0.0, 0.0, 2.0);
        let radius = 0.5;

        let a = glm::dot(ray_direction, ray_direction);
        let b = 2.0 * glm::dot(ray_direction, ray_origin);
        let c = glm::dot(ray_origin, ray_origin) - radius * radius;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant >= 0.0 {
            return 0xffff00ff;
        }

        0xff000000
    }
}
