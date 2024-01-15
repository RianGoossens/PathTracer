use nalgebra as na;

use na::Vector3;

pub trait Shader: Send + Sync {
    fn shade(&self, local_position: &Vector3<f64>) -> Vector3<f64>;
}

impl Shader for Vector3<f64> {
    fn shade(&self, _local_position: &Vector3<f64>) -> Vector3<f64> {
        *self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Checkerboard {
    color_a: Vector3<f64>,
    color_b: Vector3<f64>,
    scale: f64,
}

impl Checkerboard {
    pub fn new(color_a: Vector3<f64>, color_b: Vector3<f64>, scale: f64) -> Self {
        Self {
            color_a,
            color_b,
            scale,
        }
    }
}

impl Shader for Checkerboard {
    fn shade(&self, local_position: &Vector3<f64>) -> Vector3<f64> {
        let scaled = local_position / self.scale;

        if ((scaled.x + 10000.) as i32 % 2 == 0)
            ^ ((scaled.y + 10000.) as i32 % 2 == 0)
            ^ ((scaled.z + 10000.) as i32 % 2 == 0)
        {
            self.color_a
        } else {
            self.color_b
        }
    }
}
