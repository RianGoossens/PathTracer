use std::{rc::Rc, time::Instant};

use path_tracer::{Camera, Inverted, Material, Object, Renderer, Scene, Sphere};

use nalgebra as na;

use na::{Similarity3, Vector3};

const NUM_SAMPLES: u16 = 2500;

fn main() {
    let camera = Camera::new(300, 300, 70., 1.0, 100.0);

    let sphere_shape = Rc::new(Sphere);

    let sphere_a = Object::new(
        sphere_shape.clone(),
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
        sphere_shape.clone(),
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
        sphere_shape.clone(),
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
        sphere_shape.clone(),
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
        Rc::new(Inverted(Sphere)),
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

    let scene = Scene::new(camera, &[sphere_a, sphere_b, sphere_c, light, environment]);

    let start = Instant::now();
    let renderer = Renderer;
    let mut render_buffer = renderer.render(&scene);

    for _ in 0..NUM_SAMPLES - 1 {
        render_buffer += renderer.render(&scene);
    }

    render_buffer /= NUM_SAMPLES as f64;
    println!("Rendering took {:?}", start.elapsed());

    let image = render_buffer.to_image();

    image.save("image.png").expect("Could not save image");
}
