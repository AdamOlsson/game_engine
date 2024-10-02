use super::sprite_sheet::SpriteSheet;

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

pub struct Writer {}

impl Writer {
    pub fn write(text: &str, position: &[f32; 3], size: f32) -> Vec<FontInstance> {
        let upper = text.to_uppercase();
        let bytes = upper.as_bytes();
        
        // All characters are offset by because whitespace is the first char 
        let locations = bytes.iter()
            .map(|b| if Self::is_number(b) {
                return b - 48 + 1;
            } else if Self::is_character(b) {
                return b - 65 + 10 + 1; // +10 to get the position of the character in the sprite sheet
            } else if Self::is_whitespace(b) {
                return 0;
            } else {
                println!("Found invalid u8 character {b}");
                return 0;
            });
        
        let coordinates: Vec<FontInstance> = locations
            .enumerate()
            .map(|(i,l)| FontInstance {
                font_coord: [l as f32, 0.0, l as f32 + 1., 1.],
                position: [(i as f32 * size) + position[0], position[1], position[2]],
                size
            })
            .collect();
        return coordinates;
    }

    fn is_number(b: &u8) -> bool {
        48 <= *b && *b <= 57
    }

    fn is_character(b: &u8) -> bool {
        65 <= *b && *b <= 90
    }

    fn is_whitespace(b: &u8) -> bool {
        *b == 32
    }
}

impl Font {
    pub fn new(bytes: &[u8], char_width: u32, char_height: u32) -> Self {
        let font_sprite = SpriteSheet::new(bytes, char_width, char_height);
        Self { font_sprite }
    }

    pub fn writer(&self) -> Writer {
        Writer {}
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
    use crate::engine::renderer_engine::asset::font::Writer;

    #[test]
    fn zero(){

        let char_width = 11.0;
        let char = "0";
        let expected_out = [1.0,0.0, 2.0,1.0];
        let out = Writer::write(char, &[0.,0.,0.], char_width);
        assert_eq!(out[0].font_coord, expected_out, "Character {char} did not convert to the correct sprite coordinate");
    }

    #[test]
    fn z(){
        let char_width = 11.0;
        let char = "Z";
        let expected_out = [36.0,0.0, 37.0,1.0];
        let out = Writer::write(char, &[0.,0.,0.], char_width);
        assert_eq!(out[0].font_coord, expected_out, "Character {char} did not convert to the correct sprite coordinate");
    }
}
