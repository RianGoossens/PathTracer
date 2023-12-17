use std::rc::Rc;

use path_tracer::{Camera, Material, Object, Renderer, Scene, Sphere};

use nalgebra as na;

use na::{Similarity3, Translation3, Vector3};

const NUM_SAMPLES: u16 = 1000;

fn main() {
    let camera = Camera::new(300, 300, 70., 1.0, 100.0);

    let sphere_shape = Rc::new(Sphere);

    let sphere_a = Object::new(
        sphere_shape.clone(),
        na::convert(Translation3::new(1., 2., -5.5)),
        Material {
            color: Vector3::new(0.6, 0.2, 0.7),
            roughness: 0.1,
            ..Default::default()
        },
    );

    let sphere_b = Object::new(
        sphere_shape.clone(),
        na::convert(Similarity3::new(
            Vector3::new(-1., 2., -5.),
            Vector3::zeros(),
            1.,
        )),
        Material {
            color: Vector3::new(0.4, 0.8, 0.3),
            roughness: 0.8,
            ..Default::default()
        },
    );

    let light = Object::new(
        sphere_shape,
        na::convert(Similarity3::new(
            Vector3::new(0., 0., -5.),
            Vector3::zeros(),
            1.,
        )),
        Material {
            color: Vector3::new(3., 3., 3.),
            emissive: true,
            ..Default::default()
        },
    );

    let scene = Scene {
        camera,
        objects: vec![sphere_a, sphere_b, light],
        lights: vec![],
    };

    let renderer = Renderer;
    let mut render_buffer = renderer.render(&scene);

    for _ in 0..NUM_SAMPLES - 1 {
        render_buffer += renderer.render(&scene);
    }

    render_buffer /= NUM_SAMPLES as f64;

    let image = render_buffer.to_image();

    image.save("image.png").expect("Could not save image");
}
