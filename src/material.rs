use crate::{
    color::Color,
    hittable::HitRecord,
    random_unit_vec,
    ray::Ray,
    reflect, vec_near_zero,
};

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

#[derive(Clone, Copy)]
pub enum MaterialEnum {
    Lambertian(Lambertian),
    Metal(Metal),
}

impl Default for MaterialEnum {
    fn default() -> Self {
        MaterialEnum::Lambertian(Lambertian::default())
    }
}

impl Material for MaterialEnum {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        match self {
            MaterialEnum::Lambertian(lambertian) => {
                lambertian.scatter(r_in, rec, attenuation, scattered)
            }
            MaterialEnum::Metal(metal) => metal.scatter(r_in, rec, attenuation, scattered),
        }
    }
}

#[derive(Default, Clone, Copy)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let _ = r_in;
        let mut scatter_direction = rec.normal + random_unit_vec();

        if vec_near_zero(scatter_direction) {
            scatter_direction = rec.normal
        }

        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

pub fn lambertian(albedo: Color) -> MaterialEnum {
    MaterialEnum::Lambertian(Lambertian { albedo })
}

#[derive(Default, Clone, Copy)]
pub struct Metal {
    pub albedo: Color,
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(r_in.direction, rec.normal);
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        true
    }
}

pub fn metal(albedo: Color) -> MaterialEnum {
    MaterialEnum::Metal(Metal { albedo })
}
