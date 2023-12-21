use std::{f64::consts::PI, time::Instant};

use path_tracer::{
    aperture::{GaussianAperture, PinholeAperture},
    renderer::{BDPTRenderer, DepthRenderMode, DepthRenderer},
    shape::Plane,
    BackwardRenderer, Camera, Material, Object, Renderer, Scene, Sphere,
};

use nalgebra as na;

use na::{Similarity3, Vector3};

const NUM_SAMPLES: usize = 100;

fn main() {
    let aperture = GaussianAperture::new(0.05);
    let camera = Camera::new(300, 300, 90., 0.1, 100.0, aperture, 1.);

    let mirror_material = Material {
        color: Vector3::new(0.5, 1., 0.5),
        roughness: 0.01,
        emissive: false,
    };

    let back = Object::new(
        Plane::new(2., 2.),
        Similarity3::new(Vector3::new(0., 0., -2.), Vector3::new(0., 0., 0.), 1.),
        mirror_material,
    );
    let left = Object::new(
        Plane::new(2., 2.),
        Similarity3::new(
            Vector3::new(-1., 0., -2.),
            Vector3::new(0., PI / 2., 0.),
            1.,
        ),
        mirror_material,
    );
    let right = Object::new(
        Plane::new(2., 2.),
        Similarity3::new(
            Vector3::new(1., 0., -2.),
            Vector3::new(0., -PI / 2., 0.),
            1.,
        ),
        mirror_material,
    );
    let top = Object::new(
        Plane::new(2., 2.),
        Similarity3::new(Vector3::new(0., 1., -2.), Vector3::new(PI / 2., 0., 0.), 1.),
        mirror_material,
    );
    let bottom = Object::new(
        Plane::new(2., 2.),
        Similarity3::new(
            Vector3::new(0., -1., -2.),
            Vector3::new(-PI / 2., 0., 0.),
            1.,
        ),
        mirror_material,
    );

    let light = Object::new(
        Sphere::new(1.),
        Similarity3::new(Vector3::new(0., 0.5, -1.), Vector3::zeros(), 0.1),
        Material {
            color: Vector3::new(1., 1., 1.) * 1.,
            emissive: true,
            roughness: 1.,
        },
    );

    let scene = Scene::new(camera, vec![back, left, right, top, bottom, light]);

    let start = Instant::now();

    let renderer = BDPTRenderer::new(10).parallel(NUM_SAMPLES);
    let render_buffer = renderer.render(&scene);

    println!("Rendering took {:?}", start.elapsed());

    let image = render_buffer.srgb().to_image_u8();

    image.save("image.png").expect("Could not save image");
}
