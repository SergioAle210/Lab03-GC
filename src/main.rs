mod camera;
mod color;
mod framebuffer;
mod light;
mod material;
mod ray_intersect;
mod sphere;
mod texture;

use crate::color::Color;
use crate::framebuffer::Framebuffer;
use crate::ray_intersect::{Intersect, RayIntersect};
use crate::sphere::Sphere;
use crate::texture::Texture;
use camera::Camera;
use light::Light;
use material::Material;
use minifb::{Window, WindowOptions};
use nalgebra::center;
use nalgebra_glm::Vec3;
use rayon::prelude::*;

fn reflect(incident: &Vec3, normal: &Vec3) -> Vec3 {
    incident - 2.0 * incident.dot(normal) * normal
}

fn cast_ray(ray_origin: &Vec3, ray_direction: &Vec3, objects: &[Sphere], light: &Light) -> Color {
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
        // color de fondo azul
        return Color::new(0, 90, 150);
    }

    // Calcular la intensidad de la sombra
    let shadow_intensity = cast_shadow(&closest_intersect, light, objects);
    let light_intensity = light.intensity * (1.0 - shadow_intensity);

    // Calcular la dirección de la luz desde el punto de intersección
    let light_dir = (light.position - closest_intersect.point).normalize();

    // Calcular la intensidad difusa utilizando el producto punto
    let diffuse_intensity = closest_intersect.normal.dot(&light_dir).max(0.0).min(1.0);
    let diffuse = closest_intersect.material.diffuse
        * closest_intersect.material.albedo[0] // Albedo difuso
        * diffuse_intensity
        * light_intensity;

    // Calcular la dirección de la vista desde el punto de intersección hacia el origen del rayo
    let view_dir = (ray_origin - closest_intersect.point).normalize();

    // Calcular la dirección de reflexión de la luz
    let reflect_dir = reflect(&-light_dir, &closest_intersect.normal);

    // Calcular la intensidad especular utilizando el producto punto elevado a la potencia especular del material
    let specular_intensity = view_dir
        .dot(&reflect_dir)
        .max(0.0)
        .powf(closest_intersect.material.specular);
    let specular = light.color
        * closest_intersect.material.albedo[1] // Albedo especular
        * specular_intensity
        * light_intensity;

    // Retornar la suma del componente difuso y el especular
    diffuse + specular
}

fn render(framebuffer: &mut Framebuffer, objects: &[Sphere], camera: &Camera, light: &Light) {
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

                let pixel_color = cast_ray(&camera.eye, &ray_direction, objects, light);
                row[x] = pixel_color;
            }
        });
}

fn cast_shadow(intersect: &Intersect, light: &Light, objects: &[Sphere]) -> f32 {
    let light_dir = (light.position - intersect.point).normalize();
    // Desplazamos el origen del rayo ligeramente utilizando la normal para evitar el problema de acné
    let shadow_ray_origin = intersect.point + intersect.normal * 0.001;
    let mut shadow_intensity = 0.0;
    let light_distance = (light.position - shadow_ray_origin).magnitude();

    for object in objects {
        let shadow_intersect = object.ray_intersect(&shadow_ray_origin, &light_dir);
        if shadow_intersect.is_intersecting {
            let object_distance = shadow_intersect.distance;

            // Calculamos la intensidad de la sombra basado en la distancia
            shadow_intensity = 1.0 - (object_distance / light_distance).min(1.0);
            break;
        }
    }

    shadow_intensity
}

fn main() {
    let mut camera = Camera {
        eye: Vec3::new(0.0, 0.0, 5.0),    // Posición de la cámara
        center: Vec3::new(0.0, 0.0, 0.0), // Punto hacia el que está viendo la cámara
        up: Vec3::new(0.0, 8.0, 0.0),     // Vector "arriba" de la cámara
    };

    let light = Light::new(
        Vec3::new(5.0, 10.0, 5.0),
        Color::new(255, 255, 255), // Color blanco para la luz
        2.0,                       // Intensidad de la luz
    );

    let rubber = Material::new(
        Color::new(80, 0, 0),
        1.0,
        [0.9, 0.1], // 90% difusa, 10% especular
    );

    let ivory = Material::new(
        Color::new(100, 100, 80),
        50.0,
        [0.6, 0.3], // 60% difusa, 30% especular
    );

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
            material: rubber,
        },
    ];

    let width = 1300; // Reduce el tamaño a la mitad
    let height = 900;
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
            render(&mut framebuffer, &objects, &camera, &light);
            needs_render = false;
        }

        window
            .update_with_buffer(&framebuffer.to_u32_buffer(), width, height)
            .unwrap();
    }
}
