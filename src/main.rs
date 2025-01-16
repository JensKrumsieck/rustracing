use core::f32;
use rustracing::{
    color::{u8_color, Color},
    hittable::{HitRecord, Hittable, Sphere},
    ray::Ray,
};
use std::{
    fs::File,
    io::{self, BufWriter, Write},
    path::Path,
    time::Instant,
};

fn main() {
    tracing_subscriber::fmt::init();

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f32 / aspect_ratio) as u32;

    // Camera
    let focal_lenght = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f32 / image_height as f32);
    let camera_center = glam::vec3(0.0, 0.0, 0.0);

    // calculate vectors acrooss the horizontal and vertical viewport edges
    let viewport_u = glam::vec3(viewport_width, 0.0, 0.0);
    let viewport_v = glam::vec3(0.0, -viewport_height, 0.0);

    // calculate horizontal and vertical delta vectors
    let pixel_delta_u = viewport_u / image_width as f32;
    let pixel_delta_v = viewport_v / image_height as f32;

    //calulatoe upper left pixel
    let viewport_upper_left =
        camera_center - glam::vec3(0.0, 0.0, focal_lenght) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    tracing::info!("Rendering Image with width: {image_width} & height:{image_height}");

    let path = Path::new("output.png");
    let file = File::create(path).unwrap_or_else(|_| panic!("Could not create File {path:?}"));
    let w = &mut BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, image_width, image_height);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().expect("Could not write png header");
    tracing::debug!("Allocated image at {path:?}");

    let mut data = vec![0u8; (image_width * image_height * 3) as usize];

    tracing::info!("Started rendering");
    let clock = Instant::now();

    for y in 0..image_height {
        print!("\t\t\tRemaining lines: {}   \r", image_height - y);
        io::stdout().flush().unwrap();
        for x in 0..image_width {
            let index = ((y * image_width + x) * 3) as usize;

            //rendering here
            let pixel_center =
                pixel00_loc + (x as f32 * pixel_delta_u) + (y as f32 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);
            let pixel_color = ray_color(&r);
            //end rendering

            let pixels = u8_color(pixel_color);
            data[index] = pixels.0;
            data[index + 1] = pixels.1;
            data[index + 2] = pixels.2;
        }
    }

    tracing::info!("Finished rendering in {:.0?}", clock.elapsed());
    writer
        .write_image_data(&data)
        .expect("Could not write image data");
    tracing::debug!("Wrote image to disk");
}

fn ray_color(r: &Ray) -> Color {
    let s = Sphere {
        center: glam::vec3(0.0, 0.0, -1.0),
        radius: 0.5,
    };
    let mut rec = HitRecord::default();
    if s.hit(r, 0.0, f32::INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction = r.direction.normalize();
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}
