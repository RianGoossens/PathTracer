use std::ops::{Index, IndexMut};

use image::{Rgb, RgbImage};
use nalgebra::Vector3;

pub struct RenderBuffer {
    width: u32,
    height: u32,
    buffer: Vec<Vector3<f64>>,
}

impl RenderBuffer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            buffer: vec![Vector3::zeros(); (width * height) as usize],
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn to_image(&self) -> RgbImage {
        RgbImage::from_fn(self.width, self.height, |x, y| {
            let vector = &self[(x, y)];
            let r = (vector.x.clamp(0., 1.) * 255.) as u8;
            let g = (vector.y.clamp(0., 1.) * 255.) as u8;
            let b = (vector.z.clamp(0., 1.) * 255.) as u8;
            Rgb([r, g, b])
        })
    }
}

impl Index<(u32, u32)> for RenderBuffer {
    type Output = Vector3<f64>;

    fn index(&self, (row, column): (u32, u32)) -> &Self::Output {
        let index = row * self.height + column;
        &self.buffer[index as usize]
    }
}

impl IndexMut<(u32, u32)> for RenderBuffer {
    fn index_mut(&mut self, (row, column): (u32, u32)) -> &mut Self::Output {
        let index = row * self.height + column;
        &mut self.buffer[index as usize]
    }
}
