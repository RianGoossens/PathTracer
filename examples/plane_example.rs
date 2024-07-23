use std::time::Instant;

use path_tracer::{
    aperture::PinholeAperture, object::ObjectDefinition, renderer::BDPTRenderer, shape::Plane,
    Camera, Material, Renderer, Scene, Sphere,
};

use nalgebra as na;

use na::Vector3;

const NUM_SAMPLES: usize = 100;

fn main() {
    let aperture = PinholeAperture; //,GaussianAperture::new(0.5);
    let camera = Camera::new_at_origin(300, 300, 55., 1.0, 100.0, aperture, 5.);

    let plane = ObjectDefinition {
        shape: Box::new(Plane::new(2., 2.)),
        z: -5.,
        rx: (20f64).to_radians(),
        material: Material::new(Vector3::new(1., 1., 1.), 0.05, false),
        ..Default::default()
    };

    let sphere = ObjectDefinition {
        shape: Box::new(Sphere::new(1.)),
        y: -5.8,
        z: -6.,
        scale: 5.,
        material: Material::new(Vector3::new(1., 0., 1.), 1., false),
        ..Default::default()
    };

    let light = ObjectDefinition {
        shape: Box::new(Sphere::new(1.)),
        y: 2.,
        z: -4.,
        material: Material::new(Vector3::new(1., 1., 1.), 1.0, true),
        ..Default::default()
    };

    let scene = Scene::new(camera, vec![plane, sphere, light]);

    let start = Instant::now();

    let renderer = BDPTRenderer::new(10).parallel(NUM_SAMPLES);
    let render_buffer = renderer.render(&scene);

    println!("Rendering took {:?}", start.elapsed());

    let image = render_buffer.srgb().to_image_u8();

    image.save("image.png").expect("Could not save image");
}
