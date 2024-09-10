use image::{ImageBuffer, Rgba};

pub mod asset;
pub mod sprite_sheet;
pub mod background;

pub trait Asset {

    fn buffer(&self) -> &ImageBuffer<Rgba<u8>, Vec<u8>>;
    fn specific_data(&self) -> &Vec<f32>; 
}
