use super::super::CollisionInformation;
use crate::engine::physics_engine::collision::rigid_body::{RigidBody, RigidBodyType};
use crate::engine::physics_engine::util::equations;

#[derive(Debug)]
struct Projection {
    pub min: f32,
    pub max: f32,
}

#[allow(dead_code)]
impl Projection {
    pub fn no_axis(min: f32, max: f32) -> Self {
        Self { min, max }
    }
}

#[derive(Debug)]
struct Overlap {
    pub distance: f32,
}

#[derive(Debug)]
struct CollisionEdge {
    pub start: [f32; 3],
    pub end: [f32; 3],
    pub max: [f32; 3],
    pub edge: [f32; 3],
}

#[derive(Debug)]
struct ClippedPoint {
    pub vertex: [f32; 3],
    pub depth: f32,
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
fn sat_project_on_axis(body: &RigidBody, axis: &[f32; 3]) -> Projection {
    let (min, max) = body
        .corners()
        .iter()
        .map(|c| equations::dot(axis, c))
        .fold((f32::MAX, f32::MIN), |(min, max), value| {
            (value.min(min), value.max(max))
        });

    Projection { min, max }
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
#[allow(dead_code)]
fn sat_projection_overlap(proj_a: &Projection, proj_b: &Projection) -> bool {
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
fn sat_overlap_distance(proj_a: &Projection, proj_b: &Projection) -> Overlap {
    let max = std::cmp::min_by(proj_a, proj_b, |a, b| a.max.total_cmp(&b.max));
    let min = std::cmp::max_by(proj_a, proj_b, |a, b| a.min.total_cmp(&b.min));
    let distance = max.max - min.min;
    Overlap { distance }
}

fn sat_find_collision_edge(body: &RigidBody, collision_axis: &[f32; 3]) -> CollisionEdge {
    let corners = body.corners();

    // Note that if two corners have the same value, the latter index is returned
    let (index, _) = corners
        .iter()
        .enumerate()
        .map(|(i, c)| (i, equations::dot(collision_axis, c)))
        .max_by(|(_, v0), (_, v1)| v0.partial_cmp(&v1).unwrap())
        .unwrap();

    let left_corner = &corners[(index as i32 - 1) as usize % corners.len()];
    let mid_corner = &corners[index];
    let right_corner = &corners[(index + 1) as usize % corners.len()];

    let mut mid_left = [
        mid_corner[0] - left_corner[0],
        mid_corner[1] - left_corner[1],
        mid_corner[2] - left_corner[2],
    ];

    let mut mid_right = [
        mid_corner[0] - right_corner[0],
        mid_corner[1] - right_corner[1],
        mid_corner[2] - right_corner[2],
    ];

    equations::normalize(&mut mid_left);
    equations::normalize(&mut mid_right);

    if equations::dot(&mid_right, collision_axis) <= equations::dot(&mid_left, collision_axis) {
        CollisionEdge {
            max: mid_corner.clone(),
            start: *right_corner,
            end: *mid_corner,
            edge: equations::subtract(&mid_corner, &right_corner),
        }
    } else {
        CollisionEdge {
            max: mid_corner.clone(),
            start: *mid_corner,
            end: *left_corner,
            edge: equations::subtract(&left_corner, &mid_corner),
        }
    }
}

/// Computes the clipping points between two rectangles during a collision,
/// given the collision normal, using the Separating Axis Theorem (SAT).
///
/// # Parameters
/// - `body_a`: A reference to the first `RigidBody` involved in the collision.
/// - `body_b`: A reference to the second `RigidBody` involved in the collision.
/// - `collision_normal`: A 3D vector representing the collision axis or normal.
///
/// # Returns
/// - A `Vec<ClippedPoint>` containing the clipped points of contact between the two
///   rectangles. Each `ClippedPoint` contains:
///   - `depth`: The penetration depth of the point relative to the collision normal.
///   - `vertex`: The position of the point in world space.
/// - An empty vector if no valid clipped points are found.
///
/// # Details
/// This function determines the points of contact during a collision by using
/// edge clipping based on SAT principles. It selects reference and incident edges
/// from the two rectangles and clips them to find the intersection region.
///
/// ## Steps
/// 1. **Find Collision Edges:**
///    - For each body, the edge most aligned with the collision normal is identified
///      using `sat_find_collision_edge`.
/// 2. **Determine Reference and Incident Edges:**
///    - The reference edge is the edge most perpendicular to the collision normal.
///    - The incident edge belongs to the other rectangle and is clipped against the
///      reference edge.
/// 3. **Clipping:**
///    - The incident edge is clipped using the reference edge's line equations,
///      yielding a pair of clipped points that lie within the bounds of the reference edge.
///    - Additional clipping ensures the points are within the bounds of the reference edge.
/// 4. **Filter and Map Clipped Points:**
///    - The final clipped points are filtered to remove those outside the bounds
///      of the collision region. Their penetration depth relative to the collision
///      normal is calculated and returned.
///
/// ## Notes
/// - The function assumes the collision normal is correctly oriented and provides
///   no guarantees about the direction of the edges relative to specific rectangles.
/// - A counterclockwise rotation convention is used for edge normals.
/// - Clipping points are influenced by edge ordering, so care is required in ensuring
///   consistent edge orientation during the collision detection process.
/// - This function follows the approach outlined in *Dyn4j's Contact Points Using Clipping*
///   [https://dyn4j.org/2011/11/contact-points-using-clipping/].
///
/// # Usage
/// This function is typically called as part of a broader SAT-based collision response
/// system to compute accurate contact points and penetration depths, which are essential
/// for resolving collisions and applying corrective impulses.
fn sat_find_clipping_points(
    body_a: &RigidBody,
    body_b: &RigidBody,
    collision_normal: &[f32; 3],
) -> Vec<ClippedPoint> {
    let edge_a = sat_find_collision_edge(&body_a, &collision_normal);
    let edge_b = sat_find_collision_edge(&body_b, &equations::negate(&collision_normal));

    let (mut reference_edge, incident_edge, _flip) =
        if equations::dot(&edge_a.edge, &collision_normal).abs()
            <= equations::dot(&edge_b.edge, &collision_normal).abs()
        {
            (edge_a, edge_b, false)
        } else {
            (edge_b, edge_a, true)
        };
    equations::normalize(&mut reference_edge.edge);

    let offset_1 = equations::dot(&reference_edge.edge, &reference_edge.start);
    let clipped_points = sat_clip(
        &incident_edge.start,
        &incident_edge.end,
        &reference_edge.edge,
        offset_1,
    );

    if clipped_points.len() < 2 {
        return vec![];
    }

    let offset_2 = equations::dot(&reference_edge.edge, &reference_edge.end);

    let clipped_points = sat_clip(
        &clipped_points[0],
        &clipped_points[1],
        &equations::negate(&reference_edge.edge),
        -offset_2,
    );

    if clipped_points.len() < 2 {
        return vec![];
    }

    // NOTE: Negating of the reference edges normal caused unwanted behavior. However
    // not negating and always rotating the edge counterclockwise to get the normal
    // seem to work. This could be a point of bugs in the future.
    // If flip, negate the normal.
    // https://dyn4j.org/2011/11/contact-points-using-clipping/
    let reference_edge_norm = equations::perpendicular_2d(&reference_edge.edge);

    let max = equations::dot(&reference_edge_norm, &reference_edge.max);

    return clipped_points
        .into_iter()
        .filter(|point| equations::dot(&reference_edge_norm, point) - max >= 0.0)
        .map(|point| ClippedPoint {
            depth: equations::dot(&reference_edge_norm, &point) - max,
            vertex: point,
        })
        .collect();
}

/// Clips a line segment against a plane defined by a normal vector and an offset,
/// using the Separating Axis Theorem (SAT) for collision detection.
///
/// # Parameters
/// - `v1`: A 3D vector representing the starting vertex of the line segment.
/// - `v2`: A 3D vector representing the ending vertex of the line segment.
/// - `normal`: A 3D vector representing the normal of the clipping plane.
/// - `offset`: A scalar value representing the plane's offset from the origin
///   along its normal.
///
/// # Returns
/// - A `Vec<[f32; 3]>` containing up to two points:
///   - The vertices of the segment that lie inside or on the plane.
///   - The intersection point of the segment with the plane, if it intersects.
///
/// # Details
/// The function determines which points of a line segment defined by `v1` and `v2`
/// lie within or on the positive side of a plane defined by `normal` and `offset`.
/// If the segment crosses the plane, the intersection point is also included.
///
/// ## Steps
/// 1. Compute the signed distances (`d1` and `d2`) of the segment's endpoints
///    (`v1` and `v2`) from the plane, using the formula:
///    `d = dot(normal, vertex) - offset`.
/// 2. Depending on the signs of `d1` and `d2`:
///    - If a point lies on or outside the plane (`d >= 0`), it is added to the result.
///    - If the segment crosses the plane (`d1 * d2 < 0`), the intersection point is
///      calculated using linear interpolation:
///      - Compute the interpolation factor `u = d1 / (d1 - d2)`.
///      - Find the intersection point using `v1 + u * (v2 - v1)`.
/// 3. Return the resulting points in a vector.
///
/// ## Usage
/// This function is typically used in SAT-based collision detection to clip edges
/// of one shape against the edges or boundaries of another. The clipped points
/// define the region of overlap or contact.
///
/// ## Notes
/// - The function works in 3D but is generally used for 2D physics simulations
///   where the Z-coordinate is ignored.
/// - The returned points are ordered based on their position along the input segment.
/// - A maximum of two points is returned: the retained vertices and/or the intersection.
///
/// ## Edge Cases
/// - If both points lie outside the plane, the result is an empty vector.
/// - If both points lie inside the plane, both are included in the result.
fn sat_clip(v1: &[f32; 3], v2: &[f32; 3], normal: &[f32; 3], offset: f32) -> Vec<[f32; 3]> {
    let mut cp = vec![];

    let d1 = equations::dot(&normal, &v1) - offset;
    let d2 = equations::dot(&normal, &v2) - offset;

    if d1 >= 0.0 {
        cp.push(v1.clone());
    }

    if d2 >= 0.0 {
        cp.push(v2.clone());
    }

    if d1 * d2 < 0.0 {
        let mut e = equations::subtract(&v2, &v1);
        let u = d1 / (d1 - d2);
        equations::multiply_in_place(&mut e, u);
        equations::add_in_place(&mut e, &v1);
        cp.push(e);
    }

    cp
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

    // TODO: Whenever we project a body onto an axis and its body is axis aligned to
    // the axis, we select which points cause the projection based on its definition order.
    // This is wrong as we then can select the opposite corner from the collision. Instead,
    // if two points of an object cause the same projection point, we want to select the
    // point closest to the body for which the projection axis originates from.

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

    let (axii, index, _overlap) = std::cmp::min_by(
        (axii_a, index_axii_a, min_overlap_on_a),
        (axii_b, index_axii_b, min_overlap_on_b),
        |a, b| a.2.distance.total_cmp(&b.2.distance),
    );

    let axis = axii[index];

    // Correct the direction of the collision normal such it always points from body A
    // to body B
    let direction = body_b.position - body_a.position;
    let collision_normal = if equations::dot(&axis, &direction.into()) >= 0.0 {
        axis
    } else {
        [-axis[0], -axis[1], -axis[2]]
    };

    let clipping_points = sat_find_clipping_points(&body_a, &body_b, &collision_normal);

    // Note: For now I only return one averaged collision point as there is no need to
    // return and edge.
    let clipping_point = clipping_points
        .iter()
        .max_by(|a, b| a.depth.total_cmp(&b.depth));

    match clipping_point {
        None => None,
        Some(cp) => Some(CollisionInformation {
            penetration_depth: cp.depth,
            normal: collision_normal,
            collision_point: cp.vertex,
        }),
    }
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
            given_rect_with_no_rotation_expect_axis_aligned_axii_origo:
                RigidBodyBuilder::default().id(0)
                    .position([0.0,0.0,0.0])
                    .body_type(RigidBodyType::Rectangle { width: 10., height: 10.})
                    .build(),
              [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]

            given_rect_with_no_rotation_expect_axis_aligned_axii_top_right_quarter:
                RigidBodyBuilder::default().id(0)
                    .position([10.0,7.0,0.0])
                    .body_type(RigidBodyType::Rectangle { width: 10., height: 10.})
                    .build(),
              [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]

            given_rect_with_no_rotation_expect_axis_aligned_axii_bot_right_quarter:
                RigidBodyBuilder::default().id(0)
                    .position([10.0,-7.0,0.0])
                    .body_type(RigidBodyType::Rectangle { width: 10., height: 10.})
                    .build(),
              [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]

            given_rect_with_no_rotation_expect_axis_aligned_axii_bot_left_quarter:
                RigidBodyBuilder::default().id(0)
                    .position([-10.0,-7.0,0.0])
                    .body_type(RigidBodyType::Rectangle { width: 10., height: 10.})
                    .build(),
              [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]

            given_rect_with_no_rotation_expect_axis_aligned_axii_top_left_quarter:
                RigidBodyBuilder::default().id(0)
                    .position([-10.0,7.0,0.0])
                    .body_type(RigidBodyType::Rectangle { width: 10., height: 10.})
                    .build(),
              [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]

            given_rect_is_offset_from_origo_with_no_rotation_expect_axis_aligned_axii:
                RigidBodyBuilder::default().id(0)
                    .position([7.0,-6.0,0.0])
                    .body_type(RigidBodyType::Rectangle { width: 10., height: 10.})
                    .build(),
              [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]

            given_rect_is_offset_from_origo_with_45_degree_rotation_expect_axis_aligned_axii:
                RigidBodyBuilder::default().id(0)
                    .position([7.0,-6.0,0.0])
                    .rotation(std::f32::consts::PI/4.0)
                    .body_type(RigidBodyType::Rectangle { width: 10., height: 10.})
                    .build(),
              [0.707, 0.707, 0.0], [-0.707, 0.707, 0.0]
            given_rect_is_offset_from_origo_with_45_degree_rotation_and_uneven_height_and_width_expect_axis_aligned_axii:
                RigidBodyBuilder::default().id(0)
                    .position([7.0,-6.0,0.0])
                    .rotation(std::f32::consts::PI/4.0)
                    .body_type(RigidBodyType::Rectangle { width: 30., height: 10.})
                    .build(),
              [0.707, 0.707, 0.0], [-0.707, 0.707, 0.0]

        }
    }

    mod sat_project_on_axis {
        use super::super::{sat_project_on_axis, Projection};
        use crate::engine::physics_engine::collision::rigid_body::{
            RigidBodyBuilder, RigidBodyType,
        };
        use crate::engine::util::fixed_float::fixed_float::FixedFloat;

        macro_rules! sat_project_on_axis_test {
            ($($name:ident: $body: expr, $axis: expr, $expected_proj: expr )*) => {
                $(
                    #[test]
                    fn $name() {
                        let body = $body;
                        let axis = $axis;
                        let expected_proj= $expected_proj;
                        let projection  = sat_project_on_axis(&body, &axis);
                        let min_proj_ff: f32 = FixedFloat::from(projection.min).into();
                        let max_proj_ff: f32 = FixedFloat::from(projection.max).into();
                        assert_eq!(
                            expected_proj.min, min_proj_ff,
                            "Expected projection to be {expected_proj:?} but found {projection:?}"
                        );
                        assert_eq!(
                            expected_proj.max, max_proj_ff,
                            "Expected projection to be {expected_proj:?} but found {projection:?}"
                        );
                    }
                )*
            }
        }

        sat_project_on_axis_test! {
            given_rect_is_axis_aligned_and_not_offset_from_origo_when_projected_onto_x:
                RigidBodyBuilder::default()
                .id(0)
                .position([0.0, 0.0, 0.0])
                .body_type(RigidBodyType::Rectangle {
                    width: 10.,
                    height: 10.,
                })
                .build(),[1.0, 0.0, 0.0],
                Projection {min: -5.0, max: 5.0 }

            given_rect_is_axis_aligned_and_not_offset_from_origo_when_projected_onto_y:
                RigidBodyBuilder::default()
                .id(0)
                .position([0.0, 0.0, 0.0])
                .body_type(RigidBodyType::Rectangle {
                    width: 10.,
                    height: 10.,
                })
                .build(),[0.0, 1.0, 0.0],
                Projection {min: -5.0, max: 5.0 }

            given_rect_is_axis_aligned_and_offset_from_origo_when_projected_onto_x:
                RigidBodyBuilder::default()
                .id(0)
                .position([5.0, -5.0, 0.0])
                .body_type(RigidBodyType::Rectangle {
                    width: 10.,
                    height: 10.,
                })
                .build(),[1.0, 0.0, 0.0],
                Projection {min: 0.0, max: 10.0 }

            given_rect_is_axis_aligned_and_offset_from_origo_when_projected_onto_y:
                RigidBodyBuilder::default()
                .id(0)
                .position([5.0, -5.0, 0.0])
                .body_type(RigidBodyType::Rectangle {
                    width: 10.,
                    height: 10.,
                })
                .build(),[0.0, 1.0, 0.0],
                Projection {min: -10.0, max: 0.0 }

            given_rect_is_rotated_45_degrees_and_not_offset_from_origo_when_projected_onto_x:
                RigidBodyBuilder::default()
                .id(0)
                .position([0.0, 0.0, 0.0])
                .rotation(std::f32::consts::PI/4.0)
                .body_type(RigidBodyType::Rectangle {
                    width: 10.,
                    height: 10.,
                })
                .build(),[1.0, 0.0, 0.0],
                Projection {min: -7.071, max: 7.071 }

            given_rect_is_rotated_90_degrees_and_not_offset_from_origo_when_projected_onto_x:
                RigidBodyBuilder::default()
                .id(0)
                .position([0.0, 0.0, 0.0])
                .rotation(std::f32::consts::PI/2.0)
                .body_type(RigidBodyType::Rectangle {
                    width: 10.,
                    height: 10.,
                })
                .build(),[1.0, 0.0, 0.0],
                Projection {min: -5.0, max: 5.0 }

            given_rect_is_rotated_45_degrees_and_offset_from_origo_when_projected_onto_x:
                RigidBodyBuilder::default()
                .id(0)
                .position([5.0, 5.0, 0.0])
                .rotation(std::f32::consts::PI/4.0)
                .body_type(RigidBodyType::Rectangle {
                    width: 10.,
                    height: 10.,
                })
                .build(),[1.0, 0.0, 0.0],
                Projection {min: -2.071, max: 12.071 }
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
        use super::super::{Overlap, Projection};
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
                            expected.distance, overlap.distance,
                            "Expected projection overlap to be {expected:?} but found {overlap:?}"
                        );
                    }
                )*
            }
        }

        sat_overlap_distance_tests! {
            given_projections_does_not_overlap_1:
                Projection::no_axis(-10.0, 10.0), Projection::no_axis(10.0, 20.0),
                Overlap { distance: 0.0 }

            given_projections_does_not_overlap_2:
                Projection::no_axis(10.0, 20.0), Projection::no_axis(-10.0, 10.0),
                Overlap { distance: 0.0 }

            given_projections_do_overlap_1:
                Projection { min:10.0, max:20.0 },
                Projection { min:-9.0, max:11.0 },
                Overlap { distance: 1.0 }

            given_projections_do_overlap_2:
                Projection { min:-9.0, max:11.0 },
                Projection { min:10.0, max:20.0 },
                Overlap { distance: 1.0 }

            given_projections_do_overlap_3:
                Projection { min:-9.0, max:11.0 },
                Projection { min:10.0, max:20.0},
                Overlap { distance: 1.0 }

            given_projections_are_contained_1:
                Projection::no_axis(-10.0, 10.0), Projection::no_axis(-10.0, 10.0),
                Overlap { distance: 20.0}

            given_projections_are_contained_2:
                Projection::no_axis(-10.0, 10.0), Projection::no_axis(-9.0, 9.0),
                Overlap { distance: 18.0 }
        }
    }

    mod sat_find_collision_edge {
        use super::super::{sat_find_collision_edge, CollisionEdge};
        use crate::engine::physics_engine::collision::rigid_body::{
            RigidBodyBuilder, RigidBodyType,
        };
        use crate::engine::physics_engine::util::equations;

        macro_rules! sat_find_collision_edge_tests {
            ($($name:ident: $body: expr, $axis: expr, $expected: expr)*) => {
                $(
                    #[test]
                    fn $name() {
                        let expected_edge = $expected;
                        let body = $body;
                        let collision_axis = $axis;
                        let collision_edge = sat_find_collision_edge(&body, &collision_axis);
                        assert_eq!(
                            expected_edge.start, collision_edge.start,
                            "Expected collision edge {expected_edge:?} but found {collision_edge:?}");
                        assert_eq!(
                            expected_edge.end, collision_edge.end,
                            "Expected collision edge {expected_edge:?} but found {collision_edge:?}");
                        assert_eq!(
                            expected_edge.edge, collision_edge.edge,
                            "Expected collision edge {expected_edge:?} but found {collision_edge:?}");
                        assert_eq!(
                            expected_edge.max, collision_edge.max,
                            "Expected collision edge {expected_edge:?} but found {collision_edge:?}");

                    }
                )*
            }
        }

        sat_find_collision_edge_tests! {
            //https://dyn4j.org/2011/11/contact-points-using-clipping/
            given_example_1_body_a_at_dyn4j:
                RigidBodyBuilder::default().id(0)
                    .position([11.0,6.5,0.0])
                    .body_type(RigidBodyType::Rectangle{ width: 6.0, height: 5.0})
                    .build(),
                [0.0,-1.0,0.0],
                CollisionEdge {
                    max: [8.0, 4.0, 0.0],
                    start: [8.0, 4.0, 0.0],
                    end: [14.0, 4.0, 0.0],
                    edge: [6.0, 0.0,0.0],
                }

            //https://dyn4j.org/2011/11/contact-points-using-clipping/
            given_example_1_body_b_at_dyn4j:
                RigidBodyBuilder::default().id(1)
                    .position([8.0,3.5,0.0])
                    .body_type(RigidBodyType::Rectangle{ width: 8.0, height: 3.0})
                    .build(),
                equations::negate(&[0.0,-1.0,0.0]),
                CollisionEdge {
                    max: [12.0, 5.0, 0.0],
                    start: [12.0, 5.0, 0.0],
                    end: [4.0, 5.0, 0.0],
                    edge: [-8.0, 0.0, 0.0],
                }

            //https://dyn4j.org/2011/11/contact-points-using-clipping/
            given_example_2_body_a_at_dyn4j:
                RigidBodyBuilder::default().id(0)
                    .position([5.5,7.5,0.0])
                    .rotation(-std::f32::consts::PI/4.0)
                    .body_type(RigidBodyType::Rectangle{ width: 5.6568, height: 4.2426})
                    .build(),
                [0.0,-1.0,0.0],
                CollisionEdge {
                    max: [6.0, 4.0, 0.0],
                    start: [2.0, 8.0, 0.0],
                    end: [6.0, 4.0, 0.0],
                    edge: [4.0, -4.0, 0.0],
                }

            //https://dyn4j.org/2011/11/contact-points-using-clipping/
            given_example_2_body_b_at_dyn4j:
                RigidBodyBuilder::default().id(1)
                    .position([8.0,3.5,0.0])
                    .body_type(RigidBodyType::Rectangle{ width: 8.0, height: 3.0})
                    .build(),
                equations::negate(&[0.0,-1.0,0.0]),
                CollisionEdge {
                    max: [12.0, 5.0, 0.0],
                    start: [12.0, 5.0, 0.0],
                    end: [4.0, 5.0, 0.0],
                    edge: [-8.0, 0.0, 0.0],
                }

            //https://dyn4j.org/2011/11/contact-points-using-clipping/
            given_example_3_body_a_at_dyn4j:
                RigidBodyBuilder::default().id(0)
                    .position([11.5,5.5,0.0])
                    .rotation(-0.2449)
                    .body_type(RigidBodyType::Rectangle{ width: 4.1231, height: 4.1231})
                    .build(),
                [-0.19,-0.98,0.0],
                // Note that examples max is start, not end because of the ambiguity
                // when two corners have equal project length. Then the order of definition
                // for the points matter.
                CollisionEdge {
                    max: [13.0, 3.0, 0.0],
                    start: [9.0, 4.0, 0.0],
                    end: [13.0, 3.0, 0.0],
                    edge: [4.0, -1.0, 0.0],
                }

            //https://dyn4j.org/2011/11/contact-points-using-clipping/
            given_example_3_body_b_at_dyn4j:
                RigidBodyBuilder::default().id(0)
                    .position([8.0,3.5,0.0])
                    .body_type(RigidBodyType::Rectangle{ width: 8.0, height: 3.0})
                    .build(),
                equations::negate(&[-0.19,-0.98,0.0]),
                CollisionEdge {
                    max: [12.0, 5.0, 0.0],
                    start: [12.0, 5.0, 0.0],
                    end: [4.0, 5.0, 0.0],
                    edge: [-8.0, 0.0, 0.0],
                }

        }
    }

    mod sat_find_clipping_points {
        use super::super::{sat_find_clipping_points, ClippedPoint};
        use crate::engine::physics_engine::collision::rigid_body::{
            RigidBodyBuilder, RigidBodyType,
        };
        use crate::engine::util::fixed_float::fixed_float::FixedFloat;

        macro_rules! sat_find_clipping_points_tests{
            ($($name:ident: $body_a: expr, $body_b: expr, $normal: expr, $expected: expr)*) => {
                $(
                    #[test]
                    fn $name() {
                        let expected = $expected;
                        let clipped_points = sat_find_clipping_points(&$body_a, &$body_b, &$normal);
                        assert_eq!(expected.len(), clipped_points.len(),
                            "Expected {} clipped points but found {}", expected.len(), clipped_points.len());
                        std::iter::zip(expected, clipped_points)
                            .enumerate()
                            .for_each(|(i,(e_cp, cp))| {
                                let ff_depth: f32 = FixedFloat::from(cp.depth).into();
                                assert_eq!(e_cp.vertex, cp.vertex,
                                    "At index {i}, expected vertex {:?} but found {:?}", e_cp.vertex, cp.vertex);
                                assert_eq!(e_cp.depth, ff_depth,
                                    "At index {i}, expected depth {:?} but found {:?}", e_cp.depth, ff_depth);
                            });
                    }
                )*
            }
        }

        // Note: clipping points will point from deepest point out
        sat_find_clipping_points_tests! {
            //https://dyn4j.org/2011/11/contact-points-using-clipping/
            given_example_1_at_dyn4j:
                RigidBodyBuilder::default().id(0)
                    .position([11.0,6.5,0.0])
                    .body_type(RigidBodyType::Rectangle{ width: 6.0, height: 5.0})
                    .build(),
                RigidBodyBuilder::default().id(1)
                    .position([8.0,3.5,0.0])
                    .body_type(RigidBodyType::Rectangle{ width: 8.0, height: 3.0})
                    .build(),
                [0.0,-1.0,0.0],
                vec![
                    ClippedPoint { vertex: [12.0,5.0,0.0], depth: 1.0},
                    ClippedPoint { vertex: [8.0,5.0,0.0], depth: 1.0}]

            given_bodies_overlap_when_collision_axis_is_down:
                RigidBodyBuilder::default().id(0)
                    .position([10.0,10.0,0.0])
                    .body_type(RigidBodyType::Rectangle{ width: 6.0, height: 6.0})
                    .build(),
                RigidBodyBuilder::default().id(1)
                    .position([13.0,6.0,0.0])
                    .body_type(RigidBodyType::Rectangle{ width: 8.0, height: 4.0})
                    .build(),
                [0.0,-1.0,0.0],
                vec![
                    ClippedPoint { vertex: [9.0,8.0,0.0], depth: 1.0},
                    ClippedPoint { vertex: [13.0,8.0,0.0], depth: 1.0}]

            ////https://dyn4j.org/2011/11/contact-points-using-clipping/
            given_example_2_at_dyn4j:
                RigidBodyBuilder::default().id(0)
                    .position([5.5,7.5,0.0])
                    .rotation(-std::f32::consts::PI/4.0)
                    .body_type(RigidBodyType::Rectangle{ width: 5.6568, height: 4.2426})
                    .build(),
                RigidBodyBuilder::default().id(1)
                    .position([8.0,3.5,0.0])
                    .body_type(RigidBodyType::Rectangle{ width: 8.0, height: 3.0})
                    .build(),
                [0.0,-1.0,0.0],
                vec![
                    ClippedPoint { vertex: [6.0,4.0,0.0], depth: 1.0}]

            ////https://dyn4j.org/2011/11/contact-points-using-clipping/
            given_example_3_at_dyn4j:
                RigidBodyBuilder::default().id(0)
                    .position([11.5,5.5,0.0])
                    .rotation(-0.2449)
                    .body_type(RigidBodyType::Rectangle{ width: 4.1231, height: 4.1231})
                    .build(),
                RigidBodyBuilder::default().id(1)
                    .position([8.0,3.5,0.0])
                    .body_type(RigidBodyType::Rectangle{ width: 8.0, height: 3.0})
                    .build(),
                [-0.19,-0.98,0.0],
                // The expected value differ slightly due to what I believe is premature
                // rounding in the example
                vec![
                    ClippedPoint { vertex: [12.0,5.0,0.0], depth: 1.698},
                    ClippedPoint { vertex: [9.25,5.0,0.0], depth: 1.031}]
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
                    collision_point: [-1.0,-5.0,0.0]
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
                    normal: [-1.0,0.0,0.0],
                    collision_point: [0.0,5.0,0.0]
                })

            given_rectangles_are_axis_aligned_and_offset_from_origo_when_overlapping_on_x_axis_expect_collision:
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
                    normal: [0.0,-1.0,0.0],
                    collision_point: [-20.0,20.0,0.0]
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
                    collision_point: [10.0,5.0,0.0]
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
                    collision_point: [6.071,-1.0,0.0]
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
                    normal: [0.707,-0.707,0.0],
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
                    collision_point: [0.0,-2.071,0.0]
                })

            given_rectangles_are_offset_from_each_other_with_no_rotation_with_half_overlap_expect_collision:
                RigidBodyBuilder::default().id(0)
                    .body_type(RigidBodyType::Rectangle{ width: 10.0, height: 10.0 })
                    .position([-4.0, 2.5, 0.0])
                    .build(),
                RigidBodyBuilder::default().id(1)
                    .body_type(RigidBodyType::Rectangle{ width: 10.0, height: 10.0 })
                    .position([4.0, -2.5, 0.0])
                    .build(),
                Some(CollisionInformation {
                    penetration_depth: 2.0,
                    normal: [1.0,0.0,0.0],
                    collision_point: [-1.0,-2.5,0.0]
                })

            given_rectangles_are_offset_from_each_other_with_no_rotation_with_half_overlap_expect_collision_2:
                RigidBodyBuilder::default().id(0)
                    .body_type(RigidBodyType::Rectangle{ width: 10.0, height: 10.0 })
                    .position([4.0, 2.5, 0.0])
                    .build(),
                RigidBodyBuilder::default().id(1)
                    .body_type(RigidBodyType::Rectangle{ width: 10.0, height: 10.0 })
                    .position([-4.0, -2.5, 0.0])
                    .build(),
                Some(CollisionInformation {
                    penetration_depth: 2.0,
                    normal: [-1.0,0.0,0.0],
                    collision_point: [1.0,-2.5,0.0]
                })

        }
    }
}
