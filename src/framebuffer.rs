use crate::color::Color;
pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<Color>,
    pub background_color: Color,
    pub current_color: Color,
}

impl Framebuffer {
    // Constructor para crear un nuevo framebuffer con ancho y alto dados
    pub fn new(width: usize, height: usize) -> Self {
        let buffer = vec![Color::new(0, 0, 0); width * height]; // Inicializar con color negro
        Self {
            buffer,
            width,
            height,
            background_color: Color::new(0, 0, 0), // Color negro como predeterminado
            current_color: Color::new(255, 255, 255), // Color blanco como predeterminado
        }
    }

    pub fn point_with_color(&mut self, x: usize, y: usize, color: Color) {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] = color;
        }
    }

    // Métodos para settear los colores de fondo y actual
    pub fn set_background_color(&mut self, color: impl Into<Color>) {
        self.background_color = color.into();
    }

    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = Color::from_hex(color);
    }

    pub fn draw_rectangle(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
        color: Color,
    ) {
        for i in 0..width {
            for j in 0..height {
                self.point_with_color(x + i, y + j, color.clone());
            }
        }
    }

    // Función para limpiar el framebuffer con el color de fondo
    pub fn clear(&mut self) {
        for pixel in &mut self.buffer {
            *pixel = self.background_color.clone();
        }
    }

    pub fn is_point_set(&self, x: usize, y: usize) -> bool {
        if x < self.width && y < self.height {
            self.buffer[y * self.width + x] == Color::from_hex(0xFFFFFF)
        } else {
            false
        }
    }

    pub fn to_u32_buffer(&self) -> Vec<u32> {
        self.buffer.iter().map(|color| color.to_hex()).collect()
    }
}

fn blend_colors(existing: u32, new: u32, alpha: u32) -> u32 {
    let existing_r = (existing >> 16) & 0xFF;
    let existing_g = (existing >> 8) & 0xFF;
    let existing_b = existing & 0xFF;

    let new_r = (new >> 16) & 0xFF;
    let new_g = (new >> 8) & 0xFF;
    let new_b = new & 0xFF;

    let blended_r = (existing_r * (255 - alpha) + new_r * alpha) / 255;
    let blended_g = (existing_g * (255 - alpha) + new_g * alpha) / 255;
    let blended_b = (existing_b * (255 - alpha) + new_b * alpha) / 255;

    (blended_r << 16) | (blended_g << 8) | blended_b
}
