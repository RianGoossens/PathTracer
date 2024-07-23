use std::{f64::consts::TAU, time::Instant};

use path_tracer::{
    aperture::RegularPolygonAperture,
    camera::CameraSettings,
    object::ObjectDefinition,
    renderer::RecursiveBDPT,
    shape::{Cylinder, Plane},
    Camera, Material, Renderer, Scene, Sphere,
};

use nalgebra as na;

use na::Vector3;

const NUM_SAMPLES: usize = 500;
const SIZE: u32 = 300;

fn main() {
    let aperture = RegularPolygonAperture::new(0.05, 6);
    let camera_settings = CameraSettings {
        z: 2.,
        rx: -TAU / 7.,
        y: 2.,
        width: SIZE,
        height: SIZE,
        fov_degrees: 50.,
        znear: 1.,
        ..Default::default()
    };
    let camera = Camera::new(camera_settings, aperture, 5.);

    let floor_material = Material::new(Vector3::new(0.4, 0.3, 0.3), 1., false);

    let bottom_plane = ObjectDefinition {
        shape: Box::new(Plane::new(10., 10.)),
        material: floor_material.clone(),
        y: -1.,
        rx: TAU / 4.,
        ..Default::default()
    };

    let cylinder = ObjectDefinition {
        shape: Box::new(Cylinder::new(1.0, 0.8)),
        material: Material::new_reflective(Vector3::new(0.99, 0.1, 0.1), 0., 0., 1.),
        y: -0.75,
        rx: -TAU / 4.,
        ..Default::default()
    };

    let top_light = ObjectDefinition {
        shape: Box::new(Sphere::new(0.5)),
        material: Material::new(Vector3::new(1.0, 1.0, 1.0) * 2., 1., true),
        x: 1.5,
        y: 1.0,
        z: 2.0,
        ..Default::default()
    };

    let scene = Scene::new(camera, vec![bottom_plane, cylinder, top_light]);

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
