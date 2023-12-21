use std::f64::consts::PI;

use nalgebra as na;

use na::{Point3, Similarity3, Vector3};
use rand::{thread_rng, Rng};
use rand_distr::StandardNormal;

pub struct Cuboid {
    pub width: f64,
    pub height: f64,
    pub depth: f64,
}
