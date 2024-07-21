use std::{f64::consts::TAU, time::Instant};

use path_tracer::{
    aperture::RegularPolygonAperture,
    camera::CameraSettings,
    object::ObjectDefinition,
    renderer::RecursiveBDPT,
    shape::{Cuboid, Plane},
    Camera, Material, Object, Renderer, Scene, Sphere,
};

use nalgebra as na;

use na::Vector3;

const NUM_SAMPLES: usize = 10;
const SIZE: u32 = 300;

fn main() {
    let aperture = RegularPolygonAperture::new(0.05, 6);
    let camera_settings = CameraSettings {
        z: 2.,
        width: SIZE,
        height: SIZE,
        fov_degrees: 70.,
        znear: 1.,
        ..Default::default()
    };
    let camera = Camera::new(camera_settings, aperture, 2.25);

    let white_material = Material::new(Vector3::new(1., 1., 1.) * 0.8, 0.5, false);
    let green_material = Material::new(Vector3::new(0.1, 0.8, 0.1), 0.5, false);
    let red_material = Material::new(Vector3::new(0.8, 0.1, 0.1), 0.5, false);

    let bottom_plane = Object::new(ObjectDefinition {
        shape: Box::new(Plane::new(2., 2.)),
        material: white_material.clone(),
        y: -1.,
        rx: TAU / 4.,
        ..Default::default()
    });

    let left_plane = Object::new(ObjectDefinition {
        shape: Box::new(Plane::new(2., 2.)),
        material: red_material,
        x: -1.,
        ry: TAU / 4.,
        ..Default::default()
    });

    let right_plane = Object::new(ObjectDefinition {
        shape: Box::new(Plane::new(2., 2.)),
        material: green_material,
        x: 1.,
        ry: TAU / 4.,
        ..Default::default()
    });

    let top_plane = Object::new(ObjectDefinition {
        shape: Box::new(Plane::new(2., 2.)),
        material: white_material.clone(),
        y: 1.,
        rx: TAU / 4.,
        ..Default::default()
    });

    let back_plane = Object::new(ObjectDefinition {
        shape: Box::new(Plane::new(2., 2.)),
        material: white_material.clone(),
        z: -1.,
        ..Default::default()
    });

    let box_a = Object::new(ObjectDefinition {
        shape: Box::new(Cuboid::new(0.4, 0.4, 0.4)),
        material: Material::new(Vector3::new(0.7, 0.8, 0.6), 0.5, false),
        x: -0.25,
        y: -0.7,
        z: -0.2,
        ry: TAU / 10.,
        scale: 1.5,
        ..Default::default()
    });

    let sphere_a = Object::new(ObjectDefinition {
        shape: Box::new(Sphere::new(1.0)),
        material: Material::new_reflective(Vector3::new(0.9, 0.9, 0.9), 0., 0.5, 1.4),
        x: -0.25,
        y: -0.175,
        z: -0.2,
        scale: 0.25,
        ..Default::default()
    });

    let sphere_b = Object::new(ObjectDefinition {
        shape: Box::new(Sphere::new(1.0)),
        material: Material::new_reflective(Vector3::new(0.4, 0.6, 0.9), 0.3, 0.5, 1.),
        x: 0.5,
        y: -0.7,
        z: 0.2,
        scale: 0.3,
        ..Default::default()
    });

    let top_light = Object::new(ObjectDefinition {
        shape: Box::new(Plane::new(0.25, 0.25)),
        material: Material::new(Vector3::new(1.0, 1.0, 0.5), 1., true),
        y: 0.995,
        rx: -TAU / 4.,
        ..Default::default()
    });

    let scene = Scene::new(
        camera,
        vec![
            bottom_plane,
            back_plane,
            top_plane,
            left_plane,
            right_plane,
            box_a,
            sphere_a,
            sphere_b,
            top_light,
        ],
    );

    let start = Instant::now();
    let renderer = RecursiveBDPT::new(5).parallel(NUM_SAMPLES);
    let render_buffer = renderer.render(&scene);

    //let render_buffer = render_buffer.median_filter(3);
    println!("Rendering took {:?}", start.elapsed());

    let image = render_buffer.srgb().to_image_u8();

    image.save("image.png").expect("Could not save image");

    let image = render_buffer.to_image_f32();
    image.save("image.exr").expect("Could not save exr");
}
