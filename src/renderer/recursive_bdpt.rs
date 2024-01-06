use na::{Point3, Vector3};
use nalgebra as na;

use crate::{Material, Ray, RenderBuffer, Renderer, Scene};

#[derive(Clone, Copy)]
struct PathVertex<'a> {
    pub position: Point3<f64>,
    pub normal: Vector3<f64>,
    pub incoming: Vector3<f64>,
    pub material: &'a Material,
    pub accumulated_emission: Vector3<f64>,
}

pub struct RecursiveBDPT {
    max_bounces: u8,
}

impl RecursiveBDPT {
    pub fn new(max_bounces: u8) -> Self {
        Self { max_bounces }
    }

    fn sample_light_path<'a>(
        &'a self,
        ray: &'a Ray,
        scene: &'a Scene,
        material: &'a Material,
    ) -> Vec<PathVertex<'a>> {
        let emission = material.emission_color();

        let mut current_path = vec![];

        let mut current_ray = *ray;
        let mut accumulated_emission = emission;
        for _bounce in 0..self.max_bounces {
            if let Some((material, intersection)) = scene.intersection(&current_ray) {
                let interaction = material.interact(&current_ray, &intersection);
                let current_absorption = interaction.color_filter;
                let current_emission = interaction.emission;

                accumulated_emission =
                    accumulated_emission.component_mul(&current_absorption) + current_emission;

                let vertex = PathVertex {
                    position: intersection.position,
                    normal: intersection.normal.normalize(),
                    incoming: current_ray.direction,
                    material,
                    accumulated_emission,
                };

                current_path.push(vertex);
                if let Some(outgoing) = interaction.outgoing {
                    current_ray = outgoing;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        current_path
    }

    fn sample_camera_path(
        &self,
        ray: &Ray,
        scene: &Scene,
        light_path: &[PathVertex],
        bounce: u8,
    ) -> Vector3<f64> {
        if bounce >= self.max_bounces {
            return Vector3::zeros();
        }
        if let Some((material, intersection)) = scene.intersection(ray) {
            let interaction = material.interact(ray, &intersection);

            let current_position = &intersection.position;
            let current_normal = &intersection.normal;

            let mut current_color = Vector3::zeros();
            let mut total_importance = 0.;

            if let Some(outgoing) = &interaction.outgoing {
                let backward_path_importance = material.likelihood(
                    &ray.direction,
                    &outgoing.direction,
                    &interaction.intersection.normal,
                );
                //let backward_path_importance = 1.;

                let backward_path_color =
                    self.sample_camera_path(outgoing, scene, light_path, bounce + 1);

                current_color += backward_path_color * backward_path_importance;
                total_importance += backward_path_importance;

                for vertex_light in light_path {
                    if
                    /*current_normal.dot(&vertex_light.normal) < 0.
                    &&*/
                    scene.is_visible(current_position, &vertex_light.position) {
                        let light_color = vertex_light.accumulated_emission;

                        let difference = (vertex_light.position - current_position).normalize();

                        let ray_importance =
                            material.likelihood(&ray.direction, &difference, current_normal);

                        if ray_importance > 0. {
                            let light_importance = vertex_light.material.likelihood(
                                &vertex_light.incoming,
                                &-difference,
                                &vertex_light.normal,
                            );
                            current_color += light_color * ray_importance * light_importance; //ray_importance * light_importance * light_color;
                            total_importance += ray_importance;
                        }
                    }
                }
            }
            //println!("{total_importance} {current_color}");
            if total_importance > 0. {
                current_color /= total_importance;
            }
            current_color.component_mul_assign(&material.absorption_color());
            current_color += material.emission_color();
            current_color
        } else {
            Vector3::zeros()
        }
    }

    fn sample_color(&self, ray: &Ray, scene: &Scene) -> Vector3<f64> {
        let light = scene.random_light();

        let light_ray = light.sample_emissive_ray();
        let light_path = self.sample_light_path(&light_ray, scene, &light.material);

        self.sample_camera_path(ray, scene, &light_path, 0)
    }
}

impl Renderer for RecursiveBDPT {
    fn render(&self, scene: &crate::Scene) -> RenderBuffer {
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
