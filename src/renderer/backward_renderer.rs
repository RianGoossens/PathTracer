use crate::{Ray, RenderBuffer, Renderer, Scene};

use na::Vector3;
use nalgebra as na;

pub struct BackwardRenderer {
    pub max_bounces: u8,
}

impl BackwardRenderer {
    pub fn new(max_bounces: u8) -> Self {
        Self { max_bounces }
    }
    fn sample_color(&self, ray: &Ray, scene: &Scene) -> Vector3<f64> {
        let mut current_color_filter = Vector3::new(1., 1., 1.);
        let mut current_emission = Vector3::zeros();
        let mut current_ray = *ray;

        for _bounce in 0..self.max_bounces {
            if let Some((material, intersection)) = scene.intersection(&current_ray) {
                let interaction = material.interact(&current_ray, &intersection);

                current_emission += interaction.emission.component_mul(&current_color_filter);
                current_color_filter.component_mul_assign(&interaction.color_filter);

                if let Some(outgoing) = interaction.outgoing {
                    current_ray = outgoing;
                } else {
                    break;
                }
            } else {
                current_color_filter *= 0.;
                break;
            }
        }

        current_emission
    }
}

impl Renderer for BackwardRenderer {
    fn render(&self, scene: &Scene) -> RenderBuffer {
        let width = scene.camera.width;
        let height = scene.camera.height;

        let mut render_buffer = RenderBuffer::new(width, height);

        for x in 0..width {
            for y in 0..height {
                let ray = scene.camera.get_ray(x, y);

                render_buffer[(x, y)] = self.sample_color(&ray, scene);
            }
        }

        render_buffer
    }
}
