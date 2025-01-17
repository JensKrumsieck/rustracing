use rustracing::{
    camera::Camera,
    hittable::{sphere, HittableList},
};
use std::{fs::File, io::BufWriter, path::Path, time::Instant};

fn main() {
    tracing_subscriber::fmt::init();

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 10;
    let mut camera = Camera::new(aspect_ratio, image_width, samples_per_pixel, max_depth);
    // World
    let world: HittableList = vec![
        sphere(glam::vec3(0.0, 0.0, -1.0), 0.5),
        sphere(glam::vec3(0.0, -100.5, -1.0), 100.0),
    ];

    tracing::info!(
        "Rendering Image with width: {image_width} & height: {}",
        (image_width as f32 * aspect_ratio) as u32
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
