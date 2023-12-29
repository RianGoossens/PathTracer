use nalgebra::Vector3;

use crate::{RenderBuffer, Renderer};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DepthRenderMode {
    Raw,
    Normalized,
}

#[derive(Debug, Clone, Copy)]
pub struct DepthRenderer {
    depth_render_mode: DepthRenderMode,
}

impl DepthRenderer {
    pub fn new(depth_render_mode: DepthRenderMode) -> Self {
        Self { depth_render_mode }
    }
}

impl Renderer for DepthRenderer {
    fn render(&self, scene: &crate::Scene) -> RenderBuffer {
        let mut max_depth: f64 = 0.;
        let mut min_depth: f64 = scene.camera.perspective.zfar();

        let mut buffer = vec![0.; scene.camera.width as usize * scene.camera.height as usize];
        for x in 0..scene.camera.width {
            for y in 0..scene.camera.height {
                let ray = scene.camera.get_ray(x, y);

                if let Some((_, intersection)) = scene.intersection(&ray) {
                    buffer[y as usize * scene.camera.width as usize + x as usize] =
                        intersection.distance;

                    if intersection.distance > max_depth {
                        max_depth = intersection.distance;
                    }
                    if intersection.distance < min_depth {
                        min_depth = intersection.distance;
                    }
                }
            }
        }

        if self.depth_render_mode == DepthRenderMode::Normalized {
            for x in &mut buffer {
                *x = 1. - (*x - min_depth) / (max_depth - min_depth);
            }
        }

        let mut render_buffer = RenderBuffer::new(scene.camera.width, scene.camera.height);
        for x in 0..scene.camera.width {
            for y in 0..scene.camera.height {
                let depth = buffer[y as usize * scene.camera.width as usize + x as usize];
                render_buffer[(x, y)] = Vector3::new(depth, depth, depth);
            }
        }

        render_buffer
    }
}
