use std::sync::Arc;

use crate::color::Color;
use crate::texture::Texture;

#[derive(Debug, Clone)]
pub struct Material {
    pub diffuse: Color,
    pub specular: f32,
    pub albedo: [f32; 4],      // [difusa, especular, reflectiva, transparente]
    pub refractive_index: f32, // Índice de refracción
    pub texture: Option<Arc<Texture>>, // Referencia a la textura
    pub animation_speed: Option<(f32, f32)>, // Velocidad de animación de la textura en U y V
}

impl Material {
    pub fn new(
        diffuse: Color,
        specular: f32,
        albedo: [f32; 4],
        refractive_index: f32,
        texture: Option<Arc<Texture>>,
        animation_speed: Option<(f32, f32)>, // Velocidad de animación en U y V
    ) -> Self {
        Material {
            diffuse,
            specular,
            albedo,
            refractive_index,
            texture,
            animation_speed,
        }
    }

    pub fn black() -> Self {
        Material {
            diffuse: Color::new(0, 0, 0), // Color negro
            specular: 0.0,
            albedo: [0.0, 0.0, 0.0, 0.0],
            refractive_index: 1.0,
            texture: None,
            animation_speed: None,
        }
    }

    // Función para obtener el color difuso de la textura
    pub fn get_diffuse_color(&self, u: f32, v: f32, time: f32) -> Color {
        if let Some(texture) = &self.texture {
            // Desplazar las coordenadas de textura si hay animación
            let (u_offset, v_offset) = self.animation_speed.unwrap_or((0.0, 0.0));
            let animated_u = (u + time * u_offset) % 1.0; // Desplazamiento en U
            let animated_v = (v + time * v_offset) % 1.0; // Desplazamiento en V

            let x = (animated_u * (texture.width - 1) as f32) as usize;
            let y = (animated_v * (texture.height - 1) as f32) as usize;
            texture.get_pixel(x, y)
        } else {
            self.diffuse
        }
    }
}
