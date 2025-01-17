use std::f32::consts::PI;

pub mod camera;
pub mod color;
pub mod hittable;
pub mod interval;
pub mod ray;

pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * PI / 180.0
}

pub fn random_float() -> f32 {
    rand::random()
}

pub fn random_float_minmax(min: f32, max: f32) -> f32 {
    min + rand::random::<f32>() * (max - min)
}
