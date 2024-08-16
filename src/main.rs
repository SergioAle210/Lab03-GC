mod color;
mod framebuffer;
mod sphere;
mod texture;

use crate::color::Color;
use crate::framebuffer::Framebuffer;
use crate::sphere::Sphere;
use crate::texture::Texture;
use nalgebra_glm::Vec3;

// Definir el trait para los objetos que soportarán intersección de rayos
pub trait RayIntersect {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> bool;
}

fn cast_ray(ray_origin: &Vec3, ray_direction: &Vec3, objects: &[Box<dyn RayIntersect>]) -> u32 {
    for object in objects {
        if object.ray_intersect(ray_origin, ray_direction) {
            return 0xFFFFFF; // Color blanco si hay intersección
        }
    }
    0x000000 // Color negro si no hay intersección
}

fn main() {
    let sphere = Sphere::new(Vec3::new(0.0, 0.0, -5.0), 1.0, Color::new(255, 0, 0));
    let objects: Vec<Box<dyn RayIntersect>> = vec![Box::new(sphere)];

    let ray_origin = Vec3::new(0.0, 0.0, 0.0);
    let ray_direction = Vec3::new(0.0, 0.0, -1.0);

    let color = cast_ray(&ray_origin, &ray_direction, &objects);
    println!("Color resultante: {:06X}", color); // Debería imprimir "FFFFFF" si hay intersección
}
