use glm::Vector4;

use crate::{camera::Camera, image::Image, ray::Ray, scene::Scene, utils};

pub struct Renderer {
    image: Image,
}

struct Environment<'a> {
    active_scene: &'a Scene,
    active_camera: &'a Camera,
}

struct HitPayload {
    hit_distance: f32,
    world_normal: glm::Vector3<f32>,
    world_position: glm::Vector3<f32>,
    object_index: i32,
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
        let env = Environment {
            active_scene: scene,
            active_camera: camera,
        };

        for y in 0..self.image.get_height() {
            for x in 0..self.image.get_width() {
                let mut color = self.per_pixel(&env, x, y);
                color = glm::clamp(color, glm::to_vec4(0.0), glm::to_vec4(1.0));
                self.image.set_pixel(x, y, utils::convert_to_rgba(&color)); // ABGR
            }
        }
    }

    fn per_pixel(&mut self, env: &Environment, x: usize, y: usize) -> Vector4<f32> {
        // RayGen in Gpu architectures
        let mut ray = Ray {
            origin: env.active_camera.get_position().clone(),
            direction: env.active_camera.get_ray_directions()
                [x + y * self.final_image().get_width()],
        };

        let mut color = glm::to_vec3(0.0);
        let mut multiplier = 1.0;
        let bounces = 2;
        for _i in 0..bounces {
            let payload = self.trace_ray(env, &ray);
            if payload.hit_distance < 0.0 {
                let sky_color = glm::vec3(0.0, 0.0, 0.0);
                color = color + sky_color * multiplier;
                break;
            }

            let light_dir = glm::normalize(glm::vec3(-1.0, -1.0, -1.0));
            let light_intensity = glm::max(glm::dot(payload.world_normal, -light_dir), 0.0);

            let sphere = &env.active_scene.spheres[payload.object_index as usize];
            let mut sphere_color = env.active_scene.materials[sphere.material_index].albedo;
            sphere_color = sphere_color * light_intensity;
            color = color + sphere_color * multiplier;
            multiplier *= 0.7;

            // Shooting out another ray
            ray.origin = payload.world_position + payload.world_normal * 0.0001;
            ray.direction = glm::reflect(ray.direction, payload.world_normal);
        }

        glm::vec4(color.x, color.y, color.z, 1.0)
    }

    fn trace_ray(&mut self, env: &Environment, ray: &Ray) -> HitPayload {
        let mut object_index = -1;
        let mut hit_distance = f32::MAX;

        for (i, sphere) in env.active_scene.spheres.iter().enumerate() {
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

            if closest_t > 0.0 && closest_t < hit_distance {
                object_index = i as i32;
                hit_distance = closest_t;
            }
        }

        if object_index < 0 {
            return self.miss(ray);
        }

        self.closest_hit(env, ray, hit_distance, object_index)
    }

    fn closest_hit(
        &mut self,
        env: &Environment,
        ray: &Ray,
        hit_distance: f32,
        object_index: i32,
    ) -> HitPayload {
        let closest_sphere = &env.active_scene.spheres[object_index as usize];

        let origin = ray.origin - closest_sphere.position;
        let world_position = origin + ray.direction * hit_distance;
        let world_normal = glm::normalize(world_position);

        HitPayload {
            hit_distance,
            world_normal,
            world_position,
            object_index,
        }
    }

    fn miss(&mut self, _ray: &Ray) -> HitPayload {
        HitPayload {
            hit_distance: -1.0,
            world_normal: glm::to_vec3(0.0),
            world_position: glm::to_vec3(0.0),
            object_index: 0,
        }
    }
}
