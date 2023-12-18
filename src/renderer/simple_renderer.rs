use crate::{reflect, RenderBuffer, Renderer, Scene};

use nalgebra as na;

use na::Vector3;

pub struct SimpleRenderer;

impl Renderer for SimpleRenderer {
    fn render(&self, scene: &Scene) -> RenderBuffer {
        let width = scene.camera.width;
        let height = scene.camera.height;

        let mut render_buffer = RenderBuffer::new(width, height);

        for x in 0..width {
            for y in 0..height {
                let ray = scene.camera.get_ray(x, y);

                let intersection = scene.intersection(&ray);

                if let Some((_object, intersection)) = intersection {
                    let reflection = reflect(&ray.direction, &intersection.normal);

                    let light_direction =
                        (Vector3::new(0., 0., -5.) - intersection.position.coords).normalize();

                    let angle = reflection.dot(&light_direction);

                    let lightness = angle.max(0.);

                    render_buffer[(x, y)] = Vector3::new(lightness, lightness, lightness);
                }
            }
        }

        render_buffer
    }
}
