use nalgebra::{Isometry3, Point3, Similarity3, Vector3};

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Point3<f64>,
    pub direction: Vector3<f64>,
}

impl Ray {
    pub fn transform_similarity(&self, transform: &Similarity3<f64>) -> Ray {
        let new_origin = transform.transform_point(&self.origin);
        let new_direction = transform.transform_point(&(self.origin + self.direction)) - new_origin;
        Ray {
            origin: new_origin,
            direction: new_direction,
        }
    }

    pub fn transform_isometry(&self, transform: &Isometry3<f64>) -> Ray {
        let new_origin = transform.transform_point(&self.origin);
        let new_direction = transform.transform_point(&(self.origin + self.direction)) - new_origin;
        Ray {
            origin: new_origin,
            direction: new_direction,
        }
    }

    pub fn sample(&self, t: f64) -> Point3<f64> {
        self.origin + t * self.direction
    }
}
