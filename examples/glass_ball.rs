use std::{f64::consts::TAU, time::Instant};

use path_tracer::{
    aperture::PinholeAperture,
    camera::CameraSettings,
    object::ObjectDefinition,
    renderer::RecursiveBDPT,
    shader::Checkerboard,
    shape::{Cuboid, Plane},
    Camera, Material, Renderer, Scene, Sphere,
};

use nalgebra as na;

use na::Vector3;

const NUM_SAMPLES: usize = 1000;
const SIZE: u32 = 300;

fn main() {
    let aperture = PinholeAperture; //RegularPolygonAperture::new(0.05, 6);
    let camera_settings = CameraSettings {
        z: 3.,
        width: SIZE,
        height: SIZE,
        fov_degrees: 90.,
        znear: 1.,
        ..Default::default()
    };
    let camera = Camera::new(camera_settings, aperture, 2.);

    //let plane_shader: ColorShader = Vector3::new(0.8, 0.6, 0.6).into();
    let checkerboard = Checkerboard::new(
        Vector3::new(0.8, 0.6, 0.6),
        Vector3::new(0.6, 0.6, 0.8) * 0.5,
        0.25,
    );
    let bottom_plane = ObjectDefinition {
        shape: Box::new(Plane::new(10., 10.)),
        material: Material::new_reflective(checkerboard, 1., 0., 0.),
        ..Default::default()
    };

    let ior = 1.5;

    let left_cuboid = ObjectDefinition {
        shape: Box::new(Cuboid::new(1., 1., 0.25)),
        material: Material::new_reflective(Vector3::new(0.9, 0.99, 0.9), 0., 0.9, ior),
        x: -0.6,
        z: 1.,
        scale: 0.75,
        ..Default::default()
    };

    let right_cuboid = ObjectDefinition {
        shape: Box::new(Cuboid::new(1., 1., 0.25)),
        material: Material::new_reflective(Vector3::new(0.9, 0.99, 0.9), 0., 0.9, ior),
        x: 0.6,
        z: 1.,
        rx: TAU / 16.,
        scale: 0.75,
        ..Default::default()
    };

    let sphere = ObjectDefinition {
        shape: Box::new(Sphere::new(0.5)),
        material: Material::new_reflective(Vector3::new(0.9, 0.99, 0.9), 0., 0.9, ior),
        y: 1.,
        z: 1.,
        ..Default::default()
    };

    let inner_sphere = ObjectDefinition {
        shape: Box::new(Sphere::new(0.5)),
        material: Material::new_reflective(Vector3::new(1., 1., 1.), 0., 0.9, 1. / ior),
        y: 1.,
        z: 1.,
        scale: 0.9,
        ..Default::default()
    };

    let light = ObjectDefinition {
        shape: Box::new(Sphere::new(1.)),
        material: Material::new(Vector3::new(1.0, 1.0, 1.0) * 5.0, 1., true),
        x: 1.0,
        y: 1.0,
        z: 3.,
        ..Default::default()
    };

    let environment = ObjectDefinition {
        shape: Box::new(Sphere::new(5.)),
        material: Material::new_reflective(Vector3::new(1., 1., 1.) * 0.75, 1., 0.9, 1.),
        ..Default::default()
    };

    let scene = Scene::new(
        camera,
        vec![
            bottom_plane,
            left_cuboid,
            right_cuboid,
            sphere,
            inner_sphere,
            light,
            environment,
        ],
    );

    let start = Instant::now();
    let renderer = RecursiveBDPT::new(10).parallel(NUM_SAMPLES);
    let render_buffer = renderer.render(&scene);

    //let render_buffer = render_buffer.median_filter(9);
    println!("Rendering took {:?}", start.elapsed());

    let image = render_buffer.srgb().to_image_u8();

    image.save("image.png").expect("Could not save image");

    let image = render_buffer.to_image_f32();
    image.save("image.exr").expect("Could not save exr");
}
