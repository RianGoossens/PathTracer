use std::time::Instant;

use path_tracer::{
    aperture::RegularPolygonAperture, camera::CameraSettings, object::ObjectDefinition,
    renderer::RecursiveBDPT, Camera, Inverted, Material, Renderer, Scene, Sphere,
};

use nalgebra as na;

use na::Vector3;

const NUM_SAMPLES: usize = 100;

fn main() {
    let aperture = RegularPolygonAperture::new(0.5, 6);
    let camera_settings = CameraSettings {
        z: 6.,
        width: 300,
        height: 300,
        fov_degrees: 70.,
        znear: 1.,
        ..Default::default()
    };
    let camera = Camera::new(camera_settings, aperture, 5.);

    let sphere_shape = Sphere::new(1.);

    let sphere_a = ObjectDefinition {
        shape: Box::new(sphere_shape),
        material: Material::new(Vector3::new(0.8, 0.1, 0.1), 0.9, false),
        x: 1.5,
        y: -0.5,
        z: 1.0,
        ..Default::default()
    };

    let sphere_b = ObjectDefinition {
        shape: Box::new(sphere_shape),
        material: Material::new(Vector3::new(0.1, 0.8, 0.1), 0.9, false),
        x: 1.0,
        ..Default::default()
    };

    let sphere_c = ObjectDefinition {
        shape: Box::new(sphere_shape),
        material: Material::new(Vector3::new(0.1, 0.1, 0.8), 0.9, false),
        x: 0.5,
        y: 0.5,
        z: -1.0,
        ..Default::default()
    };

    let light = ObjectDefinition {
        shape: Box::new(sphere_shape),
        material: Material::new(Vector3::new(1.0, 1.0, 1.0), 1.0, true),
        x: -1.5,
        scale: 0.5,
        ..Default::default()
    };

    let big_sphere = ObjectDefinition {
        shape: Box::new(Sphere::new(1.0)),
        material: Material::new(Vector3::new(0.95, 1.0, 0.95), 0.5, false),
        y: -7.5,
        scale: 6.1,
        ..Default::default()
    };

    let environment = ObjectDefinition {
        shape: Box::new(Inverted(Sphere::new(1.0))),
        material: Material::new(Vector3::new(1.0, 1.0, 1.0) * 0.3, 1.0, false),
        scale: 6.1,
        ..Default::default()
    };

    let scene = Scene::new(
        camera,
        vec![sphere_a, sphere_b, sphere_c, big_sphere, environment, light],
    );

    let start = Instant::now();
    let renderer = RecursiveBDPT::new(5).parallel(NUM_SAMPLES);
    let render_buffer = renderer.render(&scene);

    println!("Rendering took {:?}", start.elapsed());

    let image = render_buffer.srgb().to_image_u8();

    image.save("image.png").expect("Could not save image");

    let image = render_buffer.to_image_f32();
    image.save("image.exr").expect("Could not save exr");
}
