use std::time::Instant;

use path_tracer::{
    aperture::PinholeAperture, renderer::BDPTRenderer, Camera, Inverted, Material, Object,
    Renderer, Scene, Sphere,
};

use nalgebra as na;

use na::{Similarity3, Vector3};

const NUM_SAMPLES: usize = 2000;

fn main() {
    let camera = Camera::new(300, 300, 55., 1.0, 100.0, PinholeAperture, 0.);

    let sphere_shape = Sphere::new(1.);

    let sphere_a = Object::new(
        sphere_shape,
        Similarity3::new(Vector3::new(1.5, 1., -5.), Vector3::zeros(), 1.),
        Material {
            color: Vector3::new(0.8, 0.6, 0.7),
            roughness: 0.1,
            ..Default::default()
        },
    );

    let sphere_b = Object::new(
        sphere_shape,
        Similarity3::new(Vector3::new(-1.5, 1., -5.), Vector3::zeros(), 1.),
        Material {
            color: Vector3::new(0.4, 0.8, 0.3),
            roughness: 0.9,
            ..Default::default()
        },
    );

    let environment = Object::new(
        Inverted(Sphere::new(1.)),
        Similarity3::new(Vector3::new(0., 0., 0.), Vector3::zeros(), 5.),
        Material {
            color: Vector3::new(0.6, 0.7, 0.5) * 0.7,
            roughness: 0.9,
            ..Default::default()
        },
    );

    let light = Object::new(
        sphere_shape,
        Similarity3::new(Vector3::new(0., -0.5, -5.), Vector3::zeros(), 0.75),
        Material {
            color: Vector3::new(1., 1., 1.) * 3.,
            emissive: true,
            ..Default::default()
        },
    );

    let scene = Scene::new(camera, vec![sphere_a, sphere_b, environment, light]);

    let start = Instant::now();

    let renderer = BDPTRenderer::new(10).parallel(NUM_SAMPLES);
    let render_buffer = renderer.render(&scene);

    println!("Rendering took {:?}", start.elapsed());

    let image = render_buffer.to_image();

    image.save("image.png").expect("Could not save image");
}
