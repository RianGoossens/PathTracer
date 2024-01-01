use std::time::Instant;

use path_tracer::{
    aperture::GaussianAperture,
    renderer::{BDPTRenderer, RecursiveBDPT},
    BackwardRenderer, Camera, Inverted, Material, Object, Renderer, Scene, Sphere,
};

use nalgebra as na;

use na::{Similarity3, Vector3};

const NUM_SAMPLES: usize = 100;

fn main() {
    let aperture = GaussianAperture::new(1.);
    let camera = Camera::new_at_origin(300, 300, 55., 1.0, 100.0, aperture, 5.);

    let sphere_shape = Sphere::new(1.);

    let sphere_a = Object::new(
        sphere_shape,
        Similarity3::new(Vector3::new(1.5, 1., -5.), Vector3::zeros(), 1.),
        Material::new(Vector3::new(0.8, 0.6, 0.7), 0.01, false),
    );

    let sphere_b = Object::new(
        sphere_shape,
        Similarity3::new(Vector3::new(-1.5, 1., -5.), Vector3::zeros(), 1.),
        Material::new(Vector3::new(0.4, 0.85, 0.3), 0.9, false),
    );

    let environment = Object::new(
        Inverted(Sphere::new(1.)),
        Similarity3::new(Vector3::new(0., 0., 0.), Vector3::zeros(), 5.),
        Material::new(Vector3::new(0.6, 0.75, 0.5) * 0.5, 0.8, false),
    );

    let light = Object::new(
        sphere_shape,
        Similarity3::new(Vector3::new(0., -0.5, -5.), Vector3::zeros(), 0.75),
        Material::new(Vector3::new(1., 1., 1.) * 1., 1.0, true),
    );

    let scene = Scene::new(camera, vec![sphere_a, sphere_b, environment, light]);

    let start = Instant::now();

    let renderer = RecursiveBDPT::new(10).parallel(NUM_SAMPLES);
    let render_buffer = renderer.render(&scene);

    println!("Rendering took {:?}", start.elapsed());

    let image = render_buffer.srgb().to_image_u8();

    image.save("image.png").expect("Could not save image");
}
