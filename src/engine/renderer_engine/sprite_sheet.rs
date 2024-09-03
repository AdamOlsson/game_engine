use image::{ImageBuffer, Rgba, RgbaImage};

#[derive(Clone)]
pub struct SpriteSheet {
    pub sprite_buf: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub sprite_data: [[f32; 4]; 7], // Needs to be 4 because of memory alignment on gpu
}

impl SpriteSheet {
    pub fn new(bytes: &[u8], cell_width: u32, cell_height: u32) -> Self {
        let texture_sprite_sheet_img = image::load_from_memory(bytes).unwrap();
        let sprite_buf = texture_sprite_sheet_img.to_rgba8();

        let (sprite_width, sprite_height) = sprite_buf.dimensions();

        let px = 1.0 / (sprite_width as f32);
        let cell_right_edge = px*(cell_width as f32);
        let cell_bottom_edge = px*(cell_height as f32);
        let sprite_size_data = [sprite_width as f32, sprite_height as f32, cell_width as f32, cell_height as f32];
        
        let sprite_data = [
            sprite_size_data,
            [0.0, 0.0, 0.0, 0.0],
            [0.0, cell_bottom_edge, 0.0, 0.0],
            [cell_right_edge, 0.0, 0.0, 0.0],
            [cell_right_edge, cell_bottom_edge, 0.0, 0.0],
            [0.0, cell_bottom_edge, 0.0, 0.0],
            [cell_right_edge, 0.0, 0.0, 0.0]
        ];

        Self { sprite_buf, sprite_data }
    }

    pub fn dimensions(&self) -> (u32, u32) {
        self.sprite_buf.dimensions()
    }
}

impl Default for SpriteSheet {
    fn default() -> Self {
        Self {
            sprite_buf: RgbaImage::new(1,1),
            sprite_data: [
                [0.0,0.0,0.0,0.0],
                [0.0,0.0,0.0,0.0],
                [0.0,0.0,0.0,0.0],
                [0.0,0.0,0.0,0.0],
                [0.0,0.0,0.0,0.0],
                [0.0,0.0,0.0,0.0],
                [0.0,0.0,0.0,0.0],
            ]
        }
    }
}
