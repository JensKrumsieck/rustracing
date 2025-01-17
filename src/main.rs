use rustracing::{
    camera::Camera,
    color::Color,
    hittable::{sphere, HittableList},
    material::{dielectric, lambertian, metal},
    random_float, random_vec, random_vec_minmax,
};
use std::{fs::File, io::BufWriter, path::Path, time::Instant};

fn main() {
    tracing_subscriber::fmt::init();

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1200;
    let samples_per_pixel = 500;
    let max_depth = 50;
    let vfov = 20.0;
    let mut camera = Camera::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        vfov,
    );

    camera.lookfrom = glam::vec3(13.0, 2.0, 3.0);
    camera.lookat = glam::vec3(0.0, 0.0, 0.0);
    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.0;

    // Materials
    let mat_ground = lambertian(Color::new(0.8, 0.8, 0.0));
    let mat_01 = dielectric(1.50);
    let mat_02 = lambertian(Color::new(0.4, 0.2, 0.1));
    let mat_03 = metal(Color::new(0.7, 0.6, 0.5), 0.5);

    // World
    let mut world: HittableList = vec![
        sphere(glam::vec3(0.0, -1000.0, 0.0), 1000.0, mat_ground),
        sphere(glam::vec3(0.0, 1.0, 0.0), 1.0, mat_01),
        sphere(glam::vec3(-4.0, 1.0, 0.0), 1.0, mat_02),
        sphere(glam::vec3(4.0, 1.0, 0.0), 1.0, mat_03),
    ];

    //generate scene
    for a in 0..11 {
        for b in 0..11 {
            let choose_mat = random_float();
            let center = glam::vec3(
                a as f32 + 0.9 * random_float(),
                0.2,
                b as f32 + 0.9 * random_float(),
            );

            if (center - glam::vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // take diffuse
                    let albedo: Color = random_vec() * random_vec();
                    let mat = lambertian(albedo);
                    world.push(sphere(center, 0.2, mat));
                } else if choose_mat < 0.95 {
                    // take metal
                    let albedo: Color = random_vec_minmax(0.5, 1.0);
                    let fuzz = random_float();
                    let mat = metal(albedo, fuzz);
                    world.push(sphere(center, 0.2, mat));
                } else {
                    //take glass
                    let mat = dielectric(1.50);
                    world.push(sphere(center, 0.2, mat))
                }
            }
        }
    }

    tracing::info!(
        "Rendering Image with width: {image_width} & height: {}",
        (image_width as f32 / aspect_ratio) as u32
    );

    let path = Path::new("output.png");
    let file = File::create(path).unwrap_or_else(|_| panic!("Could not create File {path:?}"));
    let w = &mut BufWriter::new(file);
    tracing::debug!("Allocated image at {path:?}");

    tracing::info!("Started rendering");
    let clock = Instant::now();
    camera.render(&world, w).expect("Rendering failed!");
    tracing::info!("Finished rendering in {:.0?}", clock.elapsed());
}
