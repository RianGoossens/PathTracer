use std::time::Instant;

use path_tracer::{
    aperture::RegularPolygonAperture, object::ObjectDefinition, renderer::ParallelRenderer,
    BackwardRenderer, Camera, Material, Renderer, Scene, Sphere,
};

use nalgebra as na;

use na::Vector3;

const NUM_SAMPLES: usize = 1000;

fn main() {
    let aperture = RegularPolygonAperture::new(1., 3);
    let focal_distance = 3.5;
    let camera = Camera::new_at_origin(300, 300, 55., 1.0, 100.0, aperture, focal_distance);

    let sphere_shape = Sphere::new(1.);

    let light = ObjectDefinition {
        shape: Box::new(sphere_shape),
        material: Material::new(Vector3::new(3., 3., 3.), 0., true),
        z: -5.,
        scale: 0.25,
        ..Default::default()
    };

    let scene = Scene::new(camera, vec![light]);

    let start = Instant::now();
    //let renderer = BackwardRenderer::new(5).parallel_rayon(NUM_SAMPLES);

    let renderer = ParallelRenderer::new(BackwardRenderer::new(5), NUM_SAMPLES);

    let render_buffer = renderer.render(&scene);

    println!("Rendering took {:?}", start.elapsed());

    let image = render_buffer.to_image_u8();

    image.save("image.png").expect("Could not save image");
}
