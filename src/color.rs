use std::fmt;
use std::ops::{Add, Mul};

#[derive(Debug, PartialEq, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    // Constructor que recibe valores RGB y hace clamping
    pub fn new(r: i32, g: i32, b: i32) -> Self {
        Self {
            r: Self::clamp(r),
            g: Self::clamp(g),
            b: Self::clamp(b),
        }
    }

    fn clamp(value: i32) -> u8 {
        if value < 0 {
            0
        } else if value > 255 {
            255
        } else {
            value as u8
        }
    }

    // Constructor que recibe un valor hexadecimal u32
    pub fn from_hex(hex: u32) -> Color {
        Color {
            r: ((hex >> 16) & 0xFF) as u8,
            g: ((hex >> 8) & 0xFF) as u8,
            b: (hex & 0xFF) as u8,
        }
    }

    // Método que retorna el valor hexadecimal u32 del color
    pub fn to_hex(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }
}

// Implementar el trait From para la estructura Color
impl From<u32> for Color {
    fn from(hex: u32) -> Self {
        Color::from_hex(hex)
    }
}

// Implementar el trait Add para la estructura Color
impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            r: self.r.saturating_add(other.r),
            g: self.g.saturating_add(other.g),
            b: self.b.saturating_add(other.b),
        }
    }
}

// Implementar el trait Mul para la estructura Color con un float
impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self {
            r: ((self.r as f32 * scalar).clamp(0.0, 255.0)) as u8,
            g: ((self.g as f32 * scalar).clamp(0.0, 255.0)) as u8,
            b: ((self.b as f32 * scalar).clamp(0.0, 255.0)) as u8,
        }
    }
}

// Implementar el trait Display para la estructura Color
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Color(r: {}, g: {}, b: {})", self.r, self.g, self.b)
    }
}
