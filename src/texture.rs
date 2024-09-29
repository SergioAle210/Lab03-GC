extern crate image;

use crate::color::Color;
use image::{ImageReader, RgbImage};

#[derive(Debug, Clone)]
pub struct Texture {
    image: RgbImage,
    pub width: u32,
    pub height: u32,
}

impl Texture {
    pub fn new(file_path: &str) -> Texture {
        let img = ImageReader::open(file_path)
            .unwrap()
            .decode()
            .unwrap()
            .to_rgb8();
        let width = img.width();
        let height = img.height();
        Texture {
            image: img,
            width,
            height,
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        let clamped_x = x.min((self.width - 1) as usize);
        let clamped_y = y.min((self.height - 1) as usize);
        let pixel = self.image.get_pixel(clamped_x as u32, clamped_y as u32);
        Color::new(pixel[0] as i32, pixel[1] as i32, pixel[2] as i32)
    }
}
