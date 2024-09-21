use crate::color::Color;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub diffuse: Color,
    pub specular: f32, // Coeficiente especular
}

impl Material {
    pub fn new(diffuse: Color, specular: f32) -> Self {
        Material { diffuse, specular }
    }

    pub fn black() -> Self {
        Material {
            diffuse: Color::new(0, 0, 0),
            specular: 0.0,
        }
    }
}
