use super::{background::Background, sprite_sheet::SpriteSheet};

pub struct Asset{}

impl Asset {

    pub fn sprite_sheet(bytes: &[u8], cell_width: u32, cell_height: u32) -> SpriteSheet {
        SpriteSheet::new(bytes,cell_width,cell_height)
    }

    pub fn background(bytes: &[u8]) -> Background {
        Background::new(bytes)
    }
}
