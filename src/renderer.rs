use glm::{Vector2, Vector4};

use crate::{image::Image, utils};

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

                let mut color = self.per_pixel(coord);
                color = glm::clamp(color, glm::to_vec4(0.0), glm::to_vec4(1.0));
                self.image.set_pixel(x, y, utils::convert_to_rgba(&color)); // ABGR
            }
        }
    }

    fn per_pixel(&mut self, coord: Vector2<f32>) -> Vector4<f32> {
        let ray_direction = glm::vec3(coord.x, coord.y, -1.0);
        let ray_origin = glm::vec3(0.0, 0.0, 1.0);
        let radius = 0.5;

        let a = glm::dot(ray_direction, ray_direction);
        let b = 2.0 * glm::dot(ray_direction, ray_origin);
        let c = glm::dot(ray_origin, ray_origin) - radius * radius;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return glm::vec4(0.0, 0.0, 0.0, 1.0);
        }

        let closest_t = (-b - discriminant.sqrt()) / (2.0 * a);
        let t0 = (-b + discriminant.sqrt()) / (2.0 * a);

        let hit_point = ray_origin + ray_direction * closest_t;
        let normal = glm::normalize(hit_point);

        let light_dir = glm::normalize(glm::vec3(-1.0, -1.0, -1.0));
        let light_intensity = glm::max(glm::dot(normal, -light_dir), 0.0);

        let mut sphere_color = glm::vec3(1.0, 0.0, 1.0);
        sphere_color = sphere_color * light_intensity;

        glm::vec4(sphere_color.x, sphere_color.y, sphere_color.z, 1.0)
    }
}
