use std::f64::consts::PI;

use na::Vector3;
use nalgebra as na;
use path_tracer::reflect;
use rand::thread_rng;
use rand_distr::StandardNormal;

fn main() {
    let distribution = StandardNormal;
    let mut rng = thread_rng();
    for _ in 0..100 {
        let vector_a: Vector3<f64> =
            Vector3::from_distribution(&distribution, &mut rng).normalize();
        let vector_b: Vector3<f64> =
            Vector3::from_distribution(&distribution, &mut rng).normalize();

        let angle = vector_a.dot(&vector_b).acos() / PI * 180.;

        let reverse = reflect(vector_a, vector_b);

        let reverse_angle = reverse.dot(&vector_b).acos() / PI * 180.;

        println!("{angle} {reverse_angle}");
    }
}
