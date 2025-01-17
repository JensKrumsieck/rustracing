use crate::{
    color::Color, hittable::HitRecord, random_float, random_unit_vec, ray::Ray, vec_near_zero,
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
    Dielectric(Dielectric),
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
            MaterialEnum::Dielectric(dielectric) => {
                dielectric.scatter(r_in, rec, attenuation, scattered)
            }
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
    pub fuzz: f32,
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected =
            reflect(r_in.direction, rec.normal).normalize() + (self.fuzz * random_unit_vec());
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;

        scattered.direction.dot(rec.normal) > 0.0
    }
}

pub fn metal(albedo: Color, fuzz: f32) -> MaterialEnum {
    MaterialEnum::Metal(Metal { albedo, fuzz })
}

#[derive(Default, Clone, Copy)]
pub struct Dielectric {
    pub refraction_index: f32,
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = r_in.direction.normalize();
        let cos_theta = (-unit_direction).dot(rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let direction = if cannot_refract || reflectance(cos_theta, ri) > random_float() {
            reflect(unit_direction, rec.normal)
        } else {
            refract(unit_direction, rec.normal, ri)
        };

        *scattered = Ray::new(rec.p, direction);

        true
    }
}

pub fn dielectric(refraction_index: f32) -> MaterialEnum {
    MaterialEnum::Dielectric(Dielectric { refraction_index })
}

pub fn reflect(v: glam::Vec3, n: glam::Vec3) -> glam::Vec3 {
    v - 2.0 * v.dot(n) * n
}

pub fn refract(uv: glam::Vec3, n: glam::Vec3, etai_over_etat: f32) -> glam::Vec3 {
    let cos_theta = (-uv).dot(n).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_par = -(1.0 - r_out_perp.length_squared()).sqrt() * n;
    r_out_perp + r_out_par
}

pub fn reflectance(cosine: f32, refraction_index: f32) -> f32 {
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r0sq = r0 * r0;

    r0sq + (1.0 - r0sq) * (1.0 - cosine).powf(5.0)
}
