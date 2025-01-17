use glam::vec3;
use std::f32::consts::PI;

pub mod camera;
pub mod color;
pub mod hittable;
pub mod interval;
pub mod material;
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

pub fn random_vec() -> glam::Vec3 {
    vec3(random_float(), random_float(), random_float())
}

pub fn random_vec_minmax(min: f32, max: f32) -> glam::Vec3 {
    vec3(
        random_float_minmax(min, max),
        random_float_minmax(min, max),
        random_float_minmax(min, max),
    )
}

pub fn random_unit_vec() -> glam::Vec3 {
    loop {
        let p = random_vec_minmax(-1.0, 1.0);
        let lensq = p.length_squared();

        if 1e-160 < lensq && lensq <= 1.0 {
            return p / lensq.sqrt();
        }
    }
}

pub fn random_on_hemisphere(normal: glam::Vec3) -> glam::Vec3 {
    let on_unit_sphere = random_unit_vec();
    if on_unit_sphere.dot(normal) > 0.0 {
        on_unit_sphere
    } else {
        -on_unit_sphere
    }
}

pub fn random_in_unit_disk() -> glam::Vec3 {
    loop {
        let p = glam::vec3(
            random_float_minmax(-1.0, 1.0),
            random_float_minmax(-1.0, 1.0),
            0.0,
        );
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn vec_near_zero(vec: glam::Vec3) -> bool {
    let s = 1e-8;
    (vec.x.abs() < s) && (vec.y.abs() < s) && (vec.z.abs() < s)
}
