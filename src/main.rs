mod color;
mod framebuffer;
mod ray_intersect;
mod sphere;
mod texture;

use crate::color::Color;
use crate::framebuffer::Framebuffer;
use crate::ray_intersect::{Intersect, Material, RayIntersect};
use crate::sphere::Sphere;
use crate::texture::Texture;
use nalgebra_glm::Vec3;

fn cast_ray(ray_origin: &Vec3, ray_direction: &Vec3, objects: &[Box<dyn RayIntersect>]) -> u32 {
    let mut closest_intersect = Intersect::empty();
    for object in objects {
        let intersect = object.ray_intersect(ray_origin, ray_direction);
        if intersect.is_intersecting
            && (intersect.distance < closest_intersect.distance
                || !closest_intersect.is_intersecting)
        {
            closest_intersect = intersect;
        }
    }

    if closest_intersect.is_intersecting {
        return closest_intersect.material.diffuse.to_hex();
    }
    0x000000 // Color negro si no hay intersección
}

fn main() {
    let material = Material {
        diffuse: Color::new(255, 0, 0),
    };
    let sphere = Sphere::new(Vec3::new(0.0, 0.0, -5.0), 1.0, material);
    let objects: Vec<Box<dyn RayIntersect>> = vec![Box::new(sphere)];

    let ray_origin = Vec3::new(0.0, 0.0, 0.0);
    let ray_direction = Vec3::new(0.0, 0.0, -1.0);

    let color = cast_ray(&ray_origin, &ray_direction, &objects);
    println!("Color resultante: {:06X}", color); // Debería imprimir "FF0000" si hay intersección
}
