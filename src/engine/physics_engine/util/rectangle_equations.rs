use crate::engine::util::fixed_float::fixed_float_vector::FixedFloatVector;

use super::equations;

/// Returns the moment of inertia for a solid rectangle rotating around its center
pub fn inertia(height:f32, width:f32, mass:f32) -> f32 {
    (mass/12.0)*(height.powi(2) + width.powi(2))
}

/// Returns the left-, right-, top- and bottom-most points of a rotated rectangle
pub fn cardinals(center: &[f32;3], width: f32, height:f32, rotation: f32) -> [[f32;3];4]{
    let top_left  = [-width/2.0,  height/2.0, 0.0];
    let top_right = [ width/2.0,  height/2.0, 0.0];
    let bot_right = [ width/2.0, -height/2.0, 0.0];
    let bot_left  = [-width/2.0, -height/2.0, 0.0];

    let top_left_rot = equations::rotate_z(&top_left, rotation);
    let top_right_rot = equations::rotate_z(&top_right, rotation);
    let bot_right_rot = equations::rotate_z(&bot_right, rotation);
    let bot_left_rot = equations::rotate_z(&bot_left, rotation);
    
    let top_left_offset  = [center[0] + top_left_rot[0],  center[1] + top_left_rot[1],  0.0];
    let top_right_offset = [center[0] + top_right_rot[0], center[1] + top_right_rot[1], 0.0];
    let bot_right_offset = [center[0] + bot_right_rot[0], center[1] + bot_right_rot[1], 0.0];
    let bot_left_offset  = [center[0] + bot_left_rot[0],  center[1] + bot_left_rot[1],  0.0];

    let corners = [top_left_offset, top_right_offset, bot_right_offset, bot_left_offset];
    let left_most = corners.iter().min_by(|a,b| a[0].partial_cmp(&b[0]).unwrap()).unwrap();
    let right_most = corners.iter().max_by(|a,b| a[0].partial_cmp(&b[0]).unwrap()).unwrap();
    let top_most = corners.iter().max_by(|a,b| a[1].partial_cmp(&b[1]).unwrap()).unwrap();
    let bot_most = corners.iter().min_by(|a,b| a[1].partial_cmp(&b[1]).unwrap()).unwrap();

    return [
        FixedFloatVector::from(*left_most).into(), 
        FixedFloatVector::from(*right_most).into(),
        FixedFloatVector::from(*top_most).into(),
        FixedFloatVector::from(*bot_most).into()];
}


#[cfg(test)]
mod rectangle_equations_test {
    mod cardinals {
        use super::super::cardinals;
        macro_rules! cardinals_test {
            ($($name:ident: $center: expr, $width: expr, $height: expr, $rotation: expr,
                $expected_left: expr, $expected_right: expr, $expected_top:expr, $expected_bot: expr)*) => {
                $(
                    #[test]
                    fn $name() {
                        let exp_left = $expected_left;
                        let exp_right = $expected_right;
                        let exp_top = $expected_top;
                        let exp_bot = $expected_bot;
                        let outputs = cardinals($center, $width, $height, $rotation);
                        let out_left = outputs[0];
                        let out_right = outputs[1];
                        let out_top = outputs[2];
                        let out_bot = outputs[3];
                        assert_eq!(exp_left, out_left, "Expected left most {exp_left:?} found {out_left:?}");
                        assert_eq!(exp_right, out_right, "Expected right most {exp_right:?} found {out_right:?}");
                        assert_eq!(exp_top, out_top, "Expected top most {exp_top:?} found {out_top:?}");
                        assert_eq!(exp_bot, out_bot, "Expected bottom most {exp_bot:?} found {out_bot:?}");
                    }
                )*
            }
        }
    
        cardinals_test! {
            given_rect_when_aabb_and_no_rotation_expect_corners:
                &[0.,0.,0.], 2.0, 2.0, 0.0,
                [-1.0,1.0,0.0],[1.0,-1.0,0.0],[1.0,1.0,0.0],[1.0,-1.0,0.0]
            given_rect_when_aabb_and_90_degrees_rotation_expect_corners:
                &[0.,0.,0.], 2.0, 2.0, std::f32::consts::PI/2.0,
                [-1.0,-1.0,0.0],[1.0,-1.0,0.0],[1.0,1.0,0.0],[-1.0,-1.0,0.0]
            given_rect_when_aabb_and_30_degrees_rotation_expect_corners:
                &[0.,0.,0.], 2.0, 2.0, std::f32::consts::PI/6.,
                [-1.366,0.366,0.0],[1.366,-0.366,0.0],[0.366,1.366,0.0],[-0.366,-1.366,0.0]
            
            given_rect_when_aabb_and_30_degrees_rotation_and_offset_expect_corners:
                &[1.,0.,0.], 2.0, 2.0, std::f32::consts::PI/6.,
                [-0.366,0.366,0.0],[2.366,-0.366,0.0],[1.366,1.366,0.0],[0.634,-1.366,0.0]

        }
    }
}
