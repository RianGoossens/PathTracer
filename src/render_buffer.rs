use std::ops::{AddAssign, DivAssign, Index, IndexMut};

use image::{Rgb, Rgb32FImage, RgbImage};
use nalgebra::Vector3;
#[derive(Debug)]
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

    pub fn map<F: Fn(&Vector3<f64>) -> Vector3<f64>>(&self, f: F) -> Self {
        Self {
            buffer: self.buffer.iter().map(&f).collect(),
            ..*self
        }
    }

    pub fn map_float<F: Fn(f64) -> f64>(&self, f: F) -> Self {
        self.map(|vector| vector.map(&f))
    }

    pub fn srgb(&self) -> Self {
        self.map_float(|x| x.clamp(0., 1.).powf(1. / 2.2))
    }

    pub fn to_image_u8(&self) -> RgbImage {
        RgbImage::from_fn(self.width, self.height, |x, y| {
            let vector = &self[(x, y)].map(|x| x * 255.);
            let r = vector.x as u8;
            let g = vector.y as u8;
            let b = vector.z as u8;
            Rgb([r, g, b])
        })
    }

    pub fn to_image_f32(&self) -> Rgb32FImage {
        Rgb32FImage::from_fn(self.width, self.height, |x, y| {
            let vector = &self[(x, y)];
            let r = vector.x as f32;
            let g = vector.y as f32;
            let b = vector.z as f32;
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

impl AddAssign<Self> for RenderBuffer {
    fn add_assign(&mut self, rhs: Self) {
        for x in 0..self.width.min(rhs.width) {
            for y in 0..self.height.min(rhs.height) {
                self[(x, y)] += rhs[(x, y)];
            }
        }
    }
}

impl DivAssign<f64> for RenderBuffer {
    fn div_assign(&mut self, rhs: f64) {
        for vector in &mut self.buffer {
            vector.iter_mut().for_each(|x| *x /= rhs);
        }
    }
}
