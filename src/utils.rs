use glm::Vector4;

use crate::image::Color;

pub fn convert_to_rgba(color: &Vector4<f32>) -> Color {
    let r = (color.x * 255.0) as u32;
    let g = ((color.y * 255.0) as u32) << 8;
    let b = ((color.z * 255.0) as u32) << 16;
    let a = ((color.w * 255.0) as u32) << 24;
    r | g | b | a
}
