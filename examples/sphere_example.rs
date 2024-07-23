use std::time::Instant;

use path_tracer::{
    aperture::GaussianAperture, object::ObjectDefinition, renderer::RecursiveBDPT, Camera,
    Inverted, Material, Renderer, Scene, Sphere,
};

use nalgebra as na;

use na::Vector3;

const NUM_SAMPLES: usize = 100;

fn main() {
    let aperture = GaussianAperture::new(1.);
    let camera = Camera::new_at_origin(300, 300, 55., 1.0, 100.0, aperture, 5.);

    let sphere_shape = Sphere::new(1.);

    let sphere_a = ObjectDefinition {
        shape: Box::new(sphere_shape),
        material: Material::new(Vector3::new(0.8, 0.6, 0.7), 0.01, false),
        x: 1.5,
        y: 1.,
        z: -5.,
        ..Default::default()
    };

    let sphere_b = ObjectDefinition {
        shape: Box::new(sphere_shape),
        material: Material::new(Vector3::new(0.4, 0.85, 0.3), 0.9, false),
        x: -1.5,
        y: 1.,
        z: -5.,
        ..Default::default()
    };

    let environment = ObjectDefinition {
        shape: Box::new(Inverted(Sphere::new(1.))),
        material: Material::new(Vector3::new(0.6, 0.75, 0.5) * 0.5, 0.8, false),
        scale: 5.,
        ..Default::default()
    };

    let light = ObjectDefinition {
        shape: Box::new(sphere_shape),
        material: Material::new(Vector3::new(1., 1., 1.) * 1., 1.0, true),
        y: -0.5,
        z: -5.,
        scale: 0.75,
        ..Default::default()
    };

    let scene = Scene::new(camera, vec![sphere_a, sphere_b, environment, light]);

    let start = Instant::now();

    let renderer = RecursiveBDPT::new(10).parallel(NUM_SAMPLES);
    let render_buffer = renderer.render(&scene);

    println!("Rendering took {:?}", start.elapsed());

    let image = render_buffer.srgb().to_image_u8();

    image.save("image.png").expect("Could not save image");
}
