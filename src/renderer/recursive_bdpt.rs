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
        let emission = material.emission_color(); // / light_area;

        let mut current_path = vec![PathVertex {
            position: ray.origin,
            normal: ray.direction,
            incoming: -ray.direction,
            material,
            accumulated_emission: emission,
        }];

        let mut current_ray = *ray;
        let mut accumulated_emission = emission;
        for _bounce in 0..self.max_bounces {
            if let Some((object, intersection)) = scene.intersection(&current_ray) {
                let material = object.material();
                let interaction = material.interact(&current_ray, &intersection);

                accumulated_emission = (accumulated_emission.component_mul(&interaction.filter)
                    + interaction.emission)
                    * interaction.pdf;

                let vertex = PathVertex {
                    position: interaction.position,
                    normal: interaction.surface_normal,
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
        ray: &Ray,
        scene: &Scene,
        light_path: &[PathVertex],
        bounces_left: u8,
    ) -> Vector3<f64> {
        if bounces_left == 0 {
            return Vector3::zeros();
        }
        if let Some((object, intersection)) = scene.intersection(ray) {
            let current_position = &intersection.position;
            let current_normal = &intersection.normal;
            let material = object.material();

            let mut current_color = Vector3::zeros();
            let mut total_likelihood = 0.;

            let interaction = material.interact(ray, &intersection);
            if let Some(outgoing) = &interaction.outgoing {
                let backward_path_color =
                    Self::sample_camera_path(outgoing, scene, light_path, bounces_left - 1);

                current_color += backward_path_color * interaction.pdf;
                total_likelihood += interaction.pdf;

                for vertex_light in light_path {
                    if scene.is_visible(current_position, &vertex_light.position) {
                        let light_color = vertex_light.accumulated_emission;

                        let light_to_camera_connection =
                            (current_position - vertex_light.position).normalize();

                        let light_importance = vertex_light.material.likelihood(
                            &vertex_light.incoming,
                            &light_to_camera_connection,
                            &vertex_light.normal,
                        );

                        if light_importance > 0. {
                            let ray_importance = material.likelihood(
                                &light_to_camera_connection,
                                &-ray.direction,
                                current_normal,
                            );

                            if ray_importance > 0. {
                                current_color += light_color * ray_importance * light_importance;
                                total_likelihood += ray_importance;
                            }
                        }
                    }
                }
            }
            if total_likelihood > 0. {
                current_color /= total_likelihood;
            }
            current_color
                .component_mul_assign(&material.absorption_color(&current_position.coords));
            current_color += material.emission_color(); // / object.area();
            current_color
        } else {
            Vector3::zeros()
        }
    }

    fn sample_color(&self, ray: &Ray, scene: &Scene) -> Vector3<f64> {
        let light = scene.random_light();

        let light_ray = light.sample_emissive_ray();
        let light_path = self.sample_light_path(&light_ray, scene, light.material());

        Self::sample_camera_path(ray, scene, &light_path, self.max_bounces)
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
