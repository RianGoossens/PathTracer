use std::f64::INFINITY;

use nalgebra::Point3;
use rand::{thread_rng, Rng};
use rand_distr::WeightedAliasIndex;

use crate::{shape::IntersectionInfo, Camera, Material, Object, Ray};

pub struct Scene {
    pub camera: Camera,
    pub objects: Vec<Object>,
    light_indices: Vec<usize>,
    light_distribution: WeightedAliasIndex<f64>,
}

impl Scene {
    pub fn new(camera: Camera, objects: Vec<Object>) -> Self {
        let light_indices: Vec<usize> = objects
            .iter()
            .enumerate()
            .filter_map(|(i, object)| {
                if matches!(object.material(), Material::Emissive { .. }) {
                    Some(i)
                } else {
                    None
                }
            })
            .collect();

        let light_areas = light_indices.iter().map(|i| objects[*i].area()).collect();
        let light_distribution = WeightedAliasIndex::new(light_areas).unwrap();
        Self {
            camera,
            objects,
            light_indices,
            light_distribution,
        }
    }

    pub fn is_visible(&self, origin: &Point3<f64>, destination: &Point3<f64>) -> bool {
        let difference = destination - origin;
        let direction = difference.normalize();
        let distance = difference.magnitude();

        let ray = Ray {
            origin: origin + direction * 0.001,
            direction,
        };

        if let Some((_, intersection)) = self.intersection(&ray) {
            distance <= intersection.distance + 0.002
        } else {
            true
        }
    }

    pub fn intersection(&self, ray: &Ray) -> Option<(&Object, IntersectionInfo)> {
        let mut closest_intersection = None;
        let mut closest_distance = INFINITY;

        for object in &self.objects {
            if let Some(intersection) = object.local_intersection(ray) {
                if intersection.distance >= 0. && intersection.distance < closest_distance {
                    closest_intersection = Some((object, intersection));
                    closest_distance = intersection.distance;
                }
            }
        }

        closest_intersection.map(|(object, intersection)| {
            (
                object,
                intersection.transform_similarity(object.transform()),
            )
        })
    }

    pub fn random_light(&self) -> &Object {
        let mut rng = thread_rng();
        let index = rng.sample(&self.light_distribution);
        &self.objects[self.light_indices[index]]
    }
}
