use glm::{Vector4};

use crate::{camera::Camera, image::Image, ray::Ray, utils};

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

    pub fn render(&mut self, camera: &Camera) {
        let mut ray = Ray {
            origin: camera.get_position().clone(),
            direction: glm::to_vec3(0.0),
        };

        for y in 0..self.image.get_height() {
            for x in 0..self.image.get_width() {
                ray.direction = camera.get_ray_directions()[x + y * self.final_image().get_width()];

                let mut color = self.trace_ray(&ray);
                color = glm::clamp(color, glm::to_vec4(0.0), glm::to_vec4(1.0));
                self.image.set_pixel(x, y, utils::convert_to_rgba(&color)); // ABGR
            }
        }
    }

    fn trace_ray(&mut self, ray: &Ray) -> Vector4<f32> {
        let radius = 0.5;

        let a = glm::dot(ray.direction, ray.direction);
        let b = 2.0 * glm::dot(ray.direction, ray.origin);
        let c = glm::dot(ray.origin, ray.origin) - radius * radius;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return glm::vec4(0.0, 0.0, 0.0, 1.0);
        }

        let closest_t = (-b - discriminant.sqrt()) / (2.0 * a);
        let _t0 = (-b + discriminant.sqrt()) / (2.0 * a);

        let hit_point = ray.origin + ray.direction * closest_t;
        let normal = glm::normalize(hit_point);

        let light_dir = glm::normalize(glm::vec3(-1.0, -1.0, -1.0));
        let light_intensity = glm::max(glm::dot(normal, -light_dir), 0.0);

        let mut sphere_color = glm::vec3(1.0, 0.0, 1.0);
        sphere_color = sphere_color * light_intensity;

        glm::vec4(sphere_color.x, sphere_color.y, sphere_color.z, 1.0)
    }
}
