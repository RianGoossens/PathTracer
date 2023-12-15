use std::rc::Rc;

use image::{Rgb, RgbImage};
use na::{Similarity3, Translation3, Vector3};
use nalgebra as na;
use path_tracer::{Camera, Material, Object, Scene, Sphere};

fn main() {
    let camera = Camera::new(300, 200, 70., 1.0, 100.0);

    let sphere_shape = Rc::new(Sphere);

    let sphere_a = Object::new(
        sphere_shape.clone(),
        na::convert(Translation3::new(1., 2., -5.)),
        Material {
            color: Vector3::new(1., 0., 0.),
            roughness: 0.5,
        },
    );

    let sphere_b = Object::new(
        sphere_shape,
        na::convert(Similarity3::new(
            Vector3::new(-2.5, -1., -3.),
            Vector3::zeros(),
            1.5,
        )),
        Material {
            color: Vector3::new(1., 0., 0.),
            roughness: 0.5,
        },
    );

    let scene = Scene {
        camera,
        objects: vec![sphere_a, sphere_b],
        lights: vec![],
    };

    let image = RgbImage::from_fn(camera.width, camera.height, |x, y| {
        let ray = camera.get_ray(x, y);

        let ray = ray.transform(&camera.inverse_transform);

        let intersection = scene.intersection(&ray);

        if let Some((_object, intersection)) = intersection {
            let reflection =
                ray.direction - 2. * intersection.normal.dot(&ray.direction) * intersection.normal;

            let light_direction =
                (Vector3::new(0., 0., -4.) - intersection.position.coords).normalize();

            let angle = reflection.dot(&light_direction);

            let lightness = (angle.max(0.) * 255.) as u8;

            Rgb([lightness, lightness, lightness])
        } else {
            Rgb([0, 0, 0])
        }
    });

    image.save("image.png").expect("Could not save image");
}
