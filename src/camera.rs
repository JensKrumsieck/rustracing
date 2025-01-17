use crate::{
    color::{u8_color, Color},
    degrees_to_radians,
    hittable::{HitRecord, Hittable, HittableList},
    interval::interval,
    material::Material,
    random_float, random_in_unit_disk,
    ray::Ray,
};
use indicatif::ProgressBar;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{fs::File, io::BufWriter, sync::Mutex};

#[derive(Debug)]
pub struct Camera {
    pub aspect_ratio: f32,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: i32,
    pub vfov: f32,
    pub lookfrom: glam::Vec3,
    pub lookat: glam::Vec3,
    pub vup: glam::Vec3,
    pub defocus_angle: f32,
    pub focus_dist: f32,

    image_height: u32,
    center: glam::Vec3,
    pixel00_loc: glam::Vec3,
    pixel_delta_u: glam::Vec3,
    pixel_delta_v: glam::Vec3,
    pixel_samples_scale: f32,
    u: glam::Vec3,
    v: glam::Vec3,
    w: glam::Vec3,
    defocus_disk_u: glam::Vec3,
    defocus_disk_v: glam::Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f32,
        image_width: u32,
        samples_per_pixel: u32,
        max_depth: i32,
        vfov: f32,
    ) -> Self {
        Camera {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            vfov,
            lookfrom: Default::default(),
            lookat: Default::default(),
            vup: glam::vec3(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            image_height: Default::default(),
            center: Default::default(),
            pixel00_loc: Default::default(),
            pixel_delta_u: Default::default(),
            pixel_delta_v: Default::default(),
            pixel_samples_scale: Default::default(),
            u: Default::default(),
            v: Default::default(),
            w: Default::default(),
            defocus_disk_u: Default::default(),
            defocus_disk_v: Default::default(),
        }
    }

    pub fn render(
        &mut self,
        world: &HittableList,
        handle: &mut BufWriter<File>,
    ) -> anyhow::Result<()> {
        self.init();

        let mut encoder = png::Encoder::new(handle, self.image_width, self.image_height);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header()?;

        let data = vec![0u8; (self.image_width * self.image_height * 3) as usize];
        let data = Mutex::new(data);
        let pb = ProgressBar::new(self.image_height as u64 * self.image_width as u64);
        (0..self.image_height).into_par_iter().for_each(|y| {
            for x in 0..self.image_width {
                let index = ((y * self.image_width + x) * 3) as usize;

                //rendering here
                let mut pixel_color = glam::vec3(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(x, y);
                    pixel_color += ray_color(&r, self.max_depth, world);
                }
                //end rendering

                let pixels = u8_color(pixel_color * self.pixel_samples_scale);
                let mut data = data.lock().unwrap();
                data[index] = pixels.0;
                data[index + 1] = pixels.1;
                data[index + 2] = pixels.2;
                pb.inc(1);
            }
        });
        pb.finish_with_message("Done.");
        writer.write_image_data(&data.lock().unwrap())?;

        Ok(())
    }

    pub fn init(&mut self) {
        self.image_height = (self.image_width as f32 / self.aspect_ratio) as u32;

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f32;

        self.center = self.lookfrom;

        let theta = degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f32 / self.image_height as f32);

        // calculate unit basis vectors
        self.w = (self.lookfrom - self.lookat).normalize();
        self.u = self.vup.cross(self.w).normalize();
        self.v = self.w.cross(self.u);

        // calculate vectors acrooss the horizontal and vertical viewport edges
        let viewport_u = viewport_width * self.u;
        let viewport_v = viewport_height * -self.v;

        // calculate horizontal and vertical delta vectors
        self.pixel_delta_u = viewport_u / self.image_width as f32;
        self.pixel_delta_v = viewport_v / self.image_height as f32;

        //calulatoe upper left pixel
        let viewport_upper_left =
            self.center - (self.focus_dist * self.w) - (viewport_u / 2.0) - (viewport_v / 2.0);

        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        //defocus
        let defocus_radius = self.focus_dist * (degrees_to_radians(self.defocus_angle / 2.0)).tan();
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = sample_square();

        let pixel_sample = self.pixel00_loc
            + ((i as f32 + offset.x) * self.pixel_delta_u)
            + ((j as f32 + offset.y) * self.pixel_delta_v);

        let origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let direction = pixel_sample - origin;

        Ray { origin, direction }
    }

    fn defocus_disk_sample(&self) -> glam::Vec3 {
        let p = random_in_unit_disk();
        self.center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }
}
fn ray_color(r: &Ray, depth: i32, world: &HittableList) -> Color {
    if depth < 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    let mut rec = HitRecord::default();
    if world.hit(r, interval(0.001, f32::INFINITY), &mut rec) {
        let mut scattered = Ray::default();
        let mut attenuation = Color::default();
        if rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(&scattered, depth - 1, world);
        }
        return Color::default();
    }

    let unit_direction = r.direction.normalize();
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn sample_square() -> glam::Vec3 {
    glam::vec3(random_float() - 0.5, random_float() - 0.5, 0.0)
}
