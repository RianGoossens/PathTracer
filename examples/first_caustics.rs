use std::{f64::consts::TAU, time::Instant};

use path_tracer::{
    aperture::RegularPolygonAperture,
    camera::CameraSettings,
    renderer::RecursiveBDPT,
    shape::{Cylinder, Plane},
    Camera, Material, Object, Renderer, Scene, Sphere,
};

use nalgebra as na;

use na::{Similarity3, Vector3};

const NUM_SAMPLES: usize = 100;
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

    let floor_material = Material::new(Vector3::new(0.8, 0.6, 0.6), 1., false);

    let bottom_plane = Object::new(
        Plane::new(10., 10.),
        Similarity3::new(
            Vector3::new(0., -1., 0.),
            Vector3::new(TAU / 4., 0., 0.),
            1.,
        ),
        floor_material.clone(),
    );

    let cylinder = Object::new(
        Cylinder::new(1.0, 0.8),
        Similarity3::new(
            Vector3::new(0., -0.75, 0.),
            Vector3::new(-TAU / 4., 0., 0.),
            1.,
        ),
        Material::new(Vector3::new(0.99, 0.1, 0.1), 0., false),
    );

    let top_light = Object::new(
        Sphere::new(0.5),
        Similarity3::new(Vector3::new(1.5, 1.0, 2.), Vector3::new(0., 0., 0.), 1.0),
        Material::new(Vector3::new(1.0, 1.0, 1.0) * 5.0, 1., true),
    );

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
