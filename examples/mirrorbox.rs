use std::time::Instant;

use path_tracer::{
    aperture::RegularPolygonAperture, renderer::BDPTRenderer, shape::Cuboid, Camera, Inverted,
    Material, Object, Renderer, Scene, Sphere,
};

use nalgebra as na;

use na::{Similarity3, Vector3};

const NUM_SAMPLES: usize = 50;

fn main() {
    let aperture = RegularPolygonAperture::new(0.05, 5);
    let camera = Camera::new(300, 300, 110., 0.1, 100.0, aperture, 1.75);

    let mirror_material = Material {
        color: Vector3::new(0.5, 0.8, 0.5),
        roughness: 0.001,
        emissive: false,
    };

    let cube = Object::new(
        Inverted(Cuboid::new(2., 2., 2.)),
        Similarity3::new(Vector3::new(0., 0., 0.), Vector3::new(0., 0., 0.), 1.),
        mirror_material,
    );

    let light = Object::new(
        Sphere::new(1.),
        Similarity3::new(Vector3::new(0., 0.5, 0.), Vector3::zeros(), 0.1),
        Material {
            color: Vector3::new(1., 1., 1.) * 1.,
            emissive: true,
            roughness: 1.,
        },
    );

    let scene = Scene::new(camera, vec![cube, light]);

    let start = Instant::now();

    let renderer = BDPTRenderer::new(20).parallel(NUM_SAMPLES);
    let render_buffer = renderer.render(&scene);

    println!("Rendering took {:?}", start.elapsed());

    let image = render_buffer.srgb().to_image_u8();

    image.save("image.png").expect("Could not save image");
}
