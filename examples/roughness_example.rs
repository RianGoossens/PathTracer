use std::{f64::consts::TAU, time::Instant};

use path_tracer::{
    aperture::PinholeAperture, object::ObjectDefinition, renderer::BDPTRenderer, shape::Cuboid,
    Camera, Material, Object, Renderer, Scene, Sphere,
};

use nalgebra as na;

use na::Vector3;

const NUM_SAMPLES: usize = 100;

fn main() {
    let aperture = PinholeAperture;
    let camera = Camera::new_at_origin(300, 300, 70., 1., 100.0, aperture, 1.);

    let cube_a = Object::new(ObjectDefinition {
        shape: Box::new(Cuboid::new(1., 1., 1.)),
        material: Material::new(Vector3::new(0.5, 0.5, 0.5), 1.0, false),
        x: -2.,
        y: 2.,
        z: -5.,
        rx: TAU / 4.,
        ry: TAU / 2.,
        rz: TAU / 4.,
        scale: 1.,
    });

    let cube_b = Object::new(ObjectDefinition {
        shape: Box::new(Cuboid::new(1., 1., 1.)),
        material: Material::new(Vector3::new(0.5, 0.5, 0.5), 0.1, false),
        x: 2.,
        y: -2.,
        z: -5.,
        rx: -TAU / 4.,
        ry: -TAU / 2.,
        rz: TAU / 4.,
        scale: 1.,
    });

    let sphere_a = Object::new(ObjectDefinition {
        shape: Box::new(Sphere::new(1.)),
        material: Material::new(Vector3::new(0.5, 0.5, 0.5), 1.0, false),
        x: -2.,
        y: -2.,
        z: -5.,
        rx: TAU / 4.,
        ry: TAU / 2.,
        rz: TAU / 4.,
        scale: 1.,
    });

    let sphere_b = Object::new(ObjectDefinition {
        shape: Box::new(Sphere::new(1.)),
        material: Material::new(Vector3::new(0.5, 0.5, 0.5), 0.05, false),
        x: 2.,
        y: 2.,
        z: -5.,
        rx: TAU / 4.,
        ry: TAU / 2.,
        rz: TAU / 4.,
        scale: 1.,
    });

    let light = Object::new(ObjectDefinition {
        shape: Box::new(Sphere::new(1.)),
        material: Material::new(Vector3::new(1., 1., 1.), 1., true),
        z: -5.,
        scale: 1.0,
        ..Default::default()
    });

    let scene = Scene::new(camera, vec![cube_a, cube_b, sphere_a, sphere_b, light]);

    let start = Instant::now();

    let renderer = BDPTRenderer::new(10).parallel(NUM_SAMPLES);
    let render_buffer = renderer.render(&scene);

    println!("Rendering took {:?}", start.elapsed());

    let image = render_buffer.srgb().to_image_u8();

    image.save("image.png").expect("Could not save image");
}
