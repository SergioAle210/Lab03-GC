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
use minifb::{Window, WindowOptions};
use nalgebra::center;
use nalgebra_glm::Vec3;

fn cast_ray(ray_origin: &Vec3, ray_direction: &Vec3, objects: &[Sphere]) -> Color {
    let mut closest_intersect = Intersect::empty();
    let mut zbuffer = f32::INFINITY;

    for object in objects {
        let tmp = object.ray_intersect(ray_origin, ray_direction);
        if tmp.is_intersecting && tmp.distance < zbuffer {
            zbuffer = tmp.distance;
            closest_intersect = tmp;
        }
    }

    if !closest_intersect.is_intersecting {
        return Color::new(4, 12, 36);
    }

    closest_intersect.material.diffuse
}

fn render(framebuffer: &mut Framebuffer, objects: &[Sphere]) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;
            let screen_x = screen_x * aspect_ratio;

            let ray_direction = Vec3::new(screen_x, screen_y, -1.0).normalize();
            let pixel_color = cast_ray(&Vec3::new(0.0, 0.0, 0.0), &ray_direction, objects);
            framebuffer.point_with_color(x, y, pixel_color);
        }
    }
}

fn main() {
    let ivory = Material {
        diffuse: Color::new(128, 128, 128),
    };
    let blanco = Material {
        diffuse: Color::new(255, 255, 255),
    };
    let negro = Material {
        diffuse: Color::new(0, 0, 0),
    };

    let objects = vec![
        //Cabeza
        Sphere {
            center: Vec3::new(-0.5, 0.0, -2.0),
            radius: 1.0,
            material: ivory,
        },
        Sphere {
            center: Vec3::new(0.5, 0.0, -2.0),
            radius: 1.0,
            material: ivory,
        },
        // Pupilas
        Sphere {
            center: Vec3::new(0.3, -0.2, -1.0),
            radius: 0.2,
            material: blanco,
        },
        Sphere {
            center: Vec3::new(-0.3, -0.2, -1.0),
            radius: 0.2,
            material: blanco,
        },
        // Ojos
        Sphere {
            center: Vec3::new(-0.15, -0.15, -0.5),
            radius: 0.05,
            material: negro,
        },
        Sphere {
            center: Vec3::new(-0.15, -0.15, -0.5),
            radius: 0.05,
            material: negro,
        },
    ];

    let width = 1300;
    let height = 900;
    let mut framebuffer = Framebuffer::new(width, height);

    // Usamos la función render
    render(&mut framebuffer, &objects);

    let mut window = Window::new(
        "Raytracer",
        (width as f32 / 1.3) as usize,
        (height as f32 / 1.3) as usize,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    while window.is_open() && !window.is_key_down(minifb::Key::Escape) {
        window
            .update_with_buffer(&framebuffer.to_u32_buffer(), width, height)
            .unwrap();
    }
}
