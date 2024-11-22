use super::super::CollisionInformation;
use crate::engine::physics_engine::collision::rigid_body::{RigidBody, RigidBodyType};
use crate::engine::physics_engine::util::equations;

#[derive(Debug)]
pub struct Projection {
    pub min: f32,
    pub min_corner: [f32; 3],
    pub max: f32,
    pub max_corner: [f32; 3],
}

impl Projection {
    pub fn no_axis(min: f32, max: f32) -> Self {
        Self {
            min,
            min_corner: [0., 0., 0.],
            max,
            max_corner: [0., 0., 0.],
        }
    }
}

#[derive(Debug)]
pub struct Overlap {
    pub distance: f32,
    pub min: [f32; 3],
    pub max: [f32; 3],
}

/// Computes the primary axes to test for a Separating Axis Theorem (SAT) collision
/// between rectangles in 2D space.
///
/// # Parameters
/// - `body`: A reference to a `RigidBody` representing a rectangle. The function
///   assumes the `RigidBody` is of type `Rectangle`; if not, it will panic.
///
/// # Returns
/// - A 2x3 array containing two normalized axis vectors (in 3D form) perpendicular
///   to the rectangle's edges. These axes are necessary for performing SAT-based
///   collision detection.
///
/// # Details
/// This function calculates the two primary axes of separation for the given rectangle,
/// which are based on its rotated edges. The rotation is applied around the Z-axis using
/// the body's `rotation` property.
///
/// - First, the function determines the rectangle's `width` and `height` and then finds
///   three of its corner points in world space (top-left, top-right, and bottom-right),
///   accounting for rotation.
/// - The edge vectors `axis1` (bottom-right to top-right) and `axis2` (top-right to
///   top-left) are computed, and then the perpendicular (normal) vectors for each axis
///   are derived to obtain the directions to test for separation.
/// - Finally, both perpendicular vectors are normalized, ensuring that they are unit
///   vectors suitable for projection in SAT testing.
///
/// # Panics
/// - Panics if the `RigidBody` is not of type `Rectangle`.
///
/// # Usage
/// This function is used primarily in collision detection algorithms where SAT
/// is employed to determine if two rectangles are intersecting. The returned axes
/// are used to project both rectangles and check for overlap, allowing for precise
/// collision determination.
pub fn sat_get_axii(body: &RigidBody) -> Vec<[f32; 3]> {
    let (width, height) = match body.body_type {
        RigidBodyType::Rectangle { width, height } => (width, height),
        _ => panic!("Expected rectangle body"),
    };

    let top_left = equations::rotate_z(
        &[
            body.position.x - width / 2.0,
            body.position.y + height / 2.0,
            0.0,
        ],
        body.rotation,
    );
    let top_right = equations::rotate_z(
        &[
            body.position.x + width / 2.0,
            body.position.y + height / 2.0,
            0.0,
        ],
        body.rotation,
    );
    let bot_right = equations::rotate_z(
        &[
            body.position.x + width / 2.0,
            body.position.y - height / 2.0,
            0.0,
        ],
        body.rotation,
    );

    let axis1 = [
        bot_right[0] - top_right[0],
        bot_right[1] - top_right[1],
        bot_right[2] - top_right[2],
    ];
    let axis2 = [
        top_right[0] - top_left[0],
        top_right[1] - top_left[1],
        top_right[2] - top_left[2],
    ];
    let mut normal1 = equations::perpendicular_2d(&axis1);
    let mut normal2 = equations::perpendicular_2d(&axis2);
    equations::normalize(&mut normal1);
    equations::normalize(&mut normal2);
    vec![normal1, normal2]
}

/// Projects the corners of a rectangle onto a given axis to determine the minimum
/// and maximum extents along that axis for Separating Axis Theorem (SAT) collision
/// detection.
///
/// # Parameters
/// - `body`: A reference to a `RigidBody` representing a rectangle. The function
///   assumes the `RigidBody` is of type `Rectangle`.
/// - `axis`: A 3D vector representing the axis onto which the rectangle's corners
///   will be projected. Only the x and y components are relevant as the function
///   operates in 2D space.
///
/// # Returns
/// - A struct `Projection` containing the minimum and maximum projection values along
///   the specified axis. These values define the projected interval of the rectangle
///   along the axis, which can be used to check for overlap in SAT-based collision
///   detection.
///
/// # Details
/// This function utilizes the rectangle's corners, which are computed by the `corners`
/// function. Each corner is projected onto the provided `axis` using the dot product,
/// and the minimum and maximum projections are recorded to define the range of
/// the rectangle along the axis.
///
/// - First, the `corners` function is called to get the world-space coordinates of
///   the rectangle's corners.
/// - Each corner is projected onto `axis` using the dot product, yielding a scalar
///   value that represents the corner’s distance along the axis.
/// - The minimum and maximum projection values are computed across all corners to
///   define the rectangle's interval on the axis.
///
/// # Usage
/// This function is essential for SAT collision detection between rectangles, as it
/// provides the projection intervals required to check for overlap along potential
/// separating axes.
///
/// # Panics
/// - Panics if the `RigidBody` is not of type `Rectangle`.
pub fn sat_project_on_axis(body: &RigidBody, axis: &[f32; 3]) -> Projection {
    let corners = body.corners();

    // Fold with initial values that include both min/max values and indices of the corners
    let (min, min_corner_idx, max, max_corner_idx) = corners
        .iter()
        .enumerate()
        .map(|(i, c)| (equations::dot(axis, c), i))
        .fold(
            (f32::MAX, 0, f32::MIN, 0), // Initial values: min, min_index, max, max_index
            |(min, min_idx, max, max_idx), (value, idx)| {
                (
                    if value < min { value } else { min },
                    if value < min { idx } else { min_idx },
                    if value > max { value } else { max },
                    if value > max { idx } else { max_idx },
                )
            },
        );
    let min_corner = corners[min_corner_idx];
    let max_corner = corners[max_corner_idx];
    Projection {
        min,
        min_corner,
        max,
        max_corner,
    }
}

/// Determines whether there is an overlap between the projection intervals of
/// two shapes along a specific axis, as part of the Separating Axis Theorem (SAT)
/// collision detection process.
///
/// # Parameters
/// - `proj_a`: A reference to a `Projection` representing the minimum and maximum
///   extents of the first shape along the axis.
/// - `proj_b`: A reference to a `Projection` representing the minimum and maximum
///   extents of the second shape along the same axis.
///
/// # Returns
/// - `true` if there is an overlap between the two projection intervals, indicating
///   that the two shapes are not separated along this axis and may be colliding.
/// - `false` if there is no overlap between the intervals, indicating that the
///   shapes are separated along this axis and cannot be colliding.
///
/// # Details
/// In the context of SAT-based collision detection, each shape's projection onto
/// a potential separating axis forms an interval defined by minimum and maximum
/// values. If any axis exists along which these intervals do not overlap, then
/// the two shapes are guaranteed not to collide. Conversely, if all potential
/// separating axes show an overlap, a collision is occurring.
///
/// This function checks for overlap by verifying that:
/// - The maximum of `proj_a` is greater than the minimum of `proj_b`
/// - The maximum of `proj_b` is greater than the minimum of `proj_a`
///
/// If both conditions are met, the intervals overlap.
///
/// # Usage
/// This function is used within SAT collision detection algorithms to determine
/// if two shapes are overlapping along a particular axis. It is called for each
/// axis that could potentially separate the shapes.
pub fn sat_projection_overlap(proj_a: &Projection, proj_b: &Projection) -> bool {
    proj_a.min < proj_b.max && proj_b.min < proj_a.max
}

/// Calculates the overlap distance between the projection intervals of two shapes
/// along a specific axis, as part of the Separating Axis Theorem (SAT) collision
/// detection process.
///
/// # Parameters
/// - `proj_a`: A reference to a `Projection` representing the minimum and maximum
///   extents of the first shape along the axis.
/// - `proj_b`: A reference to a `Projection` representing the minimum and maximum
///   extents of the second shape along the same axis.
///
/// # Returns
/// - A `f32` value representing the distance of the overlap between `proj_a` and
///   `proj_b` along the axis. A positive value indicates the amount of overlap,
///   while a non-positive value suggests no overlap.
///
/// # Details
/// In SAT collision detection, if two shapes are overlapping along all axes, the
/// degree of overlap along each axis can be computed. This function calculates the
/// overlap by finding the minimum of the maximum projections and subtracting the
/// maximum of the minimum projections:
///
/// - `overlap = min(proj_a.max, proj_b.max) - max(proj_a.min, proj_b.min)`
///
/// If this result is positive, it represents the amount of overlap along the axis.
/// If non-positive, it indicates no overlap, and thus no collision along that axis.
///
/// # Usage
/// This function is used in collision response algorithms where the depth of
/// penetration (overlap) between two shapes is needed to calculate the necessary
/// response or correction for intersecting shapes.
///
/// # Note
/// This function assumes that the overlap check (`sat_projection_overlap`) has
/// already confirmed an intersection along the axis. If not, the result could be
/// zero or negative, indicating no overlap.
pub fn sat_overlap_distance(proj_a: &Projection, proj_b: &Projection) -> Overlap {
    let max = std::cmp::min_by(proj_a, proj_b, |a, b| a.max.total_cmp(&b.max));
    let min = std::cmp::max_by(proj_a, proj_b, |a, b| a.min.total_cmp(&b.min));
    let distance = max.max - min.min;
    Overlap {
        distance,
        min: min.min_corner,
        max: max.max_corner,
    }
}

/// Performs collision detection between two rectangular `RigidBody` objects using
/// the Separating Axis Theorem (SAT).
///
/// # Parameters
/// - `body_a`: A reference to the first `RigidBody`.
/// - `body_b`: A reference to the second `RigidBody`.
///
/// # Returns
/// - `Some((f32, [f32; 3]))` if a collision is detected, where:
///   - `f32` is the minimum overlap distance (depth of penetration) between the
///     two rectangles along the collision axis.
///   - `[f32; 3]` is the collision axis vector, representing the axis along which
///     the shapes are intersecting. **Note**: No guarantee is made regarding the
///     direction of this axis (e.g., pointing towards a specific object).
/// - `None` if no collision is detected, meaning there is an axis along which the
///   two rectangles' projections do not overlap.
///
/// # Details
/// This function applies the SAT to determine if two rectangles are colliding.
/// The process includes:
/// 1. Retrieving the axes (perpendiculars to edges) of each rectangle by calling
///    `sat_get_axii` on both `body_a` and `body_b`.
/// 2. Projecting both rectangles onto each of the axes from `body_a` and `body_b`.
/// 3. For each axis, computing the overlap distance using `sat_overlap_distance`.
///    If any axis results in zero or negative overlap, the bodies are not colliding.
///
/// - The function iterates over all axes of both bodies, maintaining the minimum
///   overlap distance and axis (the "collision axis").
///
/// - The minimum overlap and axis values are returned to provide the depth and
///   direction of collision, which can be used in collision response calculations.
///
/// # Usage
/// This function is typically called to check for collisions between two rectangles
/// in a 2D physics engine. The returned penetration depth and axis vector can be
/// used to calculate the necessary corrective response if the objects are found
/// to be intersecting.
pub fn sat_collision_detection(
    body_a: &RigidBody,
    body_b: &RigidBody,
) -> Option<CollisionInformation> {
    let axii_a = sat_get_axii(&body_a);
    let axii_b = sat_get_axii(&body_b);

    // iterators are 0 cost, create them all
    let projections_body_a_on_axii_a = axii_a.iter().map(|ax| sat_project_on_axis(&body_a, ax));
    let projections_body_b_on_axii_a = axii_a.iter().map(|ax| sat_project_on_axis(&body_b, ax));
    let projections_axii_a =
        std::iter::zip(projections_body_a_on_axii_a, projections_body_b_on_axii_a);
    let overlap_per_axii_a =
        projections_axii_a.map(|(proj_a, proj_b)| sat_overlap_distance(&proj_a, &proj_b));

    let (index_axii_a, min_overlap_on_a) = overlap_per_axii_a
        .enumerate()
        .min_by(|(_, overlap_a), (_, overlap_b)| overlap_a.distance.total_cmp(&overlap_b.distance))
        .expect("Expected there to be axii to perform overlap checks on");

    if min_overlap_on_a.distance <= 0.0 {
        // We found an axis where the projections do not overlap and therefore
        // does not the bodies overlap
        return None;
    }

    let projections_body_a_on_axii_b = axii_b.iter().map(|ax| sat_project_on_axis(&body_a, ax));
    let projections_body_b_on_axii_b = axii_b.iter().map(|ax| sat_project_on_axis(&body_b, ax));
    let projections_axii_b =
        std::iter::zip(projections_body_a_on_axii_b, projections_body_b_on_axii_b);
    let overlap_per_axii_b =
        projections_axii_b.map(|(proj_a, proj_b)| sat_overlap_distance(&proj_a, &proj_b));

    let (index_axii_b, min_overlap_on_b) = overlap_per_axii_b
        .enumerate()
        .min_by(|(_, overlap_a), (_, overlap_b)| overlap_a.distance.total_cmp(&overlap_b.distance))
        .expect("Expected there to be axii to perform overlap checks on");

    if min_overlap_on_b.distance <= 0.0 {
        // We found an axis where the projections do not overlap and therefore
        // does not the bodies overlap
        return None;
    }

    let (axii, index, overlap) = std::cmp::min_by(
        (axii_a, index_axii_a, min_overlap_on_a),
        (axii_b, index_axii_b, min_overlap_on_b),
        |a, b| a.2.distance.total_cmp(&b.2.distance),
    );

    let axis = axii[index];
    let collision_point = [
        overlap.min[0] + axis[0] * overlap.distance,
        overlap.min[1] + axis[1] * overlap.distance,
        overlap.min[2] + axis[2] * overlap.distance,
    ];

    let collision_info = CollisionInformation {
        penetration_depth: overlap.distance,
        normal: axis,
        collision_point,
    };

    return Some(collision_info);
}

#[cfg(test)]
mod sat_test {

    mod sat_get_axii {
        use super::super::sat_get_axii;
        use crate::engine::physics_engine::collision::rigid_body::{
            RigidBodyBuilder, RigidBodyType,
        };
        use crate::engine::util::fixed_float::fixed_float_vector::FixedFloatVector;
        macro_rules! sat_get_axii_tests {
            ($($name:ident: $body: expr, $expected_axis1: expr, $expected_axis2: expr)*) => {
                $(
                    #[test]
                    fn $name() {
                        let exp1 = $expected_axis1;
                        let exp2 = $expected_axis2;
                        let axii = sat_get_axii(&$body);
                        let ax1: [f32;3] = FixedFloatVector::from(axii[0]).into();
                        let ax2: [f32;3] = FixedFloatVector::from(axii[1]).into();
                        assert_eq!(exp1, ax1, "Expected first normal to be {exp1:?} but found {ax1:?}");
                        assert_eq!(exp2, ax2, "Expected second normal to be {exp2:?} but found {ax2:?}");
                    }
                )*
            }
        }

        sat_get_axii_tests! {
            given_rect_with_no_rotation_expect_axis_aligned_axii:
                RigidBodyBuilder::default().id(0)
                    .position([0.0,0.0,0.0])
                    .body_type(RigidBodyType::Rectangle { width: 10., height: 10.})
                    .build(),
              [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]
            given_rect_is_offset_with_no_rotation_expect_axis_aligned_axii:
                RigidBodyBuilder::default().id(0)
                    .position([7.0,-6.0,0.0])
                    .body_type(RigidBodyType::Rectangle { width: 10., height: 10.})
                    .build(),
              [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]
            given_rect_is_offset_with_45_degree_rotation_expect_axis_aligned_axii:
                RigidBodyBuilder::default().id(0)
                    .position([7.0,-6.0,0.0])
                    .rotation(std::f32::consts::PI/4.0)
                    .body_type(RigidBodyType::Rectangle { width: 10., height: 10.})
                    .build(),
              [0.707, 0.707, 0.0], [-0.707, 0.707, 0.0]
            given_rect_is_offset_with_45_degree_rotation_and_uneven_height_and_width_expect_axis_aligned_axii:
                RigidBodyBuilder::default().id(0)
                    .position([7.0,-6.0,0.0])
                    .rotation(std::f32::consts::PI/4.0)
                    .body_type(RigidBodyType::Rectangle { width: 30., height: 10.})
                    .build(),
              [0.707, 0.707, 0.0], [-0.707, 0.707, 0.0]

        }
    }

    mod sat_project_on_axis {
        use super::super::sat_project_on_axis;
        use crate::engine::physics_engine::collision::rigid_body::{
            RigidBodyBuilder, RigidBodyType,
        };
        use crate::engine::util::fixed_float::fixed_float::FixedFloat;

        macro_rules! sat_project_on_axis_test {
            ($($name:ident: $body: expr, $axis: expr, $expected_min: expr, $expected_max: expr)*) => {
                $(
                    #[test]
                    fn $name() {
                        let body = $body;
                        let axis = $axis;
                        let expected_min = $expected_min;
                        let expected_max = $expected_max;
                        let projection  = sat_project_on_axis(&body, &axis);
                        let min_proj_ff: f32 = FixedFloat::from(projection.min).into();
                        let max_proj_ff: f32 = FixedFloat::from(projection.max).into();
                        assert_eq!(
                            expected_min, min_proj_ff,
                            "Expected projection minimum to be {expected_min} but found {min_proj_ff}"
                        );
                        assert_eq!(
                            expected_max, max_proj_ff,
                            "Expected projection maximum to be {expected_max} but found {max_proj_ff}"
                        );

                    }
                )*
            }
        }

        sat_project_on_axis_test! {
            given_rect_is_axis_aligned_and_not_offset_when_projected_onto_x:
                RigidBodyBuilder::default()
                .id(0)
                .position([0.0, 0.0, 0.0])
                .body_type(RigidBodyType::Rectangle {
                    width: 10.,
                    height: 10.,
                })
                .build(),[1.0, 0.0, 0.0], -5.0, 5.0

            given_rect_is_axis_aligned_and_not_offset_when_projected_onto_y:
                RigidBodyBuilder::default()
                .id(0)
                .position([0.0, 0.0, 0.0])
                .body_type(RigidBodyType::Rectangle {
                    width: 10.,
                    height: 10.,
                })
                .build(),[0.0, 1.0, 0.0], -5.0, 5.0

            given_rect_is_axis_aligned_and_offset_when_projected_onto_x:
                RigidBodyBuilder::default()
                .id(0)
                .position([5.0, -5.0, 0.0])
                .body_type(RigidBodyType::Rectangle {
                    width: 10.,
                    height: 10.,
                })
                .build(),[1.0, 0.0, 0.0], 0.0, 10.0

            given_rect_is_axis_aligned_and_offset_when_projected_onto_y:
                RigidBodyBuilder::default()
                .id(0)
                .position([5.0, -5.0, 0.0])
                .body_type(RigidBodyType::Rectangle {
                    width: 10.,
                    height: 10.,
                })
                .build(),[0.0, 1.0, 0.0], -10.0, 0.0

            given_rect_is_rotated_45_degrees_and_not_offset_when_projected_onto_x:
                RigidBodyBuilder::default()
                .id(0)
                .position([0.0, 0.0, 0.0])
                .rotation(std::f32::consts::PI/4.0)
                .body_type(RigidBodyType::Rectangle {
                    width: 10.,
                    height: 10.,
                })
                .build(),[1.0, 0.0, 0.0], -7.071, 7.071

            given_rect_is_rotated_90_degrees_and_not_offset_when_projected_onto_x:
                RigidBodyBuilder::default()
                .id(0)
                .position([0.0, 0.0, 0.0])
                .rotation(std::f32::consts::PI/2.0)
                .body_type(RigidBodyType::Rectangle {
                    width: 10.,
                    height: 10.,
                })
                .build(),[1.0, 0.0, 0.0], -5.0, 5.0

            given_rect_is_rotated_45_degrees_and_offset_when_projected_onto_x:
                RigidBodyBuilder::default()
                .id(0)
                .position([5.0, 5.0, 0.0])
                .rotation(std::f32::consts::PI/4.0)
                .body_type(RigidBodyType::Rectangle {
                    width: 10.,
                    height: 10.,
                })
                .build(),[1.0, 0.0, 0.0], -2.071, 12.071
        }
    }

    mod sat_projection_overlap {
        use super::super::sat_projection_overlap;
        use super::super::Projection;
        macro_rules! sat_projection_overlap {
            ($($name:ident: $proj_a: expr, $proj_b: expr, $expected: expr)*) => {
                $(
                    #[test]
                    fn $name() {
                        let proj_a = $proj_a;
                        let proj_b = $proj_b;
                        let expected = $expected;
                        let overlap = sat_projection_overlap(&proj_a, &proj_b);
                        assert_eq!(
                            expected, overlap,
                            "Expected projection overlap to be {expected} but found {overlap}"
                        );
                    }
                )*
            }
        }

        sat_projection_overlap! {
            given_projections_does_not_overlap_1:
                Projection::no_axis(-10.0, 10.0), Projection::no_axis(10.0, 20.0), false
            given_projections_does_not_overlap_2:
                Projection::no_axis(10.0, 20.0), Projection::no_axis(-10.0, 10.0), false
            given_projections_do_overlap_1:
                Projection::no_axis(10.0, 20.0), Projection::no_axis(-10.0, 11.0), true
            given_projections_are_contained_1:
                Projection::no_axis(-10.0, 10.0), Projection::no_axis(-10.0, 10.0), true
            given_projections_are_contained_2:
                Projection::no_axis(-10.0, 10.0), Projection::no_axis(-9.0, 9.0), true
        }
    }

    mod sat_overlap_distance {
        use super::super::sat_overlap_distance;
        use super::super::Projection;
        macro_rules! sat_overlap_distance_tests {
            ($($name:ident: $proj_a: expr, $proj_b: expr, $expected: expr)*) => {
                $(
                    #[test]
                    fn $name() {
                        let proj_a = $proj_a;
                        let proj_b = $proj_b;
                        let expected = $expected;
                        let overlap = sat_overlap_distance(&proj_a, &proj_b);
                        assert_eq!(
                            expected, overlap.distance,
                            "Expected projection overlap to be {expected} but found {overlap:?}"
                        );
                    }
                )*
            }
        }

        sat_overlap_distance_tests! {
            given_projections_does_not_overlap_1:
                Projection::no_axis(-10.0, 10.0), Projection::no_axis(10.0, 20.0), 0.0
            given_projections_does_not_overlap_2:
                Projection::no_axis(10.0, 20.0), Projection::no_axis(-10.0, 10.0), 0.0
            given_projections_do_overlap_1:
                Projection::no_axis(10.0, 20.0), Projection::no_axis(-10.0, 11.0), 1.0
            given_projections_are_contained_1:
                Projection::no_axis(-10.0, 10.0), Projection::no_axis(-10.0, 10.0), 20.0
            given_projections_are_contained_2:
                Projection::no_axis(-10.0, 10.0), Projection::no_axis(-9.0, 9.0), 18.0
        }
    }

    mod sat_collision_detection {
        use super::super::sat_collision_detection;
        use super::super::CollisionInformation;
        use crate::engine::physics_engine::collision::rigid_body::{
            RigidBodyBuilder, RigidBodyType,
        };
        use crate::engine::util::fixed_float::fixed_float::FixedFloat;
        use crate::engine::util::fixed_float::fixed_float_vector::FixedFloatVector;

        macro_rules! sat_collision_detection_tests {
            ($($name:ident: $body_a: expr, $body_b: expr, $expected: expr)*) => {
                $(
                    #[test]
                    fn $name() {
                        let expected: Option<CollisionInformation> = $expected;
                        let collision_info = sat_collision_detection(&$body_a, &$body_b);

                        match (expected, collision_info) {
                            (None, None) => (),
                            (None, Some(info)) =>  panic!("Expected result None but found {info:?}"),
                            (Some(e_info), None) => panic!("Expected result {e_info:?} but found None"),
                            (Some(e_collision_info), Some(collision_info)) => {
                                let penetration_depth_ff: f32 = FixedFloat::from(collision_info.penetration_depth).into();
                                let normal_ff: [f32;3] = FixedFloatVector::from(collision_info.normal).into();
                                let collision_point_ff: [f32;3] = FixedFloatVector::from(collision_info.collision_point).into();
                                assert_eq!(e_collision_info.penetration_depth,
                                    penetration_depth_ff,
                                    "Expected collision info {e_collision_info:?} but found {collision_info:?}");
                                assert_eq!(e_collision_info.normal,
                                    normal_ff,
                                    "Expected collision info {e_collision_info:?} but found {collision_info:?}");
                                assert_eq!(e_collision_info.collision_point,
                                    collision_point_ff,
                                    "Expected collision info {e_collision_info:?} but found {collision_info:?}");
                            },
                        }
                    }
                )*
            }
        }

        sat_collision_detection_tests! {
            given_rectangles_are_axis_aligned_when_do_not_collide_expect_no_collision:
                RigidBodyBuilder::default().id(0)
                    .body_type(RigidBodyType::Rectangle{ width: 20.0, height: 10.0 })
                    .position([-10.0,0.0,0.0])
                    .build(),
                RigidBodyBuilder::default().id(1)
                    .body_type(RigidBodyType::Rectangle{ width: 20.0, height: 10.0 })
                    .position([11.0,0.0,0.0])
                    .build(),
                None

            given_rectangles_are_axis_aligned_when_touch_on_y_axis_expect_no_collision:
                RigidBodyBuilder::default().id(0)
                    .body_type(RigidBodyType::Rectangle{ width: 20.0, height: 10.0 })
                    .position([-10.0,0.0,0.0])
                    .build(),
                RigidBodyBuilder::default().id(1)
                    .body_type(RigidBodyType::Rectangle{ width: 20.0, height: 10.0 })
                    .position([10.0,0.0,0.0])
                    .build(),
                None

            given_rectangles_are_axis_aligned_when_overlap_on_y_axis_expect_collision:
                RigidBodyBuilder::default().id(0)
                    .body_type(RigidBodyType::Rectangle{ width: 20.0, height: 10.0 })
                    .position([-10.0,0.0,0.0])
                    .build(),
                RigidBodyBuilder::default().id(1)
                    .body_type(RigidBodyType::Rectangle{ width: 20.0, height: 10.0 })
                    .position([9.0,0.0,0.0])
                    .build(),
                Some(CollisionInformation {
                    penetration_depth: 1.0,
                    normal: [1.0,0.0,0.0],
                    collision_point: [0.0,5.0,0.0]
                })

            given_rectangles_are_axis_aligned_when_overlap_on_y_axis_but_bodies_have_swapped_order_expect_collision:
                RigidBodyBuilder::default().id(1)
                    .body_type(RigidBodyType::Rectangle{ width: 20.0, height: 10.0 })
                    .position([9.0,0.0,0.0])
                    .build(),
                RigidBodyBuilder::default().id(0)
                    .body_type(RigidBodyType::Rectangle{ width: 20.0, height: 10.0 })
                    .position([-10.0,0.0,0.0])
                    .build(),
                Some(CollisionInformation {
                    penetration_depth: 1.0,
                    normal: [1.0,0.0,0.0],
                    collision_point: [0.0,5.0,0.0]
                })

            given_rectangles_are_axis_aligned_and_offset_when_overlapping_on_x_axis_expect_collision:
                RigidBodyBuilder::default().id(0)
                    .body_type(RigidBodyType::Rectangle{ width: 20.0, height: 10.0 })
                    .position([-10.0,20.0,0.0])
                    .build(),
                RigidBodyBuilder::default().id(1)
                    .body_type(RigidBodyType::Rectangle{ width: 20.0, height: 10.0 })
                    .position([-10.0,15.0,0.0])
                    .build(),
                Some(CollisionInformation {
                    penetration_depth: 5.0,
                    normal: [0.0,1.0,0.0],
                    collision_point: [0.0,20.0,0.0]
                })

            given_one_rectangle_is_axis_aligned_and_one_rotated_90_degrees_when_overlap_on_y_axis_expect_collision:
                RigidBodyBuilder::default().id(1)
                    .body_type(RigidBodyType::Rectangle{ width: 20.0, height: 10.0 })
                    .rotation(std::f32::consts::PI/2.0)
                    .position([10.0,0.0,0.0])
                    .build(),
                RigidBodyBuilder::default().id(0)
                    .body_type(RigidBodyType::Rectangle{ width: 20.0, height: 10.0 })
                    .position([0.0,0.0,0.0])
                    .build(),
                Some(CollisionInformation {
                    penetration_depth: 5.0,
                    normal: [-1.0,0.0,0.0],
                    collision_point: [5.0,5.0,0.0]
                })

            given_rectangles_are_rotated_45_degrees_when_their_sides_overlap_expect_collision:
                RigidBodyBuilder::default().id(0)
                    .body_type(RigidBodyType::Rectangle{ width: 10.0, height: 10.0 })
                    .rotation(std::f32::consts::PI/4.0)
                    .position([0.0,0.0,0.0])
                    .build(),
                RigidBodyBuilder::default().id(1)
                    .body_type(RigidBodyType::Rectangle{ width: 10.0, height: 10.0 })
                    .rotation(std::f32::consts::PI/4.0)
                    .position([6.071,6.071,0.0])
                    .build(),
                Some(CollisionInformation {
                    penetration_depth: 1.414,
                    normal: [0.707,0.707,0.0],
                    collision_point: [0.0,7.071,0.0]
                })

            given_rectangles_are_rotated_neg_45_degrees_when_their_sides_overlap_expect_collision:
                RigidBodyBuilder::default().id(0)
                    .body_type(RigidBodyType::Rectangle{ width: 10.0, height: 10.0 })
                    .rotation(std::f32::consts::PI/4.0)
                    .position([0.0,0.0,0.0])
                    .build(),
                RigidBodyBuilder::default().id(1)
                    .body_type(RigidBodyType::Rectangle{ width: 10.0, height: 10.0 })
                    .rotation(-std::f32::consts::PI/4.0)
                    .position([5.0,-5.0,0.0])
                    .build(),
                Some(CollisionInformation {
                    penetration_depth: 2.929,
                    normal: [-0.707,0.707,0.0],
                    collision_point: [5.0,2.071,0.0]
                })

            given_rectangles_are_rotated_neg_45_degrees_when_their_corners_overlap_expect_collision:
                RigidBodyBuilder::default().id(0)
                    .body_type(RigidBodyType::Rectangle{ width: 10.0, height: 10.0 })
                    .rotation(-std::f32::consts::PI/4.0)
                    .position([-5.0,0.0,0.0])
                    .build(),
                RigidBodyBuilder::default().id(1)
                    .body_type(RigidBodyType::Rectangle{ width: 10.0, height: 10.0 })
                    .rotation(std::f32::consts::PI/4.0)
                    .position([5.0,0.0,0.0])
                    .build(),
                Some(CollisionInformation {
                    penetration_depth: 2.929,
                    normal: [0.707, 0.707,0.0],
                    collision_point: [0.0,2.071,0.0]
                })
        }
    }
}
