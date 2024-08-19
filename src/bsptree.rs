use core::f64;

use nalgebra::{Point3, Vector3};

use nalgebra as na;

#[derive(Clone, Copy, Debug)]
pub struct BSPTreeAxis(Vector3<f64>);

impl BSPTreeAxis {
    pub fn random() -> Self {
        Self(Vector3::<f64>::new_random().normalize())
    }

    pub fn get_value(&self, position: &Point3<f64>) -> f64 {
        position.coords.dot(&self.0)
    }
}

pub enum BSPTree<T> {
    Branch {
        axis: BSPTreeAxis,
        split: f64,
        left: Box<BSPTree<T>>,
        right: Box<BSPTree<T>>,
    },
    Leaf(BSPElement<T>),
    Empty,
}

#[derive(Clone, Copy)]
pub struct BSPElement<T> {
    pub element: T,
    pub position: Point3<f64>,
}

impl<T> BSPElement<T> {
    pub fn get_value(&self, axis: BSPTreeAxis) -> f64 {
        axis.get_value(&self.position)
    }
}

impl<T: Copy> BSPTree<T> {
    pub fn build(mut elements: Vec<BSPElement<T>>) -> Self {
        Self::build_impl(&mut elements, BSPTreeAxis::random())
    }
    fn build_impl(elements: &mut [BSPElement<T>], axis: BSPTreeAxis) -> Self {
        if elements.is_empty() {
            Self::Empty
        } else if elements.len() == 1 {
            Self::Leaf(elements[0])
        } else {
            elements.sort_by(|a, b| a.get_value(axis).total_cmp(&b.get_value(axis)));

            let median = elements[elements.len() / 2].get_value(axis);
            let (left, right) = elements.split_at_mut(elements.len() / 2);

            let left = Self::build_impl(left, BSPTreeAxis::random());
            let right = Self::build_impl(right, BSPTreeAxis::random());

            BSPTree::Branch {
                axis,
                split: median,
                left: Box::new(left),
                right: Box::new(right),
            }
        }
    }
    pub fn closest(&self, position: &Point3<f64>) -> Option<(&BSPElement<T>, f64)> {
        self.closest_impl(position, f64::MAX)
    }
    fn closest_impl(
        &self,
        position: &Point3<f64>,
        mut smallest_distance: f64,
    ) -> Option<(&BSPElement<T>, f64)> {
        match self {
            BSPTree::Empty => None,
            BSPTree::Leaf(result) => {
                let dist = na::distance(position, &result.position);
                if dist < smallest_distance {
                    Some((result, dist))
                } else {
                    None
                }
            }
            BSPTree::Branch {
                axis,
                split,
                left,
                right,
            } => {
                let value = axis.get_value(position);

                let use_left = value < *split;
                let (first, second) = if use_left {
                    (left, right)
                } else {
                    (right, left)
                };

                let mut best = first.closest_impl(position, smallest_distance);
                if let Some((_, best_distance)) = best {
                    smallest_distance = best_distance;
                }

                let axis_distance = (split - value).abs();
                if axis_distance < smallest_distance {
                    if let second_result @ Some((_, second_candidate_distance)) =
                        second.closest_impl(position, smallest_distance)
                    {
                        if second_candidate_distance < smallest_distance {
                            best = second_result;
                        }
                    }
                }
                best
            }
        }
    }

    pub fn samples_in_sphere<'a>(
        &'a self,
        center: &Point3<f64>,
        radius: f64,
        buffer: &mut Vec<&'a BSPElement<T>>,
    ) {
        self.samples_in_sphere_impl(center, radius, buffer);
    }

    fn samples_in_sphere_impl<'a>(
        &'a self,
        center: &Point3<f64>,
        radius: f64,
        buffer: &mut Vec<&'a BSPElement<T>>,
    ) {
        match self {
            BSPTree::Empty => {}
            BSPTree::Leaf(element) => {
                if na::distance_squared(center, &element.position) <= radius * radius {
                    buffer.push(element);
                }
            }
            BSPTree::Branch {
                axis,
                split,
                left,
                right,
            } => {
                let center_value = axis.get_value(center);

                if center_value + radius < *split {
                    left.samples_in_sphere_impl(center, radius, buffer);
                } else if center_value - radius > *split {
                    right.samples_in_sphere_impl(center, radius, buffer);
                } else {
                    left.samples_in_sphere_impl(center, radius, buffer);
                    right.samples_in_sphere_impl(center, radius, buffer);
                }
            }
        }
    }
}
