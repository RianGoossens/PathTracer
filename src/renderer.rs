use crate::{reflect, Ray, RenderBuffer, Scene};

use na::Vector3;
use nalgebra as na;

pub struct SimpleRenderer;

impl SimpleRenderer {
    pub fn render(&self, scene: &Scene) -> RenderBuffer {
        let width = scene.camera.width;
        let height = scene.camera.height;

        let mut render_buffer = RenderBuffer::new(width, height);

        for x in 0..width {
            for y in 0..height {
                let ray = scene
                    .camera
                    .get_ray(x, y)
                    .expect("Ray requested was out of bounds");

                let intersection = scene.intersection(ray);

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

pub struct Renderer;

const MAX_BOUNCES: u8 = 5;
impl Renderer {
    pub fn render(&self, scene: &Scene) -> RenderBuffer {
        let width = scene.camera.width;
        let height = scene.camera.height;

        let mut render_buffer = RenderBuffer::new(width, height);

        for x in 0..width {
            for y in 0..height {
                let ray = scene
                    .camera
                    .get_ray(x, y)
                    .expect("Ray requested was out of bounds");

                render_buffer[(x, y)] = self.sample_color(ray, scene);
            }
        }

        render_buffer
    }

    fn sample_color(&self, ray: &Ray, scene: &Scene) -> Vector3<f64> {
        let mut current_color_filter = Vector3::new(1., 1., 1.);
        let mut current_emission = Vector3::zeros();
        let mut current_ray = *ray;

        for _bounce in 0..MAX_BOUNCES {
            if let Some((object, intersection)) = scene.intersection(&current_ray) {
                let interaction = object.material.interact(&current_ray, &intersection);

                current_color_filter.component_mul_assign(&interaction.color_filter);
                current_emission += interaction.emission;

                if let Some(new_ray) = interaction.outgoing {
                    current_ray = new_ray;
                } else {
                    break;
                }
            } else {
                current_color_filter *= 0.;
                break;
            }
        }

        current_color_filter.component_mul(&current_emission)
    }
}
