mod camera;
mod color;
mod cube;
mod framebuffer;
mod light;
mod material;
mod ray_intersect;
mod sphere;
mod texture; // Importa tu nuevo módulo

use crate::color::Color;
use crate::framebuffer::Framebuffer;
use crate::ray_intersect::{Intersect, RayIntersect};
use crate::sphere::Sphere;
use crate::texture::Texture;
use camera::Camera;
use cube::Cube;
use light::Light;
use material::Material;
use minifb::{Window, WindowOptions};
use nalgebra_glm::Vec3;
use once_cell::sync::Lazy;
use rayon::prelude::*;
use std::sync::Arc; // Importa tu nuevo módulo

static WALL1: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/OIP.jpeg")));

fn reflect(incident: &Vec3, normal: &Vec3) -> Vec3 {
    incident - 2.0 * incident.dot(normal) * normal
}

fn cast_ray(
    ray_origin: &Vec3,
    ray_direction: &Vec3,
    objects: &[Sphere],
    light: &Light,
    depth: u32,
) -> Color {
    // Limitar la recursión a una cierta profundidad
    if depth > 3 {
        return Color::new(0, 90, 150); // Color de fondo o "skybox"
    }

    let mut closest_intersect = Intersect::empty();
    let mut zbuffer = f32::INFINITY;

    // Buscar la intersección más cercana
    for object in objects {
        let tmp = object.ray_intersect(ray_origin, ray_direction);
        if tmp.is_intersecting && tmp.distance < zbuffer {
            zbuffer = tmp.distance;
            closest_intersect = tmp;
        }
    }

    if !closest_intersect.is_intersecting {
        return Color::new(0, 90, 150); // Retornar el color de fondo si no hay intersección
    }

    // Obtener el color difuso del material, considerando si tiene una textura o es un color plano
    let diffuse_color = closest_intersect
        .material
        .get_diffuse_color(closest_intersect.u, closest_intersect.v);

    // Calcular la intensidad de la sombra
    let shadow_intensity = cast_shadow(&closest_intersect, light, objects);
    let light_intensity = light.intensity * (1.0 - shadow_intensity);

    // Calcular la dirección de la luz desde el punto de intersección
    let light_dir = (light.position - closest_intersect.point).normalize();

    // Calcular la intensidad difusa utilizando el producto punto
    let diffuse_intensity = closest_intersect.normal.dot(&light_dir).max(0.0).min(1.0);
    let diffuse = diffuse_color
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

    // Calcular la componente de reflexión
    let mut reflect_color = Color::new(0, 0, 0); // Por defecto es negro
    let reflectivity = closest_intersect.material.albedo[2];
    if reflectivity > 0.0 {
        let bias = 0.001; // Ajusta el desplazamiento
        let reflect_dir = reflect(&ray_direction, &closest_intersect.normal).normalize();
        let reflect_origin = closest_intersect.point + closest_intersect.normal * bias;
        reflect_color = cast_ray(&reflect_origin, &reflect_dir, objects, light, depth + 1);
    }

    // Calcular la componente de refracción
    let mut refract_color = Color::new(0, 0, 0); // Por defecto es negro
    let transparency = closest_intersect.material.albedo[3];
    if transparency > 0.0 {
        let bias = 0.001; // Ajusta el desplazamiento
        let refract_dir = refract(
            &ray_direction,
            &closest_intersect.normal,
            closest_intersect.material.refractive_index,
        );
        let refract_origin = closest_intersect.point - closest_intersect.normal * bias;
        refract_color = cast_ray(&refract_origin, &refract_dir, objects, light, depth + 1);
    }

    // Combinación final del color: difuso + especular + reflejo + refracción
    (diffuse + specular) * (1.0 - reflectivity - transparency)
        + (reflect_color * reflectivity)
        + (refract_color * transparency)
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

                let pixel_color = cast_ray(&camera.eye, &ray_direction, objects, light, 0);
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

fn refract(incident: &Vec3, normal: &Vec3, eta_t: f32) -> Vec3 {
    let cosi = -incident.dot(normal).max(-1.0).min(1.0);

    let (n_cosi, eta, n_normal);

    if cosi < 0.0 {
        // El rayo está entrando en el objeto
        n_cosi = -cosi;
        eta = 1.0 / eta_t;
        n_normal = -normal;
    } else {
        // El rayo está saliendo del objeto
        n_cosi = cosi;
        eta = eta_t;
        n_normal = *normal;
    }

    let k = 1.0 - eta * eta * (1.0 - n_cosi * n_cosi);

    if k < 0.0 {
        // Reflexión interna total
        reflect(incident, &n_normal)
    } else {
        eta * incident + (eta * n_cosi - k.sqrt()) * n_normal
    }
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

    let material_con_textura = Material::new(
        Color::new(255, 255, 255),
        500.0,
        [0.8, 0.2, 0.0, 0.0],
        1.5,
        Some(WALL1.clone()),
    );

    let rubber = Material::new(
        Color::new(150, 40, 40), // Un rojo más vivo para que el caucho sea más visible
        10.0,                    // Menor especularidad, ya que el caucho no es tan brillante
        [0.8, 0.2, 0.1, 0.0],    // Más difuso, casi sin reflexión y sin transparencia
        1.1,                     // Índice de refracción del caucho
        None,
    );

    let ivory = Material::new(
        Color::new(220, 220, 200), // Un tono más claro y cálido para el marfil
        30.0,                      // Moderada especularidad
        [0.5, 0.4, 0.3, 0.0], // Mezcla equilibrada de componentes difusos y especulares, sin transparencia
        1.3,                  // Índice de refracción del marfil
        None,
    );

    let mirror = Material::new(
        Color::new(255, 255, 255), // Color base blanco
        1000.0,                    // Alta especularidad para un reflejo casi perfecto
        [0.0, 1.0, 0.9, 0.0],      // Casi todo es reflexión especular
        1.0,                       // Un índice de refracción estándar, ya que es un espejo
        None,
    );

    let glass = Material::new(
        Color::new(200, 200, 255),
        125.0,
        [0.0, 0.5, 0.1, 0.8], // Difusa, especular, reflejo, transparencia
        1.5,                  // Índice de refracción típico del vidrio
        None,
    );

    let objects = vec![
        // Esferas de ejemplo
        Sphere {
            center: Vec3::new(0.0, 0.0, 0.0),
            radius: 0.8,
            material: material_con_textura,
        },
        Sphere {
            center: Vec3::new(-2.2, 1.8, -3.0),
            radius: 0.8,
            material: rubber,
        },
        Sphere {
            center: Vec3::new(2.5, 0.5, -4.0), // Posición de la esfera espejo
            radius: 0.8,                       // Radio de la esfera espejo
            material: mirror,                  // Material de espejo
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
