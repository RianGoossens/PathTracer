use nalgebra::Point3;

pub trait HasCoordinate {
    fn coordinate(&self) -> Point3<f64>;
}

#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    x_a: f64,
    x_b: f64,
    y_a: f64,
    y_b: f64,
    z_a: f64,
    z_b: f64,
}

impl BoundingBox {
    pub fn new(x_a: f64, x_b: f64, y_a: f64, y_b: f64, z_a: f64, z_b: f64) -> Self {
        Self {
            x_a,
            x_b,
            y_a,
            y_b,
            z_a,
            z_b,
        }
    }

    fn contains(&self, position: &Point3<f64>) -> bool {
        self.x_a <= position.x
            && self.x_b >= position.x
            && self.y_a <= position.y
            && self.y_b >= position.y
            && self.z_a <= position.z
            && self.z_b >= position.z
    }

    fn split_and_return_index(&mut self, location: &Point3<f64>) -> u8 {
        let x_mid = (self.x_a + self.x_b) / 2.;
        let y_mid = (self.y_a + self.y_b) / 2.;
        let z_mid = (self.z_a + self.z_b) / 2.;

        let mut index = 0;

        if location.x < x_mid {
            self.x_b = x_mid;
        } else {
            self.x_a = x_mid;
            index += 1;
        };
        if location.y < y_mid {
            self.y_b = y_mid;
        } else {
            self.y_a = y_mid;
            index += 2;
        };
        if location.z < z_mid {
            self.z_b = z_mid;
        } else {
            self.z_a = z_mid;
            index += 4;
        };
        index
    }
}

#[derive(Debug)]
pub struct Octree<T> {
    pub bounding_box: BoundingBox,
    nodes: OctreeNode<T>,
}

#[derive(Debug)]
pub enum OctreeNode<T> {
    Branch { children: [Box<OctreeNode<T>>; 8] },
    Leaf { content: Option<T> },
}

impl<T: HasCoordinate + Copy> OctreeNode<T> {
    fn new_filled_leaf(object: T) -> Self {
        Self::Leaf {
            content: Some(object),
        }
    }
    fn new_empty_leaf() -> Self {
        Self::Leaf { content: None }
    }
    fn new_empty_branch() -> Self {
        Self::Branch {
            children: [
                Box::new(OctreeNode::new_empty_leaf()),
                Box::new(OctreeNode::new_empty_leaf()),
                Box::new(OctreeNode::new_empty_leaf()),
                Box::new(OctreeNode::new_empty_leaf()),
                Box::new(OctreeNode::new_empty_leaf()),
                Box::new(OctreeNode::new_empty_leaf()),
                Box::new(OctreeNode::new_empty_leaf()),
                Box::new(OctreeNode::new_empty_leaf()),
            ],
        }
    }
    fn add(&mut self, mut bounding_box: BoundingBox, object: T) {
        match self {
            OctreeNode::Leaf { content: None } => *self = Self::new_filled_leaf(object),

            OctreeNode::Leaf {
                content: Some(existing),
            } => {
                let mut new_branch = Self::new_empty_branch();

                new_branch.add(bounding_box, *existing);
                new_branch.add(bounding_box, object);
                *self = new_branch
            }
            OctreeNode::Branch { children } => {
                let location = object.coordinate();

                let index = bounding_box.split_and_return_index(&location);

                children[index as usize].add(bounding_box, object);
            }
        }
    }

    fn find(&self, mut bounding_box: BoundingBox, position: &Point3<f64>) -> Option<&T> {
        match self {
            OctreeNode::Leaf { content } => content.as_ref(),
            OctreeNode::Branch { children } => {
                let index = bounding_box.split_and_return_index(position);
                children[index as usize].find(bounding_box, position)
            }
        }
    }
}

impl<T: HasCoordinate + Copy> Octree<T> {
    pub fn new(bounding_box: BoundingBox) -> Self {
        Self {
            bounding_box,
            nodes: OctreeNode::new_empty_leaf(),
        }
    }

    pub fn add(&mut self, object: T) -> bool {
        if self.bounding_box.contains(&object.coordinate()) {
            self.nodes.add(self.bounding_box, object);
            true
        } else {
            false
        }
    }

    pub fn find(&self, location: &Point3<f64>) -> Option<&T>
    where
        T: std::fmt::Debug,
    {
        if self.bounding_box.contains(location) {
            self.nodes.find(self.bounding_box, location)
        } else {
            None
        }
    }
}
