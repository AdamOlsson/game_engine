use super::sprite_sheet::SpriteSheet;

use super::Asset;


pub struct Font {
    font_sprite: SpriteSheet,
    char_width: u32,
    char_height: u32,
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

pub struct Writer {
    char_width: f32,
    char_height: f32,
}

impl Writer {
    pub fn write(&self, text: &str, position: &[f32; 3]) -> Vec<FontInstance> {
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
        
        let size = 110.0;
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
        Self { font_sprite, char_width, char_height }
    }

    pub fn writer(&self) -> Writer {
        Writer { char_width: self.char_width as f32, char_height: self.char_height as f32 }
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
        let writer = Writer{ char_width: 11.0, char_height: 11.0 };
        let char = "0";
        let expected_out = [0.0,0.0, 1.0,1.0];
        let out = writer.write(char, &[0.,0.,0.]);
        assert_eq!(out[0].font_coord, expected_out, "Character {char} did not convert to the correct sprite coordinate");
    }

    #[test]
    fn z(){
        let writer = Writer{ char_width: 11.0, char_height: 11.0 };
        let char = "Z";
        let expected_out = [35.0,0.0, 36.0,1.0];
        let out = writer.write(char, &[0.,0.,0.]);
        assert_eq!(out[0].font_coord, expected_out, "Character {char} did not convert to the correct sprite coordinate");
    }
}
