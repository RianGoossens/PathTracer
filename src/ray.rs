use nalgebra::{Point3, Similarity3, Vector3};

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
}
