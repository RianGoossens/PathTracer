use crate::{
    octree::{BoundingBox, HasCoordinate, Octree},
    scene, Ray, RenderBuffer, Renderer, Scene,
};

use na::{Point3, Vector4};
use nalgebra::{self as na, Vector3};
use rand::{thread_rng, Rng};

pub struct PhotonDirectionRenderer {
    pub max_bounces: u8,
    pub bounding_box: BoundingBox,
}

#[derive(Debug, Copy, Clone)]
pub struct PhotonDirection {
    pub position: Point3<f64>,
    pub origin: Point3<f64>,
}
impl HasCoordinate for PhotonDirection {
    fn coordinate(&self) -> Point3<f64> {
        self.position
    }
}

impl PhotonDirectionRenderer {
    pub fn new(max_bounces: u8) -> Self {
        Self {
            max_bounces,
            bounding_box: BoundingBox::new(-10., 10., -10., 10., -10., 10.),
        }
    }
    fn sample_light_direction(&self, scene: &Scene) -> Option<PhotonDirection> {
        let max_bounces = self.max_bounces; //thread_rng().gen_range(1..self.max_bounces);
        let light = scene.random_light();

        let mut current_ray = light.sample_emissive_ray();
        let mut is_diffuse = true;

        let mut result = None;

        for _ in 0..max_bounces {
            if let Some((object, intersection)) = scene.intersection(&current_ray) {
                if is_diffuse {
                    result = Some(PhotonDirection {
                        origin: current_ray.origin,
                        position: intersection.position,
                    })
                }
                is_diffuse = object.material().roughness() > 0.;

                let interaction = object.material().interact(&current_ray, &intersection);
                if let Some(outgoing) = interaction.outgoing {
                    current_ray = outgoing;
                }
            } else {
                break;
            }
        }
        result
    }

    fn sample_color(
        &self,
        ray: &Ray,
        scene: &Scene,
        light_directions: &Octree<PhotonDirection>,
    ) -> Vector3<f64> {
        let mut current_color_filter = Vector3::new(1., 1., 1.);
        let mut current_emission = Vector3::zeros();
        let mut current_ray = *ray;

        for _bounce in 0..self.max_bounces {
            if let Some((object, intersection)) = scene.intersection(&current_ray) {
                let interaction = object.material().interact(&current_ray, &intersection);

                current_emission += interaction.emission.component_mul(&current_color_filter);
                current_color_filter.component_mul_assign(&interaction.filter);

                if object.material().roughness() > 0. {
                    if let Some(light_direction) = light_directions.find(&intersection.position) {
                        let direction =
                            (light_direction.origin - intersection.position).normalize();

                        if object.material().likelihood(
                            &current_ray.direction,
                            &direction,
                            &intersection.normal,
                        ) > 0.
                        {
                            current_ray =
                                Ray::new(intersection.position + direction * 0.001, direction);
                            continue;
                        }
                    }
                }
                if let Some(outgoing) = interaction.outgoing {
                    current_ray = outgoing;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        current_emission
    }

    fn sample_color_test(
        &self,
        ray: &Ray,
        scene: &Scene,
        light_directions: &Octree<PhotonDirection>,
    ) -> Vector3<f64> {
        let mut current_ray = *ray;

        for _bounce in 0..self.max_bounces {
            if let Some((object, intersection)) = scene.intersection(&current_ray) {
                let interaction = object.material().interact(&current_ray, &intersection);

                if object.material().roughness() > 0. {
                    if let Some(light_direction) = light_directions.find(&intersection.position) {
                        let distance =
                            na::distance(&intersection.position, &light_direction.position);

                        if distance <= 0.1 {
                            return Vector3::new(1., 1., 1.) * (0.1 - distance) * 10.;
                        } else {
                            return Vector3::zeros();
                        }
                    }
                }
                if let Some(outgoing) = interaction.outgoing {
                    current_ray = outgoing;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        Vector3::new(1., 0., 0.)
    }
}

impl Renderer for PhotonDirectionRenderer {
    fn render(&self, scene: &Scene) -> RenderBuffer {
        let width = scene.camera.width;
        let height = scene.camera.height;

        const PHOTONS: usize = 1000000;
        let mut octree = Octree::new(self.bounding_box);

        let mut number_of_photons_stored = 0;
        for _ in 0..PHOTONS {
            if let Some(photon) = self.sample_light_direction(scene) {
                if octree.add(photon) {
                    number_of_photons_stored += 1;
                }
            }
        }
        println!("Shot {number_of_photons_stored} succesful photons");

        let mut render_buffer = RenderBuffer::new(width, height);

        for x in 0..width {
            for y in 0..height {
                let ray = scene.camera.get_ray(x, y);

                let color = self.sample_color(&ray, scene, &octree);
                render_buffer[(x, y)] = Vector4::new(color.x, color.y, color.z, 1.);
            }
        }

        render_buffer
    }
}
