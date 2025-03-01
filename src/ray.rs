#[derive(Default)]
pub struct Ray {
    pub origin: glam::Vec3,
    pub direction: glam::Vec3,
}

impl Ray {
    pub fn new(origin: glam::Vec3, direction: glam::Vec3) -> Ray {
        Ray { origin, direction }
    }

    pub fn at(&self, t: f32) -> glam::Vec3 {
        self.origin + t * self.direction
    }
}
