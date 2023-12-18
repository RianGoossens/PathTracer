use std::time::Instant;

use path_tracer::{
    aperture::PinholeAperture, BackwardRenderer, Camera, Inverted, Material, Object, Renderer,
    Scene, Sphere,
};

use nalgebra as na;

use na::{Similarity3, Vector3};

const NUM_SAMPLES: u32 = 100;

fn main() {
    let camera = Camera::new(300, 300, 55., 1.0, 100.0, PinholeAperture, 0.);

    let sphere_shape = Sphere;

    let sphere_a = Object::new(
        sphere_shape,
        na::convert(Similarity3::new(
            Vector3::new(1.5, 1., -5.),
            Vector3::zeros(),
            1.,
        )),
        Material {
            color: Vector3::new(0.8, 0.6, 0.7),
            roughness: 0.1,
            ..Default::default()
        },
    );

    let sphere_b = Object::new(
        sphere_shape,
        na::convert(Similarity3::new(
            Vector3::new(-1.5, 1., -5.),
            Vector3::zeros(),
            1.,
        )),
        Material {
            color: Vector3::new(0.4, 0.8, 0.3),
            roughness: 0.8,
            ..Default::default()
        },
    );

    let sphere_c = Object::new(
        Inverted(Sphere),
        na::convert(Similarity3::new(Vector3::zeros(), Vector3::zeros(), 5.)),
        Material {
            color: Vector3::new(0.6, 0.7, 0.5),
            roughness: 0.9,
            ..Default::default()
        },
    );

    let light = Object::new(
        sphere_shape,
        na::convert(Similarity3::new(
            Vector3::new(0., -0.5, -5.),
            Vector3::zeros(),
            0.75,
        )),
        Material {
            color: Vector3::new(3., 3., 3.),
            emissive: true,
            ..Default::default()
        },
    );

    let scene = Scene::new(camera, vec![sphere_a, sphere_b, sphere_c, light]);

    let start = Instant::now();
    let renderer = BackwardRenderer::new(5).parallel(NUM_SAMPLES);
    let render_buffer = renderer.render(&scene);

    println!("Rendering took {:?}", start.elapsed());

    let image = render_buffer.to_image();

    image.save("image.png").expect("Could not save image");
}
