use crate::{
    interval::{interval, Interval},
    material::MaterialEnum,
    ray::Ray,
};

#[derive(Default, Clone)]
pub struct HitRecord {
    pub p: glam::Vec3,
    pub normal: glam::Vec3,
    pub mat: MaterialEnum,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: glam::Vec3) {
        self.front_face = r.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}

pub enum HittableEnum {
    Sphere(Sphere),
}

impl Hittable for HittableEnum {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        match self {
            HittableEnum::Sphere(sphere) => sphere.hit(r, ray_t, rec),
        }
    }
}

pub type HittableList = Vec<HittableEnum>;

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut tmp = HitRecord::default();
        let mut hit_anything = false;

        let mut closest_so_far = ray_t.max;

        for object in self.iter() {
            if object.hit(r, interval(ray_t.min, closest_so_far), &mut tmp) {
                hit_anything = true;
                closest_so_far = tmp.t;
                *rec = tmp.clone();
            }
        }
        hit_anything
    }
}

pub struct Sphere {
    pub center: glam::Vec3,
    pub radius: f32,
    pub mat: MaterialEnum,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
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
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat = self.mat;
        true
    }
}

pub fn sphere(center: glam::Vec3, radius: f32, mat: MaterialEnum) -> HittableEnum {
    HittableEnum::Sphere(Sphere {
        center,
        radius,
        mat,
    })
}
