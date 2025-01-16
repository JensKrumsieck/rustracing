pub type Color = glam::Vec3;

pub fn u8_color(color: Color) -> (u8, u8, u8) {
    (
        (color.x * 255.999) as u8,
        (color.y * 255.999) as u8,
        (color.z * 255.999) as u8,
    )
}
