use std::{
    fs::File,
    io::{self, BufWriter, Write},
};

use crate::{
    color::{u8_color, Color},
    hittable::{HitRecord, Hittable, HittableList},
    interval::interval,
    ray::Ray,
};

#[derive(Debug)]
pub struct Camera {
    pub aspect_ratio: f32,
    pub image_width: u32,

    image_height: u32,
    center: glam::Vec3,
    pixel00_loc: glam::Vec3,
    pixel_delta_u: glam::Vec3,
    pixel_delta_v: glam::Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f32, image_width: u32) -> Self {
        Camera {
            aspect_ratio,
            image_width,
            image_height: Default::default(),
            center: Default::default(),
            pixel00_loc: Default::default(),
            pixel_delta_u: Default::default(),
            pixel_delta_v: Default::default(),
        }
    }

    /// Get Image dimensions as Tuple (Width, Height)
    pub fn get_dimensions(&self) -> (u32, u32) {
        (self.image_width, self.image_height)
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

        let mut data = vec![0u8; (self.image_width * self.image_height * 3) as usize];
        for y in 0..self.image_height {
            for x in 0..self.image_width {
                let index = ((y * self.image_width + x) * 3) as usize;

                //rendering here
                let pixel_center = self.pixel00_loc
                    + (x as f32 * self.pixel_delta_u)
                    + (y as f32 * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let r = Ray::new(self.center, ray_direction);
                let pixel_color = ray_color(&r, world);
                //end rendering

                let pixels = u8_color(pixel_color);
                data[index] = pixels.0;
                data[index + 1] = pixels.1;
                data[index + 2] = pixels.2;
            }
        }
        writer.write_image_data(&data)?;

        Ok(())
    }

    pub fn init(&mut self) {
        self.image_height = (self.image_width as f32 / self.aspect_ratio) as u32;

        let focal_lenght = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f32 / self.image_height as f32);
        self.center = glam::vec3(0.0, 0.0, 0.0);
        // calculate vectors acrooss the horizontal and vertical viewport edges
        let viewport_u = glam::vec3(viewport_width, 0.0, 0.0);
        let viewport_v = glam::vec3(0.0, -viewport_height, 0.0);
        // calculate horizontal and vertical delta vectors
        self.pixel_delta_u = viewport_u / self.image_width as f32;
        self.pixel_delta_v = viewport_v / self.image_height as f32;

        //calulatoe upper left pixel
        let viewport_upper_left =
            self.center - glam::vec3(0.0, 0.0, focal_lenght) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }
}
fn ray_color(r: &Ray, world: &HittableList) -> Color {
    let mut rec = HitRecord::default();
    if world.hit(r, interval(0.0, f32::INFINITY), &mut rec) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction = r.direction.normalize();
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}
