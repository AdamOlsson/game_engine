use image::{ImageBuffer, Rgba};

use super::Asset;

#[derive(Clone)]
pub struct Background {
    pub img_buf: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub img_data: Vec<f32>,
}

impl Background {
    pub fn new(bytes: &[u8]) -> Self {
        let background_img = image::load_from_memory(bytes).unwrap();
        let img_buf = background_img.to_rgba8();
        let (img_width, img_height) = img_buf.dimensions();
        let img_data = [img_width as f32, img_height as f32].to_vec();
        Self { img_buf, img_data }
    }
}

impl Asset for Background {
    fn buffer(&self) -> &ImageBuffer<Rgba<u8>, Vec<u8>> {
        &self.img_buf
    }

    fn specific_data(&self) -> &Vec<f32> {
        &self.img_data
    }
}
