use std::time::Instant;

use path_tracer::{
    aperture::RegularPolygonAperture, camera::CameraSettings, renderer::RecursiveBDPT, Camera,
    Inverted, Material, Object, Renderer, Scene, Sphere,
};

use nalgebra as na;

use na::{Similarity3, Vector3};

const NUM_SAMPLES: usize = 10;

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

    let sphere_a = Object::new(
        sphere_shape,
        Similarity3::new(Vector3::new(1.5, -0.5, 1.), Vector3::zeros(), 1.),
        Material::new(Vector3::new(0.8, 0.1, 0.1), 0.9, false),
    );

    let sphere_b = Object::new(
        sphere_shape,
        Similarity3::new(Vector3::new(1., 0., 0.), Vector3::zeros(), 1.),
        Material::new(Vector3::new(0.1, 0.8, 0.1), 0.9, false),
    );

    let sphere_c = Object::new(
        sphere_shape,
        Similarity3::new(Vector3::new(0.5, 0.5, -1.), Vector3::zeros(), 1.),
        Material::new(Vector3::new(0.1, 0.1, 0.8), 0.9, false),
    );

    let light = Object::new(
        sphere_shape,
        Similarity3::new(Vector3::new(-1.5, 0., 0.), Vector3::zeros(), 1.),
        Material::new(Vector3::new(1., 1., 1.), 1.0, true),
    );

    let big_sphere = Object::new(
        Sphere::new(1.),
        Similarity3::new(Vector3::new(0., -7.5, 0.), Vector3::zeros(), 6.1),
        Material::new(Vector3::new(0.95, 1., 0.95) * 0.3, 0.01, false),
    );

    let environment = Object::new(
        Inverted(Sphere::new(1.)),
        Similarity3::new(Vector3::new(0., 0., 0.), Vector3::zeros(), 6.1),
        Material::new(Vector3::new(1., 1., 1.) * 0.3, 1., false),
    );

    let scene = Scene::new(
        camera,
        vec![sphere_a, sphere_b, sphere_c, big_sphere, environment, light],
    );

    let start = Instant::now();
    let renderer = RecursiveBDPT::new(10).parallel(NUM_SAMPLES);
    let render_buffer = renderer.render(&scene);

    println!("Rendering took {:?}", start.elapsed());

    let image = render_buffer.srgb().to_image_u8();

    image.save("image.png").expect("Could not save image");

    let image = render_buffer.to_image_f32();
    image.save("image.exr").expect("Could not save exr");
}
