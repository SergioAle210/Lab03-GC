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
}

impl Material {
    pub fn new(
        diffuse: Color,
        specular: f32,
        albedo: [f32; 4],
        refractive_index: f32,
        texture: Option<Arc<Texture>>,
    ) -> Self {
        Material {
            diffuse,
            specular,
            albedo,
            refractive_index,
            texture,
        }
    }

    pub fn black() -> Self {
        Material {
            diffuse: Color::new(0, 0, 0),
            specular: 0.0,
            albedo: [0.0, 0.0, 0.0, 0.0],
            refractive_index: 1.0, // Índice de refracción del vacío
            texture: None,         // Sin textura
        }
    }

    // Función para obtener el color difuso de la textura
    pub fn get_diffuse_color(&self, u: f32, v: f32) -> Color {
        if let Some(texture) = &self.texture {
            let x = (u * (texture.width - 1) as f32) as usize;
            let y = (v * (texture.height - 1) as f32) as usize;
            texture.get_pixel(x, y)
        } else {
            self.diffuse
        }
    }
}
