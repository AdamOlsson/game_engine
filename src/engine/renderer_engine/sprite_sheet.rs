use image::{ImageBuffer, Rgba, RgbaImage};

#[derive(Clone)]
pub struct SpriteSheet {
    pub sprite_buf: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub sprite_data: [f32; 4],
}

impl SpriteSheet {
    pub fn new(bytes: &[u8], cell_width: u32, cell_height: u32) -> Self {
        let texture_sprite_sheet_img = image::load_from_memory(bytes).unwrap();
        let sprite_buf = texture_sprite_sheet_img.to_rgba8();
        let (sprite_width, sprite_height) = sprite_buf.dimensions();
        let sprite_data = [sprite_width as f32, sprite_height as f32, cell_width as f32, cell_height as f32];
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
            sprite_data: [0.0,0.0,0.0,0.0],
        }
    }
}

#[derive(Clone)]
pub struct SpriteCoordinate {
    pub coordinate: [f32; 4],
}

impl SpriteCoordinate {
    pub fn new(top_left_cell: [f32; 2], bot_right_cell: [f32; 2]) -> Self {
        if top_left_cell[0] >= bot_right_cell[0] {
            panic!("Top left x coordinate needs to be smaller than bottom right x coordinate");
        }
        if top_left_cell[1] >= bot_right_cell[1] {
            panic!("Top left y coordinate needs to be smaller than bottom right y coordinate");
        }
       
        Self { coordinate: [
            top_left_cell[0], top_left_cell[1], 
            bot_right_cell[0], bot_right_cell[1]] }
    }

    pub fn none() -> Self {
        Self { coordinate: [-1.0,-1.0,-1.0,-1.0] }
    }
}


#[cfg(test)]
mod test {
    use cgmath::{ElementWise, Vector2};
    
    fn scale_one_coordinate(
        curr: &Vector2<f32>, target_top_left: &Vector2<f32>, target_bot_right: &Vector2<f32>
    ) -> Vector2<f32> {
        return target_top_left + curr.mul_element_wise(target_bot_right.sub_element_wise(*target_top_left));
    }

    #[test]
    fn scale_coordinates_1() {
        let target_top_left = Vector2::new(0.0,0.0);
        let target_bot_right = Vector2::new(1.0,1.0);

        let expected_top_left  = Vector2::new(0.0,0.0);
        let expected_bot_left  = Vector2::new(0.0,1.0);
        let expected_top_right = Vector2::new(1.0,0.0);
        let expected_bot_right  = Vector2::new(1.0,1.0);

        let top_left = Vector2::new(0.0,0.0) ;
        let bot_left = Vector2::new(0.0,1.0) ;
        let top_right = Vector2::new(1.0,0.0) ;
        let bot_right = Vector2::new(1.0,1.0) ;
        
        let scaled_top_left  = scale_one_coordinate(&top_left, &target_top_left, &target_bot_right);
        let scaled_bot_left  = scale_one_coordinate(&bot_left, &target_top_left, &target_bot_right);
        let scaled_top_right = scale_one_coordinate(&top_right, &target_top_left, &target_bot_right);
        let scaled_bot_right = scale_one_coordinate(&bot_right, &target_top_left, &target_bot_right);

        assert_eq!(scaled_top_left, expected_top_left, "Top left did not scale properly");
        assert_eq!(scaled_bot_left, expected_bot_left, "Bot left did not scale properly");
        assert_eq!(scaled_top_right, expected_top_right, "Top right did not scale properly");
        assert_eq!(scaled_bot_right, expected_bot_right, "Bot right did not scale properly");
    }

    #[test]
    fn scale_coordinates_2() {
        let target_top_left = Vector2::new(1.0,1.0);
        let target_bot_right = Vector2::new(3.0,2.0);

        let expected_top_left  = Vector2::new(1.0,1.0);
        let expected_bot_left  = Vector2::new(1.0,2.0);
        let expected_top_right = Vector2::new(3.0,1.0);
        let expected_bot_right  = Vector2::new(3.0,2.0);

        let top_left = Vector2::new(0.0,0.0) ;
        let bot_left = Vector2::new(0.0,1.0) ;
        let top_right = Vector2::new(1.0,0.0) ;
        let bot_right = Vector2::new(1.0,1.0) ;
        
        let scaled_top_left  = scale_one_coordinate(&top_left, &target_top_left, &target_bot_right);
        let scaled_bot_left  = scale_one_coordinate(&bot_left, &target_top_left, &target_bot_right);
        let scaled_top_right = scale_one_coordinate(&top_right, &target_top_left, &target_bot_right);
        let scaled_bot_right = scale_one_coordinate(&bot_right, &target_top_left, &target_bot_right);

        assert_eq!(scaled_top_left, expected_top_left, "Top left did not scale properly");
        assert_eq!(scaled_bot_left, expected_bot_left, "Bot left did not scale properly");
        assert_eq!(scaled_top_right, expected_top_right, "Top right did not scale properly");
        assert_eq!(scaled_bot_right, expected_bot_right, "Bot right did not scale properly");
    }}
