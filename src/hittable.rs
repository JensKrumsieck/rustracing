use crate::ray::Ray;

#[derive(Default)]
pub struct HitRecord {
    pub p: glam::Vec3,
    pub normal: glam::Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(self: &mut Self, r: &Ray, outward_normal: glam::Vec3) {
        self.front_face = r.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(self: Self, r: &Ray, ray_tmin: f32, ray_tmax: f32, rec: &mut HitRecord) -> bool;
}

pub struct Sphere {
    pub center: glam::Vec3,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn hit(self: Self, r: &Ray, ray_tmin: f32, ray_tmax: f32, rec: &mut HitRecord) -> bool {
        let oc = self.center - r.origin;
        let a = r.direction.length_squared();
        let h = r.direction.dot(oc);
        let c = oc.length_squared() - (self.radius * self.radius);

        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;
        if root <= ray_tmin || ray_tmax <= root {
            root = (h + sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

        true
    }
}
