use na::Vector3;
use nalgebra as na;

use crate::{
    camera, material::SurfaceInteraction, shape::IntersectionInfo, Material, Ray, RenderBuffer,
    Renderer, Scene,
};

pub struct BDPTRenderer {
    max_bounces: u8,
}

impl BDPTRenderer {
    pub fn new(max_bounces: u8) -> Self {
        Self { max_bounces }
    }

    fn sample_path(&self, ray: &Ray, scene: &Scene) -> Vec<(Ray, SurfaceInteraction, Material)> {
        let mut current_path = vec![];

        let mut current_ray = *ray;
        for _bounce in 0..self.max_bounces {
            if let Some((object, intersection)) = scene.intersection(&current_ray) {
                let interaction = object.material.interact(&current_ray, &intersection);

                current_path.push((current_ray, interaction, object.material));
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
        let camera_path = self.sample_path(ray, scene);

        let light = scene.random_light();

        let light_ray = light.sample_emissive_ray();
        let light_path = self.sample_path(&light_ray, scene);

        let mut total_light = Vector3::zeros();
        let mut valid_paths = 0;
        for i in 0..camera_path.len() {
            let (ray_a, interaction_a, material_a) = camera_path[i];
            for j in 0..light_path.len() {
                let (ray_b, interaction_b, material_b) = light_path[j];
                let difference = (interaction_a.intersection.position
                    - interaction_b.intersection.position)
                    .normalize();
                if material_a.can_connect(
                    ray_a.direction,
                    difference,
                    interaction_a.intersection.normal,
                ) && material_b.can_connect(
                    ray_b.direction,
                    -difference,
                    interaction_b.intersection.normal,
                ) {
                    if scene.is_visible(
                        &interaction_a.intersection.position,
                        &interaction_b.intersection.position,
                    ) {
                        let mut current_light = light.material.color;
                        for vertex in &camera_path[..=i] {
                            current_light.component_mul_assign(&vertex.1.color_filter);
                        }
                        for vertex in &light_path[j..] {
                            current_light.component_mul_assign(&vertex.1.color_filter);
                        }
                        total_light += current_light;
                    }
                    valid_paths += 1;
                }
            }
        }
        if valid_paths > 0 {
            total_light /= valid_paths as f64;
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
