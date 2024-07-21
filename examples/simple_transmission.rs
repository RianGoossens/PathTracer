use std::{f64::consts::TAU, time::Instant};

use path_tracer::{
    aperture::PinholeAperture, camera::CameraSettings, object::ObjectDefinition,
    renderer::RecursiveBDPT, shape::Cuboid, Camera, Material, Object, Renderer, Scene, Sphere,
};

use nalgebra as na;

use na::{Similarity3, Vector3};

const NUM_SAMPLES: usize = 100;
const SIZE: u32 = 300;

fn main() {
    let camera_settings = CameraSettings {
        y: 1.7,
        z: 3.,
        rz: TAU / 2.,
        ry: TAU / 10.,
        width: SIZE,
        height: SIZE,
        fov_degrees: 50.,
        znear: 1.,
        ..Default::default()
    };
    let camera = Camera::new(camera_settings, PinholeAperture, 1.);

    let plane = Object::new(ObjectDefinition {
        shape: Box::new(Cuboid::new(6., 6., 1.)),
        material: Material::new_reflective(Vector3::new(1., 1., 1.) * 0.9, 1., 0., 1.),
        z: -0.6,
        ..Default::default()
    });

    let ior = 3.;
    let sphere_a = Object::new_old(
        Sphere::new(0.3),
        Similarity3::new(Vector3::new(0.6, -0.5, 0.3), Vector3::zeros(), 1.),
        Material::new_reflective(Vector3::new(0.9, 0.1, 0.1), 0.2, 0.25, ior),
    );

    let sphere_b = Object::new_old(
        Sphere::new(0.3),
        Similarity3::new(Vector3::new(0., 0., 0.3), Vector3::zeros(), 1.),
        Material::new_reflective(Vector3::new(0.1, 0.9, 0.1), 0., 0.5, ior),
    );

    let sphere_c = Object::new_old(
        Sphere::new(0.3),
        Similarity3::new(Vector3::new(-0.6, -0.5, 0.3), Vector3::zeros(), 1.),
        Material::new_reflective(Vector3::new(0.1, 0.1, 0.9), 0.2, 0.75, ior),
    );

    let light = Object::new_old(
        Sphere::new(1.),
        Similarity3::new(Vector3::new(0., -4., 2.5), Vector3::zeros(), 1.),
        Material::Emissive {
            color: Vector3::new(1., 1., 1.) * 1.,
        },
    );

    let _light_enclosure = Object::new_old(
        Sphere::new(0.55),
        Similarity3::new(Vector3::new(0., -1., 1.), Vector3::zeros(), 1.),
        Material::new_reflective(Vector3::new(1., 1., 1.), 0., 1., 2.),
    );

    let _environment = Object::new_old(
        Sphere::new(5.),
        Similarity3::new(Vector3::new(0., 0., 0.), Vector3::zeros(), 1.),
        Material::new_reflective(Vector3::new(1., 1., 1.) * 0.1, 1.0, 0.5, 1.),
    );

    let scene = Scene::new(camera, vec![plane, sphere_a, sphere_b, sphere_c, light]);

    let start = Instant::now();
    let renderer = RecursiveBDPT::new(5).parallel(NUM_SAMPLES);
    let render_buffer = renderer.render(&scene);

    println!("Rendering took {:?}", start.elapsed());

    let image = render_buffer.srgb().to_image_u8();

    image.save("image.png").expect("Could not save image");

    let image = render_buffer.to_image_f32();
    image.save("image.exr").expect("Could not save exr");
}
