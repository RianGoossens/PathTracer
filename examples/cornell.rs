use std::{f64::consts::TAU, time::Instant};

use path_tracer::{
    aperture::RegularPolygonAperture,
    camera::CameraSettings,
    renderer::RecursiveBDPT,
    shape::{Cuboid, Plane},
    Camera, Material, Object, Renderer, Scene, Sphere,
};

use nalgebra as na;

use na::{Similarity3, Vector3};

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

    let bottom_plane = Object::new(
        Plane::new(2., 2.),
        Similarity3::new(
            Vector3::new(0., -1., 0.),
            Vector3::new(TAU / 4., 0., 0.),
            1.,
        ),
        white_material.clone(),
    );

    let left_plane = Object::new(
        Plane::new(2., 2.),
        Similarity3::new(
            Vector3::new(-1., 0., 0.),
            Vector3::new(0., TAU / 4., 0.),
            1.,
        ),
        red_material,
    );

    let right_plane = Object::new(
        Plane::new(2., 2.),
        Similarity3::new(Vector3::new(1., 0., 0.), Vector3::new(0., TAU / 4., 0.), 1.),
        green_material,
    );

    let top_plane = Object::new(
        Plane::new(2., 2.),
        Similarity3::new(Vector3::new(0., 1., 0.), Vector3::new(TAU / 4., 0., 0.), 1.),
        white_material.clone(),
    );

    let back_plane = Object::new(
        Plane::new(2., 2.),
        Similarity3::new(Vector3::new(0., 0., -1.), Vector3::new(0., 0., 0.), 1.),
        white_material.clone(),
    );

    let box_a = Object::new(
        Cuboid::new(0.4, 0.4, 0.4),
        Similarity3::new(
            Vector3::new(-0.25, -0.7, -0.2),
            Vector3::new(0., TAU / 10., 0.),
            1.5,
        ),
        Material::new(Vector3::new(0.7, 0.8, 0.6), 0.5, false),
    );

    let sphere_a: Object = Object::new(
        Sphere::new(1.0),
        Similarity3::new(
            Vector3::new(-0.25, -0.175, -0.2),
            Vector3::new(0., 0., 0.),
            0.25,
        ),
        Material::new_reflective(Vector3::new(0.9, 0.9, 0.9), 0., 0.5, 1.4),
    );

    let sphere_b: Object = Object::new(
        Sphere::new(1.0),
        Similarity3::new(Vector3::new(0.5, -0.7, 0.2), Vector3::new(0., 0., 0.), 0.3),
        Material::new_reflective(Vector3::new(0.4, 0.6, 0.9), 0.3, 0.5, 1.),
    );

    let top_light = Object::new(
        Plane::new(0.25, 0.25),
        //Sphere::new(1.),
        //Cuboid::new(1.0, 1.0, 0.2),
        //Cylinder::new(1., 0.2),
        Similarity3::new(
            Vector3::new(0., 0.995, 0.),
            Vector3::new(-TAU / 4., 0., 0.),
            1.,
        ),
        Material::new(Vector3::new(1.0, 1.0, 0.5), 1., true),
    );

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
