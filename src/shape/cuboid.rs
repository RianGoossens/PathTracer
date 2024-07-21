use crate::{Ray, Shape};

use nalgebra as na;

use na::{Point3, Vector3};
use rand::{thread_rng, Rng};
use rand_distr::WeightedAliasIndex;

pub struct Cuboid {
    pub width: f64,
    pub height: f64,
    pub depth: f64,
}

impl Cuboid {
    pub fn new(width: f64, height: f64, depth: f64) -> Self {
        Self {
            width,
            height,
            depth,
        }
    }
}

impl Shape for Cuboid {
    fn intersection_distance(&self, ray: &Ray) -> Option<f64> {
        let mut result = f64::INFINITY;

        let ts = [
            (-ray.origin.x - self.width / 2.) / ray.direction.x,
            (-ray.origin.x + self.width / 2.) / ray.direction.x,
            (-ray.origin.y - self.height / 2.) / ray.direction.y,
            (-ray.origin.y + self.height / 2.) / ray.direction.y,
            (-ray.origin.z - self.depth / 2.) / ray.direction.z,
            (-ray.origin.z + self.depth / 2.) / ray.direction.z,
        ];
        for t in ts {
            if t >= 0. && t < result {
                let position = ray.sample(t);
                if -self.width / 2. <= position.x
                    && position.x <= self.width / 2.
                    && -self.height / 2. <= position.y
                    && position.y <= self.height / 2.
                    && -self.depth / 2. <= position.z
                    && position.z <= self.depth / 2.
                {
                    result = t;
                }
            }
        }

        if result.is_finite() {
            Some(result)
        } else {
            //println!("{ts:?} {ray:?}");
            None
        }
    }

    fn sample_normal(&self, position: Point3<f64>) -> Vector3<f64> {
        let normalized_position =
            position
                .coords
                .component_div(&Vector3::new(self.width, self.height, self.depth));

        let biggest_index = (0..3)
            .max_by(|a, b| {
                normalized_position[*a]
                    .abs()
                    .total_cmp(&normalized_position[*b].abs())
            })
            .unwrap();

        let mut normal = Vector3::zeros();
        normal[biggest_index] = position[biggest_index].signum();

        normal
    }

    fn sample_random_point(&self) -> Point3<f64> {
        let mut rng = thread_rng();

        let distribution = WeightedAliasIndex::new(vec![
            self.width * self.height,
            self.height * self.depth,
            self.width * self.depth,
            self.width * self.height,
            self.height * self.depth,
            self.width * self.depth,
        ])
        .unwrap();

        let direction: u8 = rng.sample(distribution) as u8;
        let x = rng.gen_range(-0.5..=0.5);
        let y = rng.gen_range(-0.5..=0.5);
        let z = if direction < 3 { -0.5 } else { 0.5 };

        let unit_position = match direction % 3 {
            0 => Vector3::new(x, y, z),
            1 => Vector3::new(z, x, y),
            2 => Vector3::new(y, z, x),
            _ => panic!("direction % 3 is somehow bigger than 2"),
        };

        unit_position
            .component_mul(&Vector3::new(self.width, self.height, self.depth))
            .into()
    }

    fn area(&self) -> f64 {
        2. * (self.width * (self.height + self.depth) + self.height * self.depth)
    }
}
