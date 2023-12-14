use std::rc::Rc;

use image::{Rgb, RgbImage};
use na::Point3;
use nalgebra::{self as na, Perspective3, Transform3, Translation3, Vector3};
use path_tracer::{Camera, Material, Object, Ray, Sphere};

fn main() {
    let camera = Camera {
        transform: Transform3::from_matrix_unchecked(
            *Perspective3::<f64>::new(1., std::f64::consts::PI / 4.0, 1.0, 100.0).as_matrix(),
        ),
    };

    let sphere = Object {
        shape: Rc::new(Sphere),
        material: Material {
            color: Vector3::new(1., 0., 0.),
            roughness: 0.5,
        },
        transform: na::convert(Translation3::new(0., 0., -5.)),
    };

    let image = RgbImage::from_fn(100, 100, |x, y| {
        let x = x as f64 / 50. - 1.;
        let y = y as f64 / 50. - 1.;
        let origin = Point3::new(x, y, -1.);
        let direction = Vector3::new(0., 0., 1.);
        let ray = Ray { origin, direction };

        let inverse_camera = camera.transform.try_inverse().unwrap();
        let mut camera_ray = ray.transform(inverse_camera.matrix());
        camera_ray.direction.normalize_mut();
        println!("{camera_ray:?}");

        let object_ray = camera_ray.transform(sphere.transform.inverse().matrix());

        if sphere.shape.blocks(&object_ray) {
            Rgb([255, 255, 255])
        } else {
            Rgb([0, 0, 0])
        }
    });

    image.save("image.png").expect("Could not save image");
}
