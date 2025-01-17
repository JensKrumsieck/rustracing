use crate::interval::interval;

pub type Color = glam::Vec3;

pub fn u8_color(color: Color) -> (u8, u8, u8) {
    let intensity = interval(0.000, 0.999);
    (
        (intensity.clamp(color.x) * 255.999) as u8,
        (intensity.clamp(color.y) * 255.999) as u8,
        (intensity.clamp(color.z) * 255.999) as u8,
    )
}
