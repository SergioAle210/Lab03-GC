mod camera;
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
use camera::Camera;
use minifb::{Window, WindowOptions};
use nalgebra::center;
use nalgebra_glm::Vec3;
use rayon::prelude::*;

fn cast_ray(ray_origin: &Vec3, ray_direction: &Vec3, objects: &[Sphere]) -> Color {
    let mut closest_intersect = Intersect::empty();
    let mut zbuffer = f32::INFINITY;

    for object in objects {
        let tmp = object.ray_intersect(ray_origin, ray_direction);
        if tmp.is_intersecting && tmp.distance < zbuffer {
            zbuffer = tmp.distance;
            closest_intersect = tmp;
            // Salida temprana: Si sabemos que no hay objetos más cercanos, salimos.
            if zbuffer < 0.001 {
                break;
            }
        }
    }

    if !closest_intersect.is_intersecting {
        return Color::new(0, 0, 0);
    }

    closest_intersect.material.diffuse
}

fn render(framebuffer: &mut Framebuffer, objects: &[Sphere], camera: &Camera) {
    let width = framebuffer.width;
    let height = framebuffer.height;
    let aspect_ratio = width as f32 / height as f32;

    framebuffer
        .buffer
        .par_chunks_mut(width)
        .enumerate()
        .for_each(|(y, row)| {
            let screen_y = -(2.0 * y as f32) / height as f32 + 1.0;
            for x in 0..width {
                let screen_x = (2.0 * x as f32) / width as f32 - 1.0;
                let screen_x = screen_x * aspect_ratio;

                let world_direction = Vec3::new(screen_x, screen_y, -1.0);
                let ray_direction = camera.basis_change(&world_direction);

                let pixel_color = cast_ray(&camera.eye, &ray_direction, objects);
                row[x] = pixel_color;
            }
        });
}

fn main() {
    let mut camera = Camera {
        eye: Vec3::new(-4.0, 5.0, 0.0),   // Posición de la cámara
        center: Vec3::new(0.0, 0.0, 0.0), // Punto hacia el que está viendo la cámara
        up: Vec3::new(0.0, 1.0, 0.0),     // Vector "arriba" de la cámara
    };

    let ivory = Material {
        diffuse: Color::new(128, 128, 128),
    };
    let rojo = Material {
        diffuse: Color::new(210, 23, 23),
    };

    let objects = vec![
        // Esferas de ejemplo
        Sphere {
            center: Vec3::new(0.0, 0.0, 0.0),
            radius: 0.8,
            material: ivory,
        },
        Sphere {
            center: Vec3::new(-2.2, 1.8, -3.0),
            radius: 0.8,
            material: rojo,
        },
    ];

    let width = 650; // Reduce el tamaño a la mitad
    let height = 450;
    let mut framebuffer = Framebuffer::new(width, height);

    let mut window = Window::new(
        "Raytracer",
        (width as f32 / 1.3) as usize,
        (height as f32 / 1.3) as usize,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut needs_render = true;

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        if window.is_key_down(minifb::Key::Left) {
            camera.orbit(0.05, 0.0);
            needs_render = true;
        }
        if window.is_key_down(minifb::Key::Right) {
            camera.orbit(-0.05, 0.0);
            needs_render = true;
        }
        if window.is_key_down(minifb::Key::Up) {
            camera.orbit(0.0, 0.05);
            needs_render = true;
        }
        if window.is_key_down(minifb::Key::Down) {
            camera.orbit(0.0, -0.05);
            needs_render = true;
        }

        // Solo renderiza si hubo cambios
        if needs_render {
            render(&mut framebuffer, &objects, &camera);
            needs_render = false;
        }

        window
            .update_with_buffer(&framebuffer.to_u32_buffer(), width, height)
            .unwrap();
    }
}
