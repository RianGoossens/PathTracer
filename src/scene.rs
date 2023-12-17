use crate::{shape::IntersectionInfo, Camera, Object, Ray};

pub struct Scene {
    pub camera: Camera,
    pub objects: Vec<Object>,
    light_indices: Vec<usize>,
}

impl Scene {
    pub fn new(camera: Camera, objects: &[Object]) -> Self {
        let light_indices = objects
            .iter()
            .enumerate()
            .filter_map(|(i, object)| {
                if object.material.emissive {
                    Some(i)
                } else {
                    None
                }
            })
            .collect();
        Self {
            camera,
            objects: objects.into(),
            light_indices,
        }
    }

    pub fn intersection(&self, ray: &Ray) -> Option<(&Object, IntersectionInfo)> {
        let query = self
            .objects
            .iter()
            .flat_map(|object| {
                let intersection = object.local_intersection(ray);
                intersection.map(|intersection| (object, intersection))
            })
            .reduce(
                |closest @ (_, closest_intersection), current @ (_, intersection)| {
                    if intersection.distance < closest_intersection.distance {
                        current
                    } else {
                        closest
                    }
                },
            );

        if let Some((object, intersection)) = query {
            Some((
                object,
                intersection.transform(object.transform.matrix(), &object.inverse_transform),
            ))
        } else {
            None
        }
    }

    pub fn lights(&self) -> Vec<&Object> {
        self.light_indices
            .iter()
            .map(|i| &self.objects[*i])
            .collect()
    }
}
