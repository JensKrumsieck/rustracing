use crate::interval::interval;

pub type Color = glam::Vec3;

pub fn u8_color(color: Color) -> (u8, u8, u8) {
    let intensity = interval(0.000, 0.999);
    (
        (intensity.clamp(linear_to_gamma(color.x)) * 255.999) as u8,
        (intensity.clamp(linear_to_gamma(color.y)) * 255.999) as u8,
        (intensity.clamp(linear_to_gamma(color.z)) * 255.999) as u8,
    )
}

fn linear_to_gamma(linear_component: f32) -> f32 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }
    0.0
}
