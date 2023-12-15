use std::rc::Rc;

use na::{Similarity3, Translation3, Vector3};
use nalgebra as na;
use path_tracer::{reflect, Camera, Material, Object, RenderBuffer, Scene, Sphere};

fn main() {
    let camera = Camera::new(200, 200, 70., 1.0, 100.0);

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
            Vector3::new(0., 0., -3.),
            Vector3::zeros(),
            1.,
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

    let mut render_buffer = RenderBuffer::new(camera.width, camera.height);

    for x in 0..camera.width {
        for y in 0..camera.height {
            let ray = camera.get_ray(x, y);

            let ray = ray.transform(&camera.inverse_transform);

            let intersection = scene.intersection(&ray);

            if let Some((_object, intersection)) = intersection {
                let reflection = reflect(ray.direction, intersection.normal);

                let light_direction =
                    (Vector3::new(0., 0., -5.) - intersection.position.coords).normalize();

                let angle = reflection.dot(&light_direction);

                let lightness = angle.max(0.);

                render_buffer[(x, y)] = Vector3::new(lightness, lightness, lightness);
            }
        }
    }

    let image = render_buffer.to_image();

    image.save("image.png").expect("Could not save image");
}
