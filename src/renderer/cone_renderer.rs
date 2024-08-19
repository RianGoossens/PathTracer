use crate::{Material, Ray, RenderBuffer, Renderer, Scene};

use na::Vector3;
use nalgebra::{self as na, Point3, Vector4};

struct CameraPathVertex<'a> {
    pub position: Point3<f64>,
    pub normal: Vector3<f64>,
    pub path_incoming: Vector3<f64>,
    pub path_direction: Vector3<f64>,
    pub material: &'a Material,
    pub accumulated_absorption: Vector3<f64>,
    pub accumulated_emission: Vector3<f64>,
    pub accumulated_distance: f64,
    pub accumulated_backward_likelihood: f64,
    pub accumulated_forward_likelihood: f64,
}

pub struct ConeRenderer {
    pub max_bounces: u8,
    pub camera_material: Material,
}

impl ConeRenderer {
    pub fn new(max_bounces: u8) -> Self {
        Self {
            max_bounces,
            camera_material: Material::new(Vector3::new(1., 1., 1.), 0., false),
        }
    }

    fn sample_camera_path<'a>(
        &'a self,
        ray: &'a Ray,
        scene: &'a Scene,
    ) -> Vec<CameraPathVertex<'a>> {
        let mut accumulated_absorption = Vector3::new(1., 1., 1.);
        let mut accumulated_emission = Vector3::zeros();
        let mut accumulated_distance = 0.;
        let mut accumulated_backward_likelihood = 1.;
        let mut accumulated_forward_likelihood = 1.;
        let mut path = vec![];

        let mut current_ray = *ray;

        for _bounce in 0..self.max_bounces {
            if let Some((object, intersection)) = scene.intersection(&current_ray) {
                let material = object.material();
                accumulated_emission = material
                    .emission_color()
                    .component_mul(&accumulated_absorption)
                    + accumulated_emission;
                accumulated_absorption.component_mul_assign(
                    &material.absorption_color(&intersection.position.coords),
                );
                accumulated_distance += na::distance(&current_ray.origin, &intersection.position);

                let current_vertex = CameraPathVertex {
                    position: intersection.position,
                    normal: intersection.normal,
                    path_incoming: Vector3::zeros(),
                    path_direction: -current_ray.direction,
                    material,
                    accumulated_absorption,
                    accumulated_emission,
                    accumulated_distance,
                    accumulated_backward_likelihood,
                    accumulated_forward_likelihood,
                };

                path.push(current_vertex);

                if _bounce == self.max_bounces - 1 {
                    break;
                }
                let interaction = material.interact(&current_ray, &intersection);

                if let Some(outgoing) = interaction.outgoing {
                    let forward_likelihood = material.likelihood(
                        &-outgoing.direction,
                        &-current_ray.direction,
                        &intersection.normal,
                    );
                    let backward_likelihood = material.likelihood(
                        &current_ray.direction,
                        &outgoing.direction,
                        &intersection.normal,
                    );
                    accumulated_backward_likelihood *= backward_likelihood;
                    accumulated_forward_likelihood *= forward_likelihood;

                    current_ray = outgoing;
                } else {
                    break;
                }
            }
        }

        path
    }

    fn sample_light_path<'a>(&'a self, scene: &'a Scene) -> Vec<CameraPathVertex<'a>> {
        let light = scene.random_light();

        let mut current_ray = light.sample_emissive_ray();

        let mut accumulated_absorption = Vector3::new(1., 1., 1.);
        let mut accumulated_emission = light.material().emission_color();
        let mut accumulated_distance = 0.;
        let mut accumulated_backward_likelihood = 1.;
        let mut accumulated_forward_likelihood = 1.;

        let first_vertex = CameraPathVertex {
            position: current_ray.origin,
            normal: current_ray.direction,
            path_incoming: -current_ray.direction,
            path_direction: current_ray.direction,
            material: light.material(),
            accumulated_absorption,
            accumulated_emission,
            accumulated_distance,
            accumulated_backward_likelihood,
            accumulated_forward_likelihood,
        };

        let mut path = vec![first_vertex];

        for _bounce in 0..self.max_bounces {
            if let Some((object, intersection)) = scene.intersection(&current_ray) {
                let material = object.material();
                let interaction = material.interact(&current_ray, &intersection);

                if let Some(outgoing) = interaction.outgoing {
                    let current_absorption =
                        material.absorption_color(&intersection.position.coords);
                    accumulated_emission = current_absorption.component_mul(&accumulated_emission)
                        + material.emission_color();
                    accumulated_absorption.component_mul_assign(&current_absorption);
                    accumulated_distance +=
                        na::distance(&current_ray.origin, &intersection.position);

                    let current_vertex = CameraPathVertex {
                        position: intersection.position,
                        normal: intersection.normal,
                        path_incoming: current_ray.direction,
                        path_direction: outgoing.direction,
                        material,
                        accumulated_absorption,
                        accumulated_emission,
                        accumulated_distance,
                        accumulated_backward_likelihood,
                        accumulated_forward_likelihood,
                    };

                    path.push(current_vertex);

                    let forward_likelihood = material.likelihood(
                        &current_ray.direction,
                        &outgoing.direction,
                        &intersection.normal,
                    );
                    let backward_likelihood = material.likelihood(
                        &-outgoing.direction,
                        &-current_ray.direction,
                        &intersection.normal,
                    );

                    accumulated_backward_likelihood *= backward_likelihood;
                    accumulated_forward_likelihood *= forward_likelihood;

                    current_ray = outgoing;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        path
    }

    fn sample_color(&self, ray: &Ray, scene: &Scene) -> (Vector3<f64>, f64) {
        const FORWARD_WEIGHT: f64 = 0.5;
        const BACKWARD_WEIGHT: f64 = 1. - FORWARD_WEIGHT;

        let camera_path = self.sample_camera_path(ray, scene);
        let light_path = self.sample_light_path(scene);

        let mut total_importance = 0.;
        let mut total_light = Vector3::zeros();
        let mut total_samples = 0;

        if let Some(last_camera_vertex) = camera_path.last() {
            if last_camera_vertex.material.is_emissive() {
                let backwards_importance = last_camera_vertex.accumulated_backward_likelihood;
                let forwards_importance = last_camera_vertex.accumulated_forward_likelihood;

                if backwards_importance > 0. {
                    total_samples += 1;

                    total_importance = 100.
                        * (last_camera_vertex.accumulated_emission.mean() / backwards_importance);
                    //total_importance =
                    //    backwards_importance / last_camera_vertex.accumulated_distance.powi(2);
                    //(BACKWARD_WEIGHT * backwards_importance + FORWARD_WEIGHT * forwards_importance);
                    //total_importance = 100.;
                    total_light += last_camera_vertex.accumulated_emission * total_importance;
                    // last_camera_vertex.accumulated_distance.powi(2)
                }
            }
        }

        for vertex_camera in &camera_path {
            if vertex_camera.material.is_emissive() {
                continue;
            }
            for vertex_light in &light_path {
                if scene.is_visible(&vertex_light.position, &vertex_camera.position) {
                    let current_light = vertex_light
                        .accumulated_emission
                        .component_mul(&vertex_camera.accumulated_absorption)
                        + vertex_camera.accumulated_emission;

                    let edge = vertex_camera.position - vertex_light.position;
                    let edge_direction = edge.normalize();
                    let edge_distance = edge.magnitude();

                    let edge_importance_forwards = vertex_camera.material.likelihood(
                        &edge_direction,
                        &vertex_camera.path_direction,
                        &vertex_camera.normal,
                    ) * vertex_light.material.likelihood(
                        &vertex_light.path_incoming,
                        &edge_direction,
                        &vertex_light.normal,
                    );
                    let edge_importance_backwards = vertex_camera.material.likelihood(
                        &-vertex_camera.path_direction,
                        &-edge_direction,
                        &vertex_camera.normal,
                    ) * vertex_light.material.likelihood(
                        &-edge_direction,
                        &-vertex_light.path_incoming,
                        &vertex_light.normal,
                    );

                    let distance = 1.
                        + vertex_camera.accumulated_distance
                        //+ vertex_light.accumulated_distance
                        + edge_distance;
                    let likelihood_backwards = vertex_camera.accumulated_backward_likelihood
                        * vertex_light.accumulated_backward_likelihood
                        * edge_importance_backwards;
                    let likelihood_forwards = vertex_camera.accumulated_forward_likelihood
                        * vertex_light.accumulated_forward_likelihood
                        * edge_importance_forwards;
                    let likelihood = likelihood_backwards * BACKWARD_WEIGHT
                        + likelihood_forwards * FORWARD_WEIGHT;

                    let likelihood = vertex_camera.accumulated_backward_likelihood
                        * vertex_light.accumulated_forward_likelihood
                        * edge_importance_forwards;

                    if likelihood > 0. {
                        let importance = current_light.mean() / likelihood; // / distance.powi(2);

                        total_light += current_light * importance / (1. + edge_distance.powi(2));
                        total_importance += importance;
                        total_samples += 1;
                    }
                }
            }
        }

        //println!("{total_importance}");
        if total_samples > 0 && total_importance > 0. {
            //total_light *= total_importance;
            total_light /= total_samples as f64 * total_importance;
        } else {
            total_light *= 0.;
        }
        //total_importance = 1.;
        (total_light, 1.)
        //(total_light, 1.)
    }
}

impl Renderer for ConeRenderer {
    fn render(&self, scene: &Scene) -> RenderBuffer {
        let width = scene.camera.width;
        let height = scene.camera.height;

        let mut render_buffer = RenderBuffer::new(width, height);

        for x in 0..width {
            for y in 0..height {
                let ray = scene.camera.get_ray(x, y);

                let (color, weight) = self.sample_color(&ray, scene);
                render_buffer[(x, y)] = Vector4::new(color.x, color.y, color.z, weight);
            }
        }

        render_buffer
    }
}
