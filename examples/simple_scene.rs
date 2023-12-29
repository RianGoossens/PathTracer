use std::{f64::consts::TAU, time::Instant};

use path_tracer::{
    aperture::{PinholeAperture, RegularPolygonAperture},
    camera::CameraSettings,
    renderer::BDPTRenderer,
    BackwardRenderer, Camera, Inverted, Material, Object, Renderer, Scene, Sphere,
};

use nalgebra as na;

use na::{Similarity3, Vector3};

const NUM_SAMPLES: usize = 100;

fn main() {
    let aperture = PinholeAperture; // RegularPolygonAperture::new(0.5, 6);
    let camera_settings = CameraSettings {
        z: 3.,
        width: 300,
        height: 300,
        fov_degrees: 90.,
        znear: 1.,
        ..Default::default()
    };
    let camera = Camera::new(camera_settings, aperture, 5.);

    let sphere: Object = Object::new(
        Sphere::new(1.),
        Similarity3::new(Vector3::new(1., 0., 0.), Vector3::zeros(), 1.),
        Material::new(Vector3::new(0.1, 0.8, 0.1), 1., false),
    );

    let light = Object::new(
        Sphere::new(1.),
        Similarity3::new(Vector3::new(-1.5, 0., 0.), Vector3::zeros(), 0.5),
        Material::new(Vector3::new(1., 1., 1.), 1.0, true),
    );

    let scene = Scene::new(camera, vec![sphere, light]);

    let start = Instant::now();
    let renderer = BDPTRenderer::new(5).parallel(NUM_SAMPLES);
    let render_buffer = renderer.render(&scene);

    println!("Rendering took {:?}", start.elapsed());

    let image = render_buffer.srgb().to_image_u8();

    image.save("image.png").expect("Could not save image");

    let image = render_buffer.to_image_f32();
    image.save("image.exr").expect("Could not save exr");
}