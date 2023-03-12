use glm::Vector4;
use rand::Rng;

use crate::image::Color;

pub fn convert_to_rgba(color: &Vector4<f32>) -> Color {
    let r = (color.x * 255.0) as u32;
    let g = ((color.y * 255.0) as u32) << 8;
    let b = ((color.z * 255.0) as u32) << 16;
    let a = ((color.w * 255.0) as u32) << 24;
    r | g | b | a
}

pub fn identity_mat4() -> glm::Mat4 {
    glm::mat4(
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    )
}

pub fn random_range(min: f32, max: f32) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(min..max)
}

pub fn random_range_vec3(min: f32, max: f32) -> glm::Vector3<f32> {
    glm::vec3(
        random_range(min, max),
        random_range(min, max),
        random_range(min, max),
    )
}
