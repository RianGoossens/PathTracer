use std::thread;

use crate::{RenderBuffer, Scene};

mod backward_renderer;
mod bdpt_renderer;
mod depth_renderer;
mod simple_renderer;

pub use backward_renderer::BackwardRenderer;
pub use bdpt_renderer::BDPTRenderer;
pub use depth_renderer::{DepthRenderMode, DepthRenderer};
pub use simple_renderer::SimpleRenderer;

pub trait Renderer: Send + Sync {
    fn render(&self, scene: &Scene) -> RenderBuffer;

    fn iterative(self, num_samples: usize) -> IterativeRenderer<Self>
    where
        Self: Sized,
    {
        IterativeRenderer::new(self, num_samples)
    }

    fn parallel(self, num_samples: usize) -> ParallelRenderer<Self>
    where
        Self: Sized,
    {
        ParallelRenderer::new(self, num_samples)
    }
}
pub struct IterativeRenderer<R: Renderer> {
    renderer: R,
    num_samples: usize,
}

impl<R: Renderer> IterativeRenderer<R> {
    pub fn new(renderer: R, num_samples: usize) -> Self {
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

#[derive(Debug, Clone, Copy)]
pub struct ParallelRenderer<R: Renderer> {
    renderer: R,
    num_samples: usize,
}

impl<R: Renderer> ParallelRenderer<R> {
    pub fn new(renderer: R, num_samples: usize) -> Self {
        Self {
            renderer,
            num_samples,
        }
    }
}

impl<R: Renderer> Renderer for ParallelRenderer<R> {
    fn render(&self, scene: &Scene) -> RenderBuffer {
        let width = scene.camera.width;
        let height = scene.camera.height;

        let mut render_buffer = RenderBuffer::new(width, height);

        let num_threads: usize = thread::available_parallelism().unwrap().into();
        let num_threads = num_threads.min(self.num_samples);

        let division = self.num_samples / num_threads;
        let remainder = self.num_samples % num_threads;
        let samples_per_thread = (0..num_threads)
            .map(|i| {
                if i < remainder {
                    division + 1
                } else {
                    division
                }
            })
            .collect::<Vec<_>>();

        thread::scope(|s| {
            let thread_handles = samples_per_thread
                .into_iter()
                .map(|num_samples| {
                    s.spawn(move || {
                        let mut render_buffer = self.renderer.render(scene);

                        for _ in 0..num_samples - 1 {
                            render_buffer += self.renderer.render(scene);
                        }

                        render_buffer
                    })
                })
                .collect::<Vec<_>>();

            for handle in thread_handles {
                let thread_render_buffer = handle.join().unwrap();
                render_buffer += thread_render_buffer;
            }
        });

        render_buffer /= self.num_samples as f64;
        render_buffer
    }
}
