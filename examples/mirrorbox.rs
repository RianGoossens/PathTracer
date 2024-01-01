use std::time::Instant;

use path_tracer::{
    aperture::{GaussianAperture},
    camera::CameraSettings,
    renderer::{RecursiveBDPT},
    shape::Cuboid, Camera, Material, Object, Renderer, Scene, Sphere,
};

use nalgebra as na;

use na::{Similarity3, Vector3};

const NUM_SAMPLES: usize = 10;

fn main() {
    let aperture = GaussianAperture::new(0.01); //RegularPolygonAperture::new(0.05, 3);
    let camera = Camera::new(
        CameraSettings {
            width: 300,
            height: 300,
            fov_degrees: 90.,
            z: 0.5,
            ..Default::default()
        },
        aperture,
        2.,
    );

    let mirror_material = Material::new(Vector3::new(0.5, 0.8, 0.5), 0., false);

    let cube = Object::new(
        Cuboid::new(2., 2., 2.),
        Similarity3::new(Vector3::new(0., 0., 0.), Vector3::new(0., 0., 0.), 1.),
        mirror_material,
    );

    let light = Object::new(
        Sphere::new(1.),
        Similarity3::new(Vector3::new(0., 0.5, 0.0), Vector3::new(0., 0., 0.), 0.2),
        Material::new(Vector3::new(1., 1., 1.) * 1., 1., true),
    );

    let scene = Scene::new(camera, vec![cube, light]);

    let start = Instant::now();

    let renderer = RecursiveBDPT::new(20).parallel(NUM_SAMPLES);
    let render_buffer = renderer.render(&scene);

    println!("Rendering took {:?}", start.elapsed());

    let image = render_buffer.srgb().to_image_u8();

    image.save("image.png").expect("Could not save image");
}
