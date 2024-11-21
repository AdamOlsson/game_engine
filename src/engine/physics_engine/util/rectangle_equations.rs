use super::equations;
use crate::engine::{
    physics_engine::collision::rigid_body::{RigidBody, RigidBodyType},
    util::fixed_float::fixed_float_vector::FixedFloatVector,
};

/// Returns the moment of inertia for a solid rectangle rotating around its center
pub fn inertia(height: f32, width: f32, mass: f32) -> f32 {
    (mass / 12.0) * (height.powi(2) + width.powi(2))
}

/// Returns the left-, right-, top- and bottom-most points of a rotated rectangle
pub fn cardinals(center: &[f32; 3], width: f32, height: f32, rotation: f32) -> [[f32; 3]; 4] {
    let top_left = [-width / 2.0, height / 2.0, 0.0];
    let top_right = [width / 2.0, height / 2.0, 0.0];
    let bot_right = [width / 2.0, -height / 2.0, 0.0];
    let bot_left = [-width / 2.0, -height / 2.0, 0.0];

    let top_left_rot = equations::rotate_z(&top_left, rotation);
    let top_right_rot = equations::rotate_z(&top_right, rotation);
    let bot_right_rot = equations::rotate_z(&bot_right, rotation);
    let bot_left_rot = equations::rotate_z(&bot_left, rotation);

    let top_left_offset = [
        center[0] + top_left_rot[0],
        center[1] + top_left_rot[1],
        0.0,
    ];
    let top_right_offset = [
        center[0] + top_right_rot[0],
        center[1] + top_right_rot[1],
        0.0,
    ];
    let bot_right_offset = [
        center[0] + bot_right_rot[0],
        center[1] + bot_right_rot[1],
        0.0,
    ];
    let bot_left_offset = [
        center[0] + bot_left_rot[0],
        center[1] + bot_left_rot[1],
        0.0,
    ];

    let corners = [
        top_left_offset,
        top_right_offset,
        bot_right_offset,
        bot_left_offset,
    ];
    let left_most = corners
        .iter()
        .min_by(|a, b| a[0].partial_cmp(&b[0]).unwrap())
        .unwrap();
    let right_most = corners
        .iter()
        .max_by(|a, b| a[0].partial_cmp(&b[0]).unwrap())
        .unwrap();
    let top_most = corners
        .iter()
        .max_by(|a, b| a[1].partial_cmp(&b[1]).unwrap())
        .unwrap();
    let bot_most = corners
        .iter()
        .min_by(|a, b| a[1].partial_cmp(&b[1]).unwrap())
        .unwrap();

    return [
        FixedFloatVector::from(*left_most).into(),
        FixedFloatVector::from(*right_most).into(),
        FixedFloatVector::from(*top_most).into(),
        FixedFloatVector::from(*bot_most).into(),
    ];
}

pub fn click_inside(point: (f32, f32), rectangle: &RigidBody) -> bool {
    let (width, height) = match rectangle.body_type {
        RigidBodyType::Rectangle { width, height } => (width, height),
        _ => unreachable!(),
    };

    let transformed_point = [
        point.0 - rectangle.position.x,
        point.1 - rectangle.position.y,
        0.0,
    ];
    let local_point = equations::rotate_z(&transformed_point, -rectangle.rotation);
    local_point[0].abs() <= width / 2.0 && local_point[1].abs() <= height / 2.0
}

/// Computes the world-space coordinates of the four corners of a rectangle,
/// taking into account its position and rotation.
///
/// # Parameters
/// - `body`: A reference to a `RigidBody` representing a rectangle. The function
///   assumes the `RigidBody` is of type `Rectangle`; if not, it will panic.
///
/// # Returns
/// - A `Vec<[f32; 3]>` containing four 3D points (x, y, z = 0) representing the
///   rectangle's corners in world space, after rotation and translation.
///
/// # Details
/// This function calculates the positions of a rectangle's corners in 2D space
/// and applies a rotation around the Z-axis using the `rotation` property from
/// the `body`. The Z-coordinate is set to 0 for each corner point, as the function
/// is intended for 2D operations within a 3D space.
///
/// - The function first extracts the `width` and `height` of the rectangle and computes
///   each corner's initial coordinates based on the rectangle's center.
/// - Rotation is applied individually to each corner using the specified `rotation`
///   angle.
/// - Finally, the rotated corners are translated by the rectangle's `position`,
///   yielding their world-space locations.
///
/// # Panics
/// - Panics if the `RigidBody` is not of type `Rectangle`.
///
/// # Usage
/// This function is useful in collision detection or rendering contexts where
/// the precise positions of the rectangle's corners in world space are required.
pub fn corners(body: &RigidBody) -> Vec<[f32; 3]> {
    let (width, height) = match body.body_type {
        RigidBodyType::Rectangle { width, height } => (width, height),
        _ => panic!("Expected rectangle body"),
    };

    let top_left_rot = equations::rotate_z(&[-width / 2.0, height / 2.0, 0.0], body.rotation);
    let top_right_rot = equations::rotate_z(&[width / 2.0, height / 2.0, 0.0], body.rotation);
    let bot_right_rot = equations::rotate_z(&[width / 2.0, -height / 2.0, 0.0], body.rotation);
    let bot_left_rot = equations::rotate_z(&[-width / 2.0, -height / 2.0, 0.0], body.rotation);
    vec![
        [
            top_left_rot[0] + body.position.x,
            top_left_rot[1] + body.position.y,
            0.0,
        ],
        [
            top_right_rot[0] + body.position.x,
            top_right_rot[1] + body.position.y,
            0.0,
        ],
        [
            bot_right_rot[0] + body.position.x,
            bot_right_rot[1] + body.position.y,
            0.0,
        ],
        [
            bot_left_rot[0] + body.position.x,
            bot_left_rot[1] + body.position.y,
            0.0,
        ],
    ]
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

    mod get_corners {
        use super::super::corners;
        use crate::engine::physics_engine::collision::rigid_body::{
            RigidBodyBuilder, RigidBodyType,
        };
        use crate::engine::util::fixed_float::fixed_float_vector::FixedFloatVector;

        macro_rules! get_corners_tests {
            ($($name:ident: $body: expr, $expected_corner1: expr, $expected_corner2: expr,
                    $expected_corner3: expr, $expected_corner4: expr)*) => {
                $(
                    #[test]
                    fn $name() {
                        let exp1 = $expected_corner1;
                        let exp2 = $expected_corner2;
                        let exp3 = $expected_corner3;
                        let exp4 = $expected_corner4;
                        let corners = corners(&$body);
                        let c1: [f32;3] = FixedFloatVector::from(corners[0]).into();
                        let c2: [f32;3] = FixedFloatVector::from(corners[1]).into();
                        let c3: [f32;3] = FixedFloatVector::from(corners[2]).into();
                        let c4: [f32;3] = FixedFloatVector::from(corners[3]).into();
                        assert_eq!(exp1, c1, "Expected first corner to be {exp1:?} but found {c1:?}");
                        assert_eq!(exp2, c2, "Expected second corner to be {exp2:?} but found {c2:?}");
                        assert_eq!(exp3, c3, "Expected third corner to be {exp3:?} but found {c3:?}");
                        assert_eq!(exp4, c4, "Expected third corner to be {exp4:?} but found {c4:?}");
                    }
                )*
            }
        }

        get_corners_tests! {
            given_rect_is_axis_aligned_and_not_offset_expect_even_corners:
                RigidBodyBuilder::default().id(0)
                .position([0.0,0.0,0.0])
                .body_type(RigidBodyType::Rectangle { width: 10., height: 10.})
                .build(),
            [-5.0, 5.0, 0.0], [5.0, 5.0, 0.0],[5.0, -5.0, 0.0], [-5.0, -5.0, 0.0]

            given_rect_is_axis_aligned_and_offset_expect_even_corners:
                RigidBodyBuilder::default().id(0)
                .position([4.0,5.0,0.0])
                .body_type(RigidBodyType::Rectangle { width: 10., height: 10.})
                .build(),
            [-1.0, 10.0, 0.0], [9.0, 10.0, 0.0],[9.0, 0.0, 0.0], [-1.0, 0.0, 0.0]

            given_rect_is_rotated_and_not_offset_expect_even_corners:
                RigidBodyBuilder::default().id(0)
                .position([0.0,0.0,0.0])
                .rotation(std::f32::consts::PI/4.0)
                .body_type(RigidBodyType::Rectangle { width: 10., height: 10.})
                .build(),
            [-7.071, 0.0, 0.0], [0.0, 7.071, 0.0],[7.071, 0.0, 0.0], [0.0, -7.071, 0.0]

            given_rect_is_rotated_and_offset_expect_even_corners:
                RigidBodyBuilder::default().id(0)
                .position([1.0,2.0,0.0])
                .rotation(std::f32::consts::PI/4.0)
                .body_type(RigidBodyType::Rectangle { width: 10., height: 10.})
                .build(),
            [-6.071, 2.0, 0.0], [1.0, 9.071, 0.0],[8.071, 2.0, 0.0], [1.0, -5.071, 0.0]
        }
    }
}
