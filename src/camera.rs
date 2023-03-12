use glm::{Matrix4, Vector2, Vector3};

use crate::utils;

pub struct Camera {
    projection: Matrix4<f32>,
    view: Matrix4<f32>,
    inverse_projection: Matrix4<f32>,
    inverse_view: Matrix4<f32>,
    vertical_fov: f32,
    near_clip: f32,
    far_clip: f32,
    position: Vector3<f32>,
    forward_direction: Vector3<f32>,
    ray_directions: Vec<Vector3<f32>>,
    _last_mouse_position: Vector2<f32>,
    viewport_width: usize,
    viewport_height: usize,
}

impl Camera {
    pub fn new(vertical_fov: f32, near_clip: f32, far_clip: f32) -> Self {
        Self {
            projection: utils::identity_mat4(),
            view: utils::identity_mat4(),
            inverse_projection: utils::identity_mat4(),
            inverse_view: utils::identity_mat4(),
            vertical_fov,
            near_clip,
            far_clip,
            position: glm::vec3(0.0, 0.0, 6.0),
            forward_direction: glm::vec3(0.0, 0.0, -1.0),
            ray_directions: Vec::new(),
            _last_mouse_position: glm::to_vec2(0.0),
            viewport_width: 0,
            viewport_height: 0,
        }
    }

    pub fn on_update(&mut self, _ts: f32) {
        let moved: bool = false;

        // TODO: Add support for movement. First need Input

        if moved {
            self.recalculate_view();
            self.recalculate_ray_directions();
        }
    }

    pub fn on_resize(&mut self, width: usize, height: usize) {
        if width == self.viewport_width && height == self.viewport_height {
            return;
        }

        self.viewport_width = width;
        self.viewport_height = height;

        self.recalculate_projection();
        self.recalculate_ray_directions();
    }

    pub fn _get_projection(&self) -> &Matrix4<f32> {
        &self.projection
    }

    pub fn _get_inverse_projection(&self) -> &Matrix4<f32> {
        &self.inverse_projection
    }

    pub fn _get_view(&self) -> &Matrix4<f32> {
        &self.view
    }

    pub fn _get_inverse_view(&self) -> &Matrix4<f32> {
        &self.projection
    }

    pub fn get_position(&self) -> &Vector3<f32> {
        &self.position
    }

    pub fn _get_direction(&self) -> &Vector3<f32> {
        &self.forward_direction
    }

    pub fn get_ray_directions(&self) -> &Vec<Vector3<f32>> {
        &self.ray_directions
    }

    pub fn _get_rotation_speed() -> f32 {
        0.3
    }

    fn recalculate_projection(&mut self) {
        let aspect = self.viewport_width as f32 / self.viewport_height as f32;
        self.projection = glm::ext::perspective(
            glm::radians(self.vertical_fov),
            aspect,
            self.near_clip,
            self.far_clip,
        );
        self.inverse_projection = glm::inverse(&self.projection);
    }

    fn recalculate_view(&mut self) {
        self.view = glm::ext::look_at(
            self.position,
            self.position + self.forward_direction,
            glm::vec3(0.0, 1.0, 0.0),
        );
        self.inverse_view = glm::inverse(&self.view);
    }

    fn recalculate_ray_directions(&mut self) {
        self.ray_directions.clear();
        for y in 0..self.viewport_height {
            for x in 0..self.viewport_width {
                let mut coord = glm::vec2(
                    x as f32 / self.viewport_width as f32,
                    y as f32 / self.viewport_height as f32,
                );
                coord = coord * 2.0 - 1.0;

                let target = self.inverse_projection * glm::vec4(coord.x, coord.y, 1.0, 1.0);
                let target_vec3 = glm::vec3(target.x, target.y, target.z);
                let normalized_target = glm::normalize(target_vec3 / target.w);

                let ray_direction = self.inverse_view
                    * glm::vec4(
                        normalized_target.x,
                        normalized_target.y,
                        normalized_target.z,
                        0.0,
                    );
                let ray_direction_vec3 =
                    glm::vec3(ray_direction.x, ray_direction.y, ray_direction.z);

                self.ray_directions.push(ray_direction_vec3);
            }
        }
    }
}
