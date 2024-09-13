use image::{ImageBuffer, Rgba, RgbaImage};

use super::Asset;

#[derive(Clone)]
pub struct SpriteSheet {
    pub sprite_buf: ImageBuffer<Rgba<u8>, Vec<u8>>,
    pub sprite_data: Vec<f32>,
}

impl SpriteSheet {
    pub fn new(bytes: &[u8], cell_width: u32, cell_height: u32) -> Self {
        let texture_sprite_sheet_img = image::load_from_memory(bytes).unwrap();
        let sprite_buf = texture_sprite_sheet_img.to_rgba8();
        let (sprite_width, sprite_height) = sprite_buf.dimensions();
        let sprite_data = [sprite_width as f32, sprite_height as f32, cell_width as f32, cell_height as f32].to_vec();
        Self { sprite_buf, sprite_data }
    }
}

impl Asset for SpriteSheet {
    fn buffer(&self) -> &ImageBuffer<Rgba<u8>, Vec<u8>> {
        &self.sprite_buf
    }

    fn specific_data(&self) -> &Vec<f32> {
        &self.sprite_data
    }
}


impl Default for SpriteSheet {
    fn default() -> Self {
        Self {
            sprite_buf: RgbaImage::new(1,1),
            sprite_data: [0.0,0.0,0.0,0.0].to_vec(),
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
    
    fn scale_one_coordinate_rect(
        curr: &Vector2<f32>, target_top_left: &Vector2<f32>, target_bot_right: &Vector2<f32>
    ) -> Vector2<f32> {
        return target_top_left + curr.mul_element_wise(target_bot_right.sub_element_wise(*target_top_left));
    }

    #[test]
    fn scale_coordinates_rect_1() {
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
        
        let scaled_top_left  = scale_one_coordinate_rect(&top_left, &target_top_left, &target_bot_right);
        let scaled_bot_left  = scale_one_coordinate_rect(&bot_left, &target_top_left, &target_bot_right);
        let scaled_top_right = scale_one_coordinate_rect(&top_right, &target_top_left, &target_bot_right);
        let scaled_bot_right = scale_one_coordinate_rect(&bot_right, &target_top_left, &target_bot_right);

        assert_eq!(scaled_top_left, expected_top_left, "Top left did not scale properly (actual, expected)");
        assert_eq!(scaled_bot_left, expected_bot_left, "Bot left did not scale properly (actual, expected)");
        assert_eq!(scaled_top_right, expected_top_right, "Top right did not scale properly (actual, expected)");
        assert_eq!(scaled_bot_right, expected_bot_right, "Bot right did not scale properly (actual, expected)");
    }

    #[test]
    fn scale_coordinates_rect_2() {
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
        
        let scaled_top_left  = scale_one_coordinate_rect(&top_left, &target_top_left, &target_bot_right);
        let scaled_bot_left  = scale_one_coordinate_rect(&bot_left, &target_top_left, &target_bot_right);
        let scaled_top_right = scale_one_coordinate_rect(&top_right, &target_top_left, &target_bot_right);
        let scaled_bot_right = scale_one_coordinate_rect(&bot_right, &target_top_left, &target_bot_right);

        assert_eq!(scaled_top_left, expected_top_left, "Top left did not scale properly (actual, expected)");
        assert_eq!(scaled_bot_left, expected_bot_left, "Bot left did not scale properly (actual, expected)");
        assert_eq!(scaled_top_right, expected_top_right, "Top right did not scale properly (actual, expected)");
        assert_eq!(scaled_bot_right, expected_bot_right, "Bot right did not scale properly (actual, expected)");
    }

    fn scale_one_coordinate_circle(
        vertex_position: &Vector2<f32>,
        target_top_left: &Vector2<f32>, target_bot_right: &Vector2<f32>
    ) -> Vector2<f32> {
        let target_dimensions = target_bot_right - target_top_left;
        let target_center = (target_top_left + target_bot_right) / 2.0;
        let target_radius = target_dimensions / 2.0;
        return (vertex_position.mul_element_wise(target_radius)) + target_center;
    }

    #[test]
    fn scale_coordinates_circle_1() {
        let target_top_left = Vector2::new(0.0,0.0);
        let target_bot_right = Vector2::new(1.0,1.0);
        
        let expected_center = Vector2::new(0.5, 0.5);
        let expected_right = Vector2::new(1.0, 0.5);
        let expected_top = Vector2::new(0.5, 1.0);
        let expected_left = Vector2::new(0.0, 0.5);
        let expected_bot = Vector2::new(0.5, 0.0);

        let vertex_center = Vector2::new(0.0,0.0);
        let vertex_right = Vector2::new(1.0,0.0);
        let vertex_top = Vector2::new(0.0,1.0);
        let vertex_left = Vector2::new(-1.0,0.0);
        let vertex_bot = Vector2::new(0.0,-1.0);

        let new_vertex_center = scale_one_coordinate_circle(&vertex_center, &target_top_left, &target_bot_right);
        let new_vertex_right = scale_one_coordinate_circle(&vertex_right, &target_top_left, &target_bot_right);
        let new_vertex_top = scale_one_coordinate_circle(&vertex_top, &target_top_left, &target_bot_right);
        let new_vertex_left = scale_one_coordinate_circle(&vertex_left, &target_top_left, &target_bot_right);
        let new_vertex_bot = scale_one_coordinate_circle(&vertex_bot, &target_top_left, &target_bot_right);

        assert_eq!(new_vertex_center, expected_center, "Center did not scale properly (actual, expected)");
        assert_eq!(new_vertex_right, expected_right, "Right did not scale properly (actual, expected)");
        assert_eq!(new_vertex_top, expected_top, "Top did not scale properly (actual, expected)");
        assert_eq!(new_vertex_left, expected_left, "Left did not scale properly (actual, expected)");
        assert_eq!(new_vertex_bot, expected_bot, "Bot did not scale properly (actual, expected)");
    }


    #[test]
    fn scale_coordinates_circle_2() {
        let target_top_left = Vector2::new(1.0,2.0);
        let target_bot_right = Vector2::new(2.0,3.0);
        
        let expected_center = Vector2::new(1.5, 2.5);
        let expected_right = Vector2::new(2.0, 2.5);
        let expected_top = Vector2::new(1.5, 3.0);
        let expected_left = Vector2::new(1.0, 2.5);
        let expected_bot = Vector2::new(1.5, 2.0);

        let vertex_center = Vector2::new(0.0,0.0);
        let vertex_right = Vector2::new(1.0,0.0);
        let vertex_top = Vector2::new(0.0,1.0);
        let vertex_left = Vector2::new(-1.0,0.0);
        let vertex_bot = Vector2::new(0.0,-1.0);

        let new_vertex_center = scale_one_coordinate_circle(&vertex_center, &target_top_left, &target_bot_right);
        let new_vertex_right = scale_one_coordinate_circle(&vertex_right, &target_top_left, &target_bot_right);
        let new_vertex_top = scale_one_coordinate_circle(&vertex_top, &target_top_left, &target_bot_right);
        let new_vertex_left = scale_one_coordinate_circle(&vertex_left, &target_top_left, &target_bot_right);
        let new_vertex_bot = scale_one_coordinate_circle(&vertex_bot, &target_top_left, &target_bot_right);

        assert_eq!(new_vertex_center, expected_center, "Center did not scale properly (actual, expected)");
        assert_eq!(new_vertex_right, expected_right, "Right did not scale properly (actual, expected)");
        assert_eq!(new_vertex_top, expected_top, "Top did not scale properly (actual, expected)");
        assert_eq!(new_vertex_left, expected_left, "Left did not scale properly (actual, expected)");
        assert_eq!(new_vertex_bot, expected_bot, "Bot did not scale properly (actual, expected)");
    }
}
