use crate::{shape::IntersectionInfo, Camera, Object, PointLight, Ray};

pub struct Scene {
    pub camera: Camera,
    pub objects: Vec<Object>,
    pub lights: Vec<PointLight>,
}

impl Scene {
    pub fn intersection(&self, ray: &Ray) -> Option<(&Object, IntersectionInfo)> {
        let query = self
            .objects
            .iter()
            .filter_map(|object| {
                let intersection = object.local_intersection(ray);
                intersection.map(|intersection| (object, intersection))
            })
            .min_by(|(_, a), (_, b)| a.distance.total_cmp(&b.distance));

        if let Some((object, intersection)) = query {
            Some((
                object,
                intersection.transform(object.transform.matrix(), &object.inverse_transform),
            ))
        } else {
            None
        }
    }
}
