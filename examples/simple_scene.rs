use std::time::Instant;

use path_tracer::{
    aperture::PinholeAperture, camera::CameraSettings, object::ObjectDefinition,
    renderer::RecursiveBDPT, Camera, Material, Object, Renderer, Scene, Sphere,
};

use nalgebra as na;

use na::Vector3;

const NUM_SAMPLES: usize = 100;

fn main() {
    let aperture = PinholeAperture; // RegularPolygonAperture::new(0.5, 6);
    let camera_settings = CameraSettings {
        x: 1.,
        z: 3.,
        width: 300,
        height: 300,
        fov_degrees: 90.,
        znear: 1.,
        ..Default::default()
    };
    let camera = Camera::new(camera_settings, aperture, 5.);

    let sphere = Object::new(ObjectDefinition {
        shape: Box::new(Sphere::new(1.)),
        material: Material::new(Vector3::new(0.1, 0.8, 0.1), 1., false),
        x: 1.,
        y: 1.,
        scale: 1.,
        ..Default::default()
    });

    let light = Object::new(ObjectDefinition {
        shape: Box::new(Sphere::new(1.)),
        material: Material::Emissive {
            color: Vector3::new(1., 1., 1.) * 1.,
        },
        x: 1.0,
        y: -1.5,
        scale: 1.,
        ..Default::default()
    });

    let scene = Scene::new(camera, vec![sphere, light]);

    let start = Instant::now();
    let renderer = RecursiveBDPT::new(5).parallel(NUM_SAMPLES);
    let render_buffer = renderer.render(&scene);

    println!("Rendering took {:?}", start.elapsed());

    let image = render_buffer.srgb().to_image_u8();
    image.save("image.png").expect("Could not save image");

    let image = render_buffer.to_image_f32();
    image.save("image.exr").expect("Could not save exr");
}
