use nalgebra::Point3;

use nalgebra as na;

#[derive(Clone, Copy, Debug)]
pub enum BSPTreeAxis {
    X,
    Y,
    Z,
}

impl BSPTreeAxis {
    pub fn next(&self) -> Self {
        match self {
            BSPTreeAxis::X => BSPTreeAxis::Y,
            BSPTreeAxis::Y => BSPTreeAxis::Z,
            BSPTreeAxis::Z => BSPTreeAxis::X,
        }
    }
    pub fn get_value(&self, position: &Point3<f64>) -> f64 {
        match self {
            BSPTreeAxis::X => position.x,
            BSPTreeAxis::Y => position.y,
            BSPTreeAxis::Z => position.z,
        }
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
        match axis {
            BSPTreeAxis::X => self.position.x,
            BSPTreeAxis::Y => self.position.y,
            BSPTreeAxis::Z => self.position.z,
        }
    }
}

impl<T: Copy> BSPTree<T> {
    pub fn build(elements: Vec<BSPElement<T>>) -> Self {
        Self::build_impl(elements, BSPTreeAxis::X)
    }
    fn build_impl(mut elements: Vec<BSPElement<T>>, axis: BSPTreeAxis) -> Self {
        if elements.is_empty() {
            Self::Empty
        } else if elements.len() == 1 {
            Self::Leaf(elements[0])
        } else {
            elements.sort_by(|a, b| a.get_value(axis).total_cmp(&b.get_value(axis)));

            let median = elements[elements.len() / 2].get_value(axis);
            let (left, right) = elements.split_at(elements.len() / 2);

            let left = Self::build_impl(left.to_owned(), axis.next());
            let right = Self::build_impl(right.to_owned(), axis.next());

            BSPTree::Branch {
                axis,
                split: median,
                left: Box::new(left),
                right: Box::new(right),
            }
        }
    }
    pub fn closest(&self, position: &Point3<f64>) -> Option<(&BSPElement<T>, f64)> {
        match self {
            BSPTree::Empty => None,
            BSPTree::Leaf(result) => Some((result, na::distance(position, &result.position))),
            BSPTree::Branch {
                axis,
                split,
                left,
                right,
            } => {
                let value = axis.get_value(position);

                let use_left = value < *split;
                let first_to_consider = if use_left { left } else { right };
                let second_to_consider = if use_left { right } else { left };

                if let first_result @ Some((_, candidate_distance)) =
                    first_to_consider.closest(position)
                {
                    let axis_distance = (split - value).abs();
                    if candidate_distance > axis_distance {
                        if let second_result @ Some((_, second_candidate_distance)) =
                            second_to_consider.closest(position)
                        {
                            if second_candidate_distance < candidate_distance {
                                second_result
                            } else {
                                first_result
                            }
                        } else {
                            first_result
                        }
                    } else {
                        first_result
                    }
                } else {
                    second_to_consider.closest(position)
                }
            }
        }
    }
}
