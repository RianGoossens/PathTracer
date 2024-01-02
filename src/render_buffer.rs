use std::ops::{AddAssign, DivAssign, Index, IndexMut};

use image::{Rgb, Rgb32FImage, RgbImage};
use nalgebra::Vector3;
#[derive(Debug, Clone)]
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

    pub fn median_filter(&self, kernel_size: usize) -> Self {
        let mut result = self.clone();
        let kernel_size = kernel_size as i32;
        for row in 0..self.height as i32 {
            for col in 0..self.width as i32 {
                let mut colors = vec![];
                for row_offset in -kernel_size / 2..=kernel_size / 2 {
                    let row_index = row + row_offset;
                    if row_index >= 0 && row_index < self.height as i32 {
                        for col_offset in -kernel_size / 2..=kernel_size / 2 {
                            let col_index = col + col_offset;
                            if col_index >= 0 && col_index < self.width as i32 {
                                let color = self[(row_index as u32, col_index as u32)];
                                colors.push(color);
                            }
                        }
                    }
                }
                colors.sort_by(|a, b| a.sum().total_cmp(&b.sum()));
                let color = colors[colors.len() / 2];
                result[(row as u32, col as u32)] = color;
            }
        }

        result
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
