use super::sprite_sheet::{SpriteCoordinate, SpriteSheet};

use super::Asset;

pub struct Font {
    font_sprite: SpriteSheet,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Debug)]
pub struct FontInstance {
    pub position: [f32; 3],
    pub font_coord: [f32; 4],
    pub size: f32,
}

/* UTF8 Character Table
 * Characters 0 - 9 = 48 - 57
 * Characters A - Z = 65 - 90
 */

impl Font {
    pub fn new(bytes: &[u8], char_width: u32, char_height: u32) -> Self {
        let font_sprite = SpriteSheet::new(bytes, char_width, char_height);
        Self { font_sprite }
    }

    fn is_number(b: &u8) -> bool {
        48 <= *b && *b <= 57
    }

    fn is_character(b: &u8) -> bool {
        65 <= *b && *b <= 90
    }

    pub fn text_to_coordinates(text: &str) -> Vec<SpriteCoordinate> {
        let upper = text.to_uppercase();
        let bytes = upper.as_bytes();

        let locations = bytes.iter()
            .map(|b| if Self::is_number(b) {
                return b - 48;
            } else if Self::is_character(b) {
                return b - 65 + 10; // +10 to get the position of the character in the sprite sheet
            } else {
                println!("Found invalid u8 character {b}");
                return 0;
            });

        let coordinates: Vec<SpriteCoordinate> = locations
            .map(|l| SpriteCoordinate::new([l as f32, 0.0], [l as f32 + 1., 1.]))
            .collect();

        return coordinates;
    }

    pub fn instance_buffer_desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<FontInstance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 7]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32,
                },
            ],
        }
    }
}

impl Asset for Font {
    fn buffer(&self) -> &image::ImageBuffer<image::Rgba<u8>, Vec<u8>> {
        self.font_sprite.buffer()
    }

    fn specific_data(&self) -> &Vec<f32> {
        self.font_sprite.specific_data()
    }
}

#[cfg(test)]
mod test {
    use super::Font;

    #[test]
    fn zero(){
        let char = "0";
        let expected_out = [0.0,0.0, 1.0,1.0];
        let out = Font::text_to_coordinates(char);
        assert_eq!(out[0].coordinate, expected_out, "Character {char} did not convert to the correct sprite coordinate");
    }

    #[test]
    fn z(){
        let char = "Z";
        let expected_out = [35.0,0.0, 36.0,1.0];
        let out = Font::text_to_coordinates(char);
        assert_eq!(out[0].coordinate, expected_out, "Character {char} did not convert to the correct sprite coordinate");
    }
}
