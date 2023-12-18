use std::time::Instant;

use path_tracer::{
    aperture::RegularPolygonAperture, BackwardRenderer, Camera, Inverted, Material, Object,
    Renderer, Scene, Sphere,
};

use nalgebra as na;

use na::{Similarity3, Vector3};

const NUM_SAMPLES: usize = 1000;

fn main() {
    let aperture = RegularPolygonAperture::new(1., 6);
    let camera = Camera::new(300, 300, 70., 1.0, 100.0, aperture, 5.);

    let sphere_shape = Sphere;

    let sphere_a = Object::new(
        sphere_shape,
        na::convert(Similarity3::new(
            Vector3::new(1.5, -0.5, -5.),
            Vector3::zeros(),
            1.,
        )),
        Material {
            color: Vector3::new(0.8, 0.1, 0.1),
            roughness: 0.9,
            ..Default::default()
        },
    );

    let sphere_b = Object::new(
        sphere_shape,
        na::convert(Similarity3::new(
            Vector3::new(1., 0., -6.),
            Vector3::zeros(),
            1.,
        )),
        Material {
            color: Vector3::new(0.1, 0.8, 0.1),
            roughness: 0.9,
            ..Default::default()
        },
    );

    let sphere_c = Object::new(
        sphere_shape,
        na::convert(Similarity3::new(
            Vector3::new(0.5, 0.5, -7.),
            Vector3::zeros(),
            1.,
        )),
        Material {
            color: Vector3::new(0.1, 0.1, 0.8),
            roughness: 0.9,
            ..Default::default()
        },
    );

    let light = Object::new(
        sphere_shape,
        na::convert(Similarity3::new(
            Vector3::new(-1.5, 0., -6.),
            Vector3::zeros(),
            1.,
        )),
        Material {
            color: Vector3::new(6., 6., 6.),
            emissive: true,
            roughness: 1.,
        },
    );

    let environment = Object::new(
        Inverted(Sphere),
        na::convert(Similarity3::new(
            Vector3::new(0., 0., -6.),
            Vector3::zeros(),
            6.1,
        )),
        Material {
            color: Vector3::new(0.9, 0.9, 0.9),
            roughness: 0.2,
            ..Default::default()
        },
    );

    let scene = Scene::new(
        camera,
        vec![sphere_a, sphere_b, sphere_c, light, environment],
    );

    let start = Instant::now();
    let renderer = BackwardRenderer::new(5).parallel(NUM_SAMPLES);
    let render_buffer = renderer.render(&scene);

    println!("Rendering took {:?}", start.elapsed());

    let image = render_buffer.to_image();

    image.save("image.png").expect("Could not save image");
}
