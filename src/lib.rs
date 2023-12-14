use std::rc::Rc;

use nalgebra::{Matrix4, Point3, Projective3, Transform3, Vector3};

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Point3<f64>,
    pub direction: Vector3<f64>,
}

impl Ray {
    pub fn transform(&self, transform: &Matrix4<f64>) -> Ray {
        let new_origin = transform.transform_point(&self.origin);
        let new_direction = transform.transform_point(&(self.origin + self.direction)) - new_origin;
        Ray {
            origin: new_origin,
            direction: new_direction,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Hit {
    pub position: Point3<f64>,
    pub normal: Vector3<f64>,
}

pub trait Shape {
    fn intersection(&self, ray: &Ray) -> Vec<Hit>;

    fn blocks(&self, ray: &Ray) -> bool {
        !self.intersection(ray).is_empty()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Sphere;

impl Shape for Sphere {
    fn intersection(&self, ray: &Ray) -> Vec<Hit> {
        //length(origin + k * direction) == 1
        //sqrt(sum((origin + k * direction) ^ 2)) == 1
        //sum((origin + k * direction) ^ 2) == 1
        //sum(origin ^ 2 + 2 * k * origin * direction + k ^ 2 * direction ^ 2) == 1
        //sum(2 * k * origin * direction + k ^ 2 * direction ^ 2) == 1 - sum(origin ^ 2)
        //k * sum(2*origin*direction) + k ^ 2 * sum(direction ^ 2) == 1 - sum(origin ^ 2)
        //0 == k ^ 2 * sum(direction^2) + k * sum(2*origin*direction) + (sum(origin ^ 2) - 1)
        let a = ray.direction.dot(&ray.direction);
        let b = 2. * ray.origin.coords.dot(&ray.direction);
        let c = ray.origin.coords.dot(&ray.origin.coords) - 1.;

        let determinant = b * b - 4. * a * c;
        if determinant < 0. {
            vec![]
        } else {
            let d_root = determinant.sqrt();
            let t1 = (-b - d_root) / (2. * a);
            let t2 = (-b + d_root) / (2. * a);
            let position_a = ray.origin + ray.direction * t1;
            let position_b = ray.origin - ray.direction * t2;
            vec![
                Hit {
                    position: position_a,
                    normal: position_a.coords,
                },
                Hit {
                    position: position_b,
                    normal: position_b.coords,
                },
            ]
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Material {
    pub color: Vector3<f64>,
    pub roughness: f64,
}

#[derive(Clone)]
pub struct Object {
    pub shape: Rc<dyn Shape>,
    pub transform: Projective3<f64>,
    pub material: Material,
}

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    pub transform: Transform3<f64>,
}

#[derive(Clone, Copy, Debug)]
pub struct PointLight {
    pub position: Point3<f64>,
    pub color: Vector3<f64>,
}

pub struct Scene {
    pub camera: Camera,
    pub objects: Vec<Object>,
    pub lights: Vec<PointLight>,
}
