use crate::{RenderBuffer, Scene};

mod backward_renderer;
mod depth_renderer;
mod simple_renderer;

pub use backward_renderer::BackwardRenderer;
pub use simple_renderer::SimpleRenderer;

pub trait Renderer {
    fn render(&self, scene: &Scene) -> RenderBuffer;

    fn iterative(self, num_samples: u32) -> IterativeRenderer<Self>
    where
        Self: Sized,
    {
        IterativeRenderer::new(self, num_samples)
    }

    fn parallel(self, num_samples: u32) -> ParallelRenderer<Self>
    where
        Self: Sized,
    {
        ParallelRenderer::new(self, num_samples)
    }
}
pub struct IterativeRenderer<R: Renderer> {
    renderer: R,
    num_samples: u32,
}

impl<R: Renderer> IterativeRenderer<R> {
    pub fn new(renderer: R, num_samples: u32) -> Self {
        Self {
            renderer,
            num_samples,
        }
    }
}

impl<R: Renderer> Renderer for IterativeRenderer<R> {
    fn render(&self, scene: &Scene) -> RenderBuffer {
        let mut render_buffer = self.renderer.render(scene);

        for _ in 0..self.num_samples - 1 {
            render_buffer += self.renderer.render(scene);
        }

        render_buffer /= self.num_samples as f64;
        render_buffer
    }
}
pub struct ParallelRenderer<R: Renderer> {
    renderer: R,
    num_samples: u32,
}

impl<R: Renderer> ParallelRenderer<R> {
    pub fn new(renderer: R, num_samples: u32) -> Self {
        Self {
            renderer,
            num_samples,
        }
    }
}

use rayon::prelude::*;
impl<R: Renderer + Sync> Renderer for ParallelRenderer<R> {
    fn render(&self, scene: &Scene) -> RenderBuffer {
        let renderer = &self.renderer;

        let buffers: Vec<_> = (0..self.num_samples)
            .into_par_iter()
            .map(move |_| renderer.render(scene))
            .collect();

        let mut render_buffer = RenderBuffer::new(buffers[0].width(), buffers[0].height());

        for buffer in buffers {
            render_buffer += buffer;
        }

        render_buffer /= self.num_samples as f64;
        render_buffer
    }
}
