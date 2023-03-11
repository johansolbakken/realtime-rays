use glm::Vector4;

use crate::{camera::Camera, image::Image, ray::Ray, scene::Scene, utils};

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

    pub fn render(&mut self, scene: &Scene, camera: &Camera) {
        let mut ray = Ray {
            origin: camera.get_position().clone(),
            direction: glm::to_vec3(0.0),
        };

        for y in 0..self.image.get_height() {
            for x in 0..self.image.get_width() {
                ray.direction = camera.get_ray_directions()[x + y * self.final_image().get_width()];

                let mut color = self.trace_ray(scene, &ray);
                color = glm::clamp(color, glm::to_vec4(0.0), glm::to_vec4(1.0));
                self.image.set_pixel(x, y, utils::convert_to_rgba(&color)); // ABGR
            }
        }
    }

    fn trace_ray(&mut self, scene: &Scene, ray: &Ray) -> Vector4<f32> {
        if scene.spheres.len() == 0 {
            return glm::vec4(0.0, 0.0, 0.0, 1.0);
        }

        let mut closest_sphere_index = -1;
        let mut hit_distance = f32::MAX;

        for (i, sphere) in scene.spheres.iter().enumerate() {
            let origin = ray.origin - sphere.position;

            let a = glm::dot(ray.direction, ray.direction);
            let b = 2.0 * glm::dot(ray.direction, origin);
            let c = glm::dot(origin, origin) - sphere.radius * sphere.radius;

            let discriminant = b * b - 4.0 * a * c;
            if discriminant < 0.0 {
                continue;
            }

            let closest_t = (-b - discriminant.sqrt()) / (2.0 * a);
            // let t0 = (-b + discriminant.sqrt()) / (2.0 * a); // Second hit point. Will us this later.

            if closest_t < hit_distance {
                closest_sphere_index = i as i32;
                hit_distance = closest_t;
            }
        }

        if closest_sphere_index < 0 {
            return glm::vec4(0.0, 0.0, 0.0, 1.0);
        }

        let closest_sphere = &scene.spheres[closest_sphere_index as usize];
        let origin = ray.origin - closest_sphere.position;

        let hit_point = origin + ray.direction * hit_distance;
        let normal = glm::normalize(hit_point);

        let light_dir = glm::normalize(glm::vec3(-1.0, -1.0, -1.0));
        let light_intensity = glm::max(glm::dot(normal, -light_dir), 0.0);

        let sphere_color = closest_sphere.albedo * light_intensity;
        
        glm::vec4(sphere_color.x, sphere_color.y, sphere_color.z, 1.0)
    }
}
