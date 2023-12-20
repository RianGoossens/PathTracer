use na::{Point3, Vector3};
use nalgebra as na;

use crate::{Material, Ray, RenderBuffer, Renderer, Scene};

/*
A vertex has at least:
- position
- incoming ray direction (reversed for camera path)
- material
- normal
- absorption color
- illumination color
    */
#[derive(Debug, Clone, Copy)]
struct PathVertex {
    pub position: Point3<f64>,
    pub normal: Vector3<f64>,
    pub incoming: Vector3<f64>,
    pub material: Material,
    pub accumulated_absorption: Vector3<f64>,
    pub accumulated_emission: Vector3<f64>,
}

enum PathDirection {
    CameraPath,
    LightPath,
}

pub struct BDPTRenderer {
    max_bounces: u8,
}

impl BDPTRenderer {
    pub fn new(max_bounces: u8) -> Self {
        Self { max_bounces }
    }

    fn sample_path(
        &self,
        ray: &Ray,
        scene: &Scene,
        initial_light: &Vector3<f64>,
        path_direction: PathDirection,
    ) -> Vec<PathVertex> {
        let first_vertex = PathVertex {
            position: ray.origin,
            normal: ray.direction,
            incoming: -ray.direction,
            material: match path_direction {
                PathDirection::CameraPath => Material {
                    color: Vector3::new(1., 1., 1.),
                    roughness: 0.,
                    emissive: false,
                },
                PathDirection::LightPath => Material {
                    color: *initial_light,
                    roughness: 1.,
                    emissive: true,
                },
            },
            accumulated_absorption: Vector3::new(1., 1., 1.),
            accumulated_emission: *initial_light,
        };

        let mut current_path = vec![first_vertex];

        let mut current_ray = *ray;
        let mut accumulated_emission = *initial_light;
        let mut accumulated_absorption = Vector3::new(1., 1., 1.);
        for _bounce in 0..self.max_bounces {
            if let Some((object, intersection)) = scene.intersection(&current_ray) {
                let interaction = object.material.interact(&current_ray, &intersection);
                let current_absorption = interaction.color_filter;
                let current_emission = interaction.emission;

                accumulated_emission = match path_direction {
                    PathDirection::CameraPath => {
                        current_emission.component_mul(&accumulated_absorption)
                            + accumulated_emission
                    }
                    PathDirection::LightPath => {
                        accumulated_emission.component_mul(&current_absorption) + current_emission
                    }
                };
                accumulated_absorption.component_mul_assign(&current_absorption);

                let vertex = PathVertex {
                    position: intersection.position,
                    normal: intersection.normal.normalize(),
                    incoming: current_ray.direction,
                    material: object.material,
                    accumulated_absorption,
                    accumulated_emission,
                };

                current_path.push(vertex);
                if let Some(new_ray) = interaction.outgoing {
                    current_ray = new_ray;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        current_path
    }

    fn sample_color(&self, ray: &Ray, scene: &Scene) -> Vector3<f64> {
        let camera_path =
            self.sample_path(ray, scene, &Vector3::zeros(), PathDirection::CameraPath);

        let light = scene.random_light();

        let light_ray = light.sample_emissive_ray();
        let light_path = self.sample_path(
            &light_ray,
            scene,
            &light.material.color,
            PathDirection::LightPath,
        );
        let mut total_importance = 1.;
        let mut total_light =
            total_importance * camera_path[camera_path.len() - 1].accumulated_emission;

        for vertex_camera in &camera_path[1..] {
            for vertex_light in &light_path {
                let difference = (vertex_light.position - vertex_camera.position).normalize();
                if vertex_camera.material.can_connect(
                    vertex_camera.incoming,
                    -difference,
                    vertex_camera.normal,
                ) && vertex_light.material.can_connect(
                    vertex_light.incoming,
                    difference,
                    vertex_light.normal,
                ) && scene.is_visible(&vertex_camera.position, &vertex_light.position)
                {
                    let current_light = vertex_light
                        .accumulated_emission
                        .component_mul(&vertex_camera.accumulated_absorption)
                        + vertex_camera.accumulated_emission;

                    let importance = 1.;
                    total_light += current_light * importance;
                    total_importance += importance;
                }
            }
        }
        if total_importance > 0. {
            total_light /= total_importance;
        }
        total_light
    }
}

impl Renderer for BDPTRenderer {
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
