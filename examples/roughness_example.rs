use std::{f64::consts::TAU, time::Instant};

use path_tracer::{
    aperture::PinholeAperture, renderer::BDPTRenderer, shape::Cuboid, Camera, Material, Object,
    Renderer, Scene, Sphere,
};

use nalgebra as na;

use na::{Similarity3, Vector3};

const NUM_SAMPLES: usize = 100;

fn main() {
    let aperture = PinholeAperture;
    let camera = Camera::new_at_origin(300, 300, 70., 1., 100.0, aperture, 1.);

    let cube_a = Object::new_old(
        Cuboid::new(1., 1., 1.),
        Similarity3::new(
            Vector3::new(-2., 2., -5.),
            Vector3::new(TAU / 4., TAU / 2., TAU / 4.),
            1.,
        ),
        Material::new(Vector3::new(0.5, 0.5, 0.5), 1.0, false),
    );

    let cube_b = Object::new_old(
        Cuboid::new(1., 1., 1.),
        Similarity3::new(
            Vector3::new(2., -2., -5.),
            Vector3::new(-TAU / 4., -TAU / 2., TAU / 4.),
            1.,
        ),
        Material::new(Vector3::new(0.5, 0.5, 0.5), 0.1, false),
    );

    let sphere_a = Object::new_old(
        Sphere::new(1.),
        Similarity3::new(
            Vector3::new(-2., -2., -5.),
            Vector3::new(TAU / 4., TAU / 2., TAU / 4.),
            1.,
        ),
        Material::new(Vector3::new(0.5, 0.5, 0.5), 1.0, false),
    );

    let sphere_b = Object::new_old(
        Sphere::new(1.),
        Similarity3::new(
            Vector3::new(2., 2., -5.),
            Vector3::new(TAU / 4., TAU / 2., TAU / 4.),
            1.,
        ),
        Material::new(Vector3::new(0.5, 0.5, 0.5), 0.05, false),
    );

    let light = Object::new_old(
        Sphere::new(1.),
        Similarity3::new(Vector3::new(0., 0.0, -5.), Vector3::new(0., 0., 0.), 1.0),
        Material::new(Vector3::new(1., 1., 1.), 1., true),
    );

    let scene = Scene::new(camera, vec![cube_a, cube_b, sphere_a, sphere_b, light]);

    let start = Instant::now();

    let renderer = BDPTRenderer::new(10).parallel(NUM_SAMPLES);
    let render_buffer = renderer.render(&scene);

    println!("Rendering took {:?}", start.elapsed());

    let image = render_buffer.srgb().to_image_u8();

    image.save("image.png").expect("Could not save image");
}
