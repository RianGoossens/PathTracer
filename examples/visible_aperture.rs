use std::time::Instant;

use path_tracer::{
    aperture::RegularPolygonAperture, BackwardRenderer, Camera, Material, Object, Renderer, Scene,
    Sphere,
};

use nalgebra as na;

use na::{Similarity3, Vector3};

const NUM_SAMPLES: u32 = 10000;

fn main() {
    let aperture = RegularPolygonAperture::new(1., 3);
    let focal_distance = 3.5;
    let camera = Camera::new(300, 300, 55., 1.0, 100.0, aperture, focal_distance);

    let sphere_shape = Sphere;

    let light = Object::new(
        sphere_shape,
        na::convert(Similarity3::new(
            Vector3::new(0., 0., -5.),
            Vector3::zeros(),
            0.25,
        )),
        Material {
            color: Vector3::new(3., 3., 3.),
            emissive: true,
            ..Default::default()
        },
    );

    let scene = Scene::new(camera, vec![light]);

    let start = Instant::now();
    let renderer = BackwardRenderer::new(5).parallel(NUM_SAMPLES);
    let render_buffer = renderer.render(&scene);

    println!("Rendering took {:?}", start.elapsed());

    let image = render_buffer.to_image();

    image.save("image.png").expect("Could not save image");
}
