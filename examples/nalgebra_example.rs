use nalgebra::{Matrix4, Vector4};
use rand::*;

fn main() {
    let mut rng = rand::thread_rng();

    let a = Vector4::new(
        rng.gen::<f32>(),
        rng.gen::<f32>(),
        rng.gen::<f32>(),
        rng.gen::<f32>(),
    );
    let b = Matrix4::new(
        rng.gen::<f32>(),
        rng.gen::<f32>(),
        rng.gen::<f32>(),
        rng.gen::<f32>(),
        rng.gen::<f32>(),
        rng.gen::<f32>(),
        rng.gen::<f32>(),
        rng.gen::<f32>(),
        rng.gen::<f32>(),
        rng.gen::<f32>(),
        rng.gen::<f32>(),
        rng.gen::<f32>(),
        rng.gen::<f32>(),
        rng.gen::<f32>(),
        rng.gen::<f32>(),
        rng.gen::<f32>(),
    );
    let c = b.try_inverse().unwrap() * a;
    println!("{}", c);
}
