use std::time::Instant;

use path_tracer::{
    aperture::PinholeAperture, renderer::BDPTRenderer, shape::Plane, Camera, Material, Object,
    Renderer, Scene, Sphere,
};

use nalgebra as na;

use na::{Similarity3, Vector3};

const NUM_SAMPLES: usize = 100;

fn main() {
    let aperture = PinholeAperture; //,GaussianAperture::new(0.5);
    let camera = Camera::new_at_origin(300, 300, 55., 1.0, 100.0, aperture, 5.);

    let plane = Object::new(
        Plane::new(2., 2.),
        Similarity3::new(
            Vector3::new(0., 0., -5.),
            Vector3::new((20f64).to_radians(), 0., 0.),
            1.,
        ),
        Material::new(Vector3::new(1., 1., 1.), 0.05, false),
    );

    let sphere = Object::new(
        Sphere::new(1.),
        Similarity3::new(Vector3::new(0., -5.8, -6.), Vector3::zeros(), 5.),
        Material::new(Vector3::new(1., 0., 1.), 1., false),
    );

    let light = Object::new(
        Sphere::new(1.),
        Similarity3::new(Vector3::new(0., 2., -4.), Vector3::zeros(), 1.),
        Material::new(Vector3::new(1., 1., 1.), 1.0, true),
    );

    let scene = Scene::new(camera, vec![plane, sphere, light]);

    let start = Instant::now();

    let renderer = BDPTRenderer::new(10).parallel(NUM_SAMPLES);
    let render_buffer = renderer.render(&scene);

    println!("Rendering took {:?}", start.elapsed());

    let image = render_buffer.srgb().to_image_u8();

    image.save("image.png").expect("Could not save image");
}
