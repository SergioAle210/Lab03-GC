mod camera;
mod color;
mod cuboid;
mod framebuffer;
mod light;
mod material;
mod ray_intersect;
mod texture; // Importa tu nuevo módulo

use crate::color::Color;
use crate::framebuffer::Framebuffer;
use crate::ray_intersect::{Intersect, RayIntersect};
use crate::texture::Texture;
use camera::Camera;
use cuboid::Cuboid;
use light::Light;
use material::Material;
use minifb::{Window, WindowOptions};
use nalgebra::ComplexField;
use nalgebra_glm::Vec3;
use once_cell::sync::Lazy;
use rayon::prelude::*;
use std::sync::Arc; // Importa tu nuevo módulo

static WALL1: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/OIP.jpeg")));
static LADRILLOS: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/ladrillos.png")));
static WATER: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/WATER.jpg")));
static LADRILLOS_NEGROS: Lazy<Arc<Texture>> =
    Lazy::new(|| Arc::new(Texture::new("assets/ladrillos_negros.png")));
static SUELO: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/suelo.png")));
static BRICKS: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("assets/Bricks.png")));

fn reflect(incident: &Vec3, normal: &Vec3) -> Vec3 {
    incident - 2.0 * incident.dot(normal) * normal
}

fn cast_ray(
    ray_origin: &Vec3,
    ray_direction: &Vec3,
    objects: &[Box<dyn RayIntersect>],
    lights: &[Light], // Cambiar a un vector de luces
    depth: u32,
    use_normal_map: bool, // Añade un parámetro para controlar si se usa el mapeo de normales
) -> Color {
    if depth > 3 {
        return Color::new(0, 90, 150); // Color de fondo o "skybox"
    }

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
        return Color::new(0, 90, 150);
    }

    let bias = 0.01;
    closest_intersect.point += closest_intersect.normal * bias;

    let normal = closest_intersect.normal;

    let diffuse_color = if use_normal_map {
        let r = ((normal.x + 1.0) * 0.5 * 255.0) as i32;
        let g = ((normal.y + 1.0) * 0.5 * 255.0) as i32;
        let b = ((normal.z + 1.0) * 0.5 * 255.0) as i32;
        Color::new(r, g, b)
    } else {
        closest_intersect
            .material
            .get_diffuse_color(closest_intersect.u, closest_intersect.v)
    };

    let mut final_color = Color::new(0, 0, 0);

    // Iterar sobre todas las fuentes de luz
    for light in lights {
        let shadow_intensity = cast_shadow(&closest_intersect, light, objects);
        let light_intensity = light.intensity * (1.0 - shadow_intensity);

        let light_dir = (light.position - closest_intersect.point).normalize();

        let diffuse_intensity = normal.dot(&light_dir).max(0.0).min(1.0);
        let diffuse = diffuse_color
            * closest_intersect.material.albedo[0]
            * diffuse_intensity
            * light_intensity;

        let view_dir = (ray_origin - closest_intersect.point).normalize();
        let reflect_dir = reflect(&-light_dir, &normal);

        let specular_intensity = view_dir
            .dot(&reflect_dir)
            .max(0.0)
            .powf(closest_intersect.material.specular);
        let specular = light.color
            * closest_intersect.material.albedo[1]
            * specular_intensity
            * light_intensity;

        final_color += diffuse + specular;
    }

    let reflectivity = closest_intersect.material.albedo[2];
    let transparency = closest_intersect.material.albedo[3];

    if reflectivity > 0.0 {
        let reflect_dir = reflect(&ray_direction, &normal).normalize();
        let reflect_origin = closest_intersect.point + normal * bias;
        let reflect_color = cast_ray(
            &reflect_origin,
            &reflect_dir,
            objects,
            lights,
            depth + 1,
            use_normal_map,
        );
        final_color += reflect_color * reflectivity;
    }

    if transparency > 0.0 {
        let refract_dir = refract(
            &ray_direction,
            &normal,
            closest_intersect.material.refractive_index,
        );
        let refract_origin = closest_intersect.point - normal * bias;
        let refract_color = cast_ray(
            &refract_origin,
            &refract_dir,
            objects,
            lights,
            depth + 1,
            use_normal_map,
        );
        final_color += refract_color * transparency;
    }

    final_color
}

fn render(
    framebuffer: &mut Framebuffer,
    objects: &[Box<dyn RayIntersect>],
    camera: &Camera,
    lights: &[Light], // Cambiar a un vector de luces
    use_normal_map: bool,
) {
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

                let pixel_color = cast_ray(
                    &camera.eye,
                    &ray_direction,
                    objects,
                    lights,
                    0,
                    use_normal_map,
                );
                row[x] = pixel_color;
            }
        });
}

fn cast_shadow(intersect: &Intersect, light: &Light, objects: &[Box<dyn RayIntersect>]) -> f32 {
    let light_dir = (light.position - intersect.point).normalize();
    let bias = 0.001; // Use the same bias value as in cast_ray
    let shadow_ray_origin = intersect.point + intersect.normal * bias;
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

    // Definir múltiples luces
    let light1 = Light::new(Vec3::new(5.0, 10.0, 5.0), Color::new(255, 255, 255), 2.0);

    let light2 = Light::new(
        Vec3::new(5.0, 10.0, -5.0),
        Color::new(255, 100, 100), // Luz rojiza
        1.0,
    );

    let mut lights = vec![light1, light2];

    let material_con_textura = Material::new(
        Color::new(255, 255, 255),
        500.0,
        [0.8, 0.2, 0.0, 0.0],
        1.5,
        Some(LADRILLOS.clone()),
    );

    let water = Material::new(
        Color::new(200, 200, 255),
        125.0,
        [0.0, 0.5, 0.7, 0.5], // Difusa, especular, reflejo, transparencia
        1.33,                 // Índice de refracción del agua
        Some(WATER.clone()),
    );

    let ladrillos_neg = Material::new(
        Color::new(255, 255, 255),
        500.0,
        [0.8, 0.2, 0.0, 0.1], // Difusa, especular, reflejo, transparencia
        1.5,
        Some(LADRILLOS_NEGROS.clone()),
    );

    let suelo = Material::new(
        Color::new(128, 128, 128), // Color gris para el suelo
        100.0,                     // Menor reflectividad que ladrillos_neg
        [0.6, 0.3, 0.1, 0.0],      // Coeficientes de reflexión diferentes
        1.0,                       // Índice de refracción diferente
        Some(SUELO.clone()),       // Textura del suelo
    );

    let texture_bricks = Material::new(
        Color::new(255, 255, 255), // Color blanco para los ladrillos
        250.0,                     // Alta reflectividad
        [0.9, 0.3, 0.0, 0.0],      // Coeficientes de reflexión
        1.0,                       // Índice de refracción
        Some(BRICKS.clone()),      // Textura de los ladrillos
    );

    // Agua compuesta por 4 cubos
    let cuboid1 = Cuboid::new(
        Vec3::new(-1.0, 0.0, 0.0), // Centro del cubo
        1.0,                       // Ancho del cubo
        1.0,                       // Altura del cubo
        1.0,                       // Profundidad del cubo
        water.clone(),             // Material de caucho
    );

    let cuboid2 = Cuboid::new(
        Vec3::new(-2.0, 0.0, 0.0), // Centro del cubo
        1.0,                       // Ancho del cubo
        1.0,                       // Altura del cubo
        1.0,                       // Profundidad del cubo
        water.clone(),             // Material de caucho
    );

    let cuboid3 = Cuboid::new(
        Vec3::new(-1.0, 0.0, 1.0), // Centro del cubo
        1.0,                       // Ancho del cubo
        1.0,                       // Altura del cubo
        1.0,                       // Profundidad del cubo
        water.clone(),             // Material de caucho
    );

    let cuboid4 = Cuboid::new(
        Vec3::new(-2.0, 0.0, 1.0), // Centro del cubo
        1.0,                       // Ancho del cubo
        1.0,                       // Altura del cubo
        1.0,                       // Profundidad del cubo
        water.clone(),             // Material de caucho
    );

    // Columnas de ladrillos
    let cuboid5 = Cuboid::new(
        Vec3::new(0.0, 0.0, -1.0), // Centro del cubo
        1.0,                       // Ancho del cubo
        1.0,                       // Altura del cubo
        1.0,                       // Profundidad del cubo
        ladrillos_neg.clone(),     // Material de caucho
    );

    let cuboid6: Cuboid = Cuboid::new(
        Vec3::new(0.0, 1.0, -1.0), // Centro del cubo
        1.0,                       // Ancho del cubo
        1.0,                       // Altura del cubo
        1.0,                       // Profundidad del cubo
        ladrillos_neg.clone(),     // Material de caucho
    );

    let cuboid7: Cuboid = Cuboid::new(
        Vec3::new(0.0, 2.0, -1.0), // Centro del cubo
        0.5,                       // Ancho del cubo
        0.5,                       // Altura del cubo
        0.5,                       // Profundidad del cubo
        ladrillos_neg.clone(),     // Material de caucho
    );

    let cuboid8: Cuboid = Cuboid::new(
        Vec3::new(0.0, 0.0, 2.0), // Centro del cubo
        1.0,                      // Ancho del cubo
        1.0,                      // Altura del cubo
        1.0,                      // Profundidad del cubo
        ladrillos_neg.clone(),    // Material de caucho
    );

    let cuboid9: Cuboid = Cuboid::new(
        Vec3::new(0.0, 1.0, 2.0), // Centro del cubo
        1.0,                      // Ancho del cubo
        1.0,                      // Altura del cubo
        1.0,                      // Profundidad del cubo
        ladrillos_neg.clone(),    // Material de caucho
    );

    let cuboid10: Cuboid = Cuboid::new(
        Vec3::new(0.0, 2.0, 2.0), // Centro del cubo
        0.5,                      // Ancho del cubo
        0.5,                      // Altura del cubo
        0.5,                      // Profundidad del cubo
        ladrillos_neg.clone(),    // Material de caucho
    );

    let cuboid11: Cuboid = Cuboid::new(
        Vec3::new(-3.0, 0.0, 2.0), // Centro del cubo
        1.0,                       // Ancho del cubo
        1.0,                       // Altura del cubo
        1.0,                       // Profundidad del cubo
        ladrillos_neg.clone(),     // Material de caucho
    );

    let cuboid12: Cuboid = Cuboid::new(
        Vec3::new(-3.0, 1.0, 2.0), // Centro del cubo
        1.0,                       // Ancho del cubo
        1.0,                       // Altura del cubo
        1.0,                       // Profundidad del cubo
        ladrillos_neg.clone(),     // Material de caucho
    );

    let cuboid13: Cuboid = Cuboid::new(
        Vec3::new(-3.0, 2.0, 2.0), // Centro del cubo
        0.5,                       // Ancho del cubo
        0.5,                       // Altura del cubo
        0.5,                       // Profundidad del cubo
        ladrillos_neg.clone(),     // Material de caucho
    );

    let cuboid14: Cuboid = Cuboid::new(
        Vec3::new(-3.0, 0.0, -1.0), // Centro del cubo
        1.0,                        // Ancho del cubo
        1.0,                        // Altura del cubo
        1.0,                        // Profundidad del cubo
        ladrillos_neg.clone(),      // Material de caucho
    );

    let cuboid15: Cuboid = Cuboid::new(
        Vec3::new(-3.0, 1.0, -1.0), // Centro del cubo
        1.0,                        // Ancho del cubo
        1.0,                        // Altura del cubo
        1.0,                        // Profundidad del cubo
        ladrillos_neg.clone(),      // Material de caucho
    );

    let cuboid16: Cuboid = Cuboid::new(
        Vec3::new(-3.0, 2.0, -1.0), // Centro del cubo
        0.5,                        // Ancho del cubo
        0.5,                        // Altura del cubo
        0.5,                        // Profundidad del cubo
        ladrillos_neg.clone(),      // Material de caucho
    );

    // Suelo
    let cuboid17: Cuboid = Cuboid::new(
        Vec3::new(0.0, 0.0, 0.0), // Centro del cubo
        1.0,                      // Ancho del cubo
        1.0,                      // Altura del cubo
        1.0,                      // Profundidad del cubo
        texture_bricks.clone(),   // Material de caucho
    );

    let cuboid18: Cuboid = Cuboid::new(
        Vec3::new(0.0, 0.0, 1.0), // Centro del cubo
        1.0,                      // Ancho del cubo
        1.0,                      // Altura del cubo
        1.0,                      // Profundidad del cubo
        suelo.clone(),            // Material de caucho
    );

    let cuboid19: Cuboid = Cuboid::new(
        Vec3::new(-1.0, 0.0, 2.0),    // Centro del cubo
        1.0,                          // Ancho del cubo
        1.0,                          // Altura del cubo
        1.0,                          // Profundidad del cubo
        material_con_textura.clone(), // Material de caucho
    );

    let cuboid20: Cuboid = Cuboid::new(
        Vec3::new(-2.0, 0.0, 2.0), // Centro del cubo
        1.0,                       // Ancho del cubo
        1.0,                       // Altura del cubo
        1.0,                       // Profundidad del cubo
        suelo.clone(),             // Material de caucho
    );

    let cuboid21: Cuboid = Cuboid::new(
        Vec3::new(-3.0, 0.0, 0.0), // Centro del cubo
        1.0,                       // Ancho del cubo
        1.0,                       // Altura del cubo
        1.0,                       // Profundidad del cubo
        texture_bricks.clone(),    // Material de caucho
    );

    let cuboid22: Cuboid = Cuboid::new(
        Vec3::new(-3.0, 0.0, 1.0),    // Centro del cubo
        1.0,                          // Ancho del cubo
        1.0,                          // Altura del cubo
        1.0,                          // Profundidad del cubo
        material_con_textura.clone(), // Material de caucho
    );

    let cuboid23: Cuboid = Cuboid::new(
        Vec3::new(-1.0, 0.0, -1.0), // Centro del cubo
        1.0,                        // Ancho del cubo
        1.0,                        // Altura del cubo
        1.0,                        // Profundidad del cubo
        suelo.clone(),              // Material de caucho
    );

    let cuboid24: Cuboid = Cuboid::new(
        Vec3::new(-2.0, 0.0, -1.0), // Centro del cubo
        1.0,                        // Ancho del cubo
        1.0,                        // Altura del cubo
        1.0,                        // Profundidad del cubo
        texture_bricks.clone(),     // Material de caucho
    );

    let objects: Vec<Box<dyn RayIntersect>> = vec![
        // Agua
        Box::new(cuboid1),
        Box::new(cuboid2),
        Box::new(cuboid3),
        Box::new(cuboid4),
        // Columnas de ladrillos
        Box::new(cuboid5),
        Box::new(cuboid6),
        Box::new(cuboid7),
        Box::new(cuboid8),
        Box::new(cuboid9),
        Box::new(cuboid10),
        Box::new(cuboid11),
        Box::new(cuboid12),
        Box::new(cuboid13),
        Box::new(cuboid14),
        Box::new(cuboid15),
        Box::new(cuboid16),
        // Suelo
        Box::new(cuboid17),
        Box::new(cuboid18),
        Box::new(cuboid19),
        Box::new(cuboid20),
        Box::new(cuboid21),
        Box::new(cuboid22),
        Box::new(cuboid23),
        Box::new(cuboid24),
    ];

    let mut angle = 1.0; // Ángulo para el movimiento de la luz

    let mut use_normal_map = false;

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
    let mut m_key_pressed = false;

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

        // Añadir control de zoom
        if window.is_key_down(minifb::Key::W) {
            camera.zoom(-0.1); // Acercar
            needs_render = true;
        }
        if window.is_key_down(minifb::Key::S) {
            camera.zoom(0.1); // Alejar
            needs_render = true;
        }

        // Alternar entre normal map y textura con la tecla "M"
        if window.is_key_down(minifb::Key::M) {
            if !m_key_pressed {
                use_normal_map = !use_normal_map;
                m_key_pressed = true;
                needs_render = true; // Forzar el renderizado después de cambiar
            }
        } else {
            m_key_pressed = false; // Restablecer el estado de la tecla "M"
        }

        // Controlar el ciclo de día y noche con las teclas A y D
        if window.is_key_down(minifb::Key::A) {
            angle -= 0.05; // Girar la luz en sentido antihorario
            needs_render = true;
        }
        if window.is_key_down(minifb::Key::D) {
            angle += 0.05; // Girar la luz en sentido horario
            needs_render = true;
        }

        // Actualizar la posición y el color de las múltiples luces
        for light in &mut lights {
            light.position.x = 10.0 * angle.cos();
            light.position.z = 10.0 * angle.sin();
            light.position.y = 10.0 * angle.sin();

            if light.position.y > 0.0 {
                let intensity_factor = (light.position.y / 10.0).clamp(0.0, 1.0);
                light.color = Color::new(
                    (255.0 * intensity_factor) as i32,
                    (223.0 * intensity_factor) as i32,
                    (191.0 * intensity_factor) as i32,
                );
                light.intensity = 2.0 * intensity_factor;
            } else {
                let intensity_factor = (-light.position.y / 10.0).clamp(0.0, 1.0);
                light.color = Color::new(
                    (64.0 * intensity_factor) as i32,
                    (96.0 * intensity_factor) as i32,
                    (255.0 * intensity_factor) as i32,
                );
                light.intensity = 0.5 * intensity_factor;
            }
        }

        // Solo renderiza si hubo cambios
        if needs_render {
            render(&mut framebuffer, &objects, &camera, &lights, use_normal_map);
            needs_render = false;
        }

        window
            .update_with_buffer(&framebuffer.to_u32_buffer(), width, height)
            .unwrap();
    }
}
