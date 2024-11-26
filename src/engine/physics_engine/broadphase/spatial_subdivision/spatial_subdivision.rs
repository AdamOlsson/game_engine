use std::collections::{HashMap, HashSet};

use cgmath::{InnerSpace, MetricSpace, Vector3};
use rayon::iter::{
    IndexedParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator,
};

use crate::engine::physics_engine::collision::collision_candidates::CollisionCandidates;
use crate::engine::physics_engine::collision::rigid_body::{RigidBody, RigidBodyType};

use super::super::BroadPhase;
use super::cell_id::{CellId, CellIdType};
use super::object_id::ObjectId;

#[derive(Debug)]
struct BoundingCircle {
    pub center: Vector3<f32>,
    pub radius: f32,
}

impl std::fmt::Display for BoundingCircle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BoundingCircle{{ center: {:?}, radius: {} }}",
            self.center, self.radius
        )
    }
}

pub struct SpatialSubdivision {}

const CONTROL_BIT_BOUNDING_VOLUME_1: u8 = 0b0000_0001;
const CONTROL_BIT_BOUNDING_VOLUME_2: u8 = 0b0000_0010;
const CONTROL_BIT_BOUNDING_VOLUME_3: u8 = 0b0000_0100;
const CONTROL_BIT_BOUNDING_VOLUME_4: u8 = 0b0000_1000;
const CONTROL_BIT_HOME_CELL_1: u8 = 0b0000_0000;
const CONTROL_BIT_HOME_CELL_2: u8 = 0b0001_0000;
const CONTROL_BIT_HOME_CELL_3: u8 = 0b0010_0000;
const CONTROL_BIT_HOME_CELL_4: u8 = 0b0011_0000;
const BOUNDING_VOLUME_MASK: u8 = 0b0000_1111;
const HOME_CELL_MASK: u8 = 0b1111_0000;

impl SpatialSubdivision {
    pub fn new() -> Self {
        Self {}
    }

    fn get_control_bit_for_home_cell_id(cell_id: (u32, u32, u32)) -> u8 {
        let x_mod = cell_id.0 % 2;
        let y_mod = cell_id.1 % 2;
        match (x_mod, y_mod) {
            (0, 0) => CONTROL_BIT_HOME_CELL_1, // Top-left cell
            (1, 0) => CONTROL_BIT_HOME_CELL_2, // Top-right cell
            (0, 1) => CONTROL_BIT_HOME_CELL_3, // Bottom-left cell
            (1, 1) => CONTROL_BIT_HOME_CELL_4, // Bottom-right cell
            _ => unreachable!("Unknown home cell"),
        }
    }

    fn get_control_bit_for_bounding_volume_cell_id(cell_id: (u32, u32, u32)) -> u8 {
        let x_mod = cell_id.0 % 2;
        let y_mod = cell_id.1 % 2;
        match (x_mod, y_mod) {
            (0, 0) => CONTROL_BIT_BOUNDING_VOLUME_1, // Top-left cell
            (1, 0) => CONTROL_BIT_BOUNDING_VOLUME_2, // Top-right cell
            (0, 1) => CONTROL_BIT_BOUNDING_VOLUME_3, // Bottom-left cell
            (1, 1) => CONTROL_BIT_BOUNDING_VOLUME_4, // Bottom-right cell
            _ => unreachable!("Unknown bounding volume cell"),
        }
    }

    /// Create the cell object for a given bounding sphere
    fn create_cell_object(
        bcircle: &BoundingCircle,
        cell_width: f32,
        object_id: usize,
    ) -> (ObjectId, Vec<CellId>) {
        let x = bcircle.center.x;
        let y = bcircle.center.y;
        let radius = bcircle.radius;
        debug_assert!(x >= 0.0, "Expected x to be 0 or more, found {x}");
        debug_assert!(y >= 0.0, "Expected y to be 0 or more, found {y}");

        let x_norm = x / cell_width;
        let y_norm = y / cell_width;
        let r_norm = radius / cell_width;
        let xy_norm = Vector3::new(x_norm, y_norm, 0.0);

        // Global cell mean cell number in entire grid
        let home_cell_x = x_norm.floor() as u32;
        let home_cell_y = y_norm.floor() as u32;
        let home_cell_id = CellId::new((home_cell_x, home_cell_y, 0), CellIdType::Home, object_id);
        let mut control_bits = Self::get_control_bit_for_home_cell_id(home_cell_id.cell_id);
        control_bits |= Self::get_control_bit_for_bounding_volume_cell_id(home_cell_id.cell_id);

        // Determine which quad of its cell the center belongs to
        let quad_x = x_norm - x_norm.floor();
        let quad_y = y_norm - y_norm.floor();

        debug_assert!(
            {
                let pred =
                    (home_cell_x == 0 && r_norm > quad_x) || (home_cell_y == 0 && r_norm > quad_y);
                if pred {
                    eprintln!("Expected that the object always be offset such it can't overlap the left or top cell if the home cell x or y value is 0.");
                    eprintln!("The offending object has id {object_id}");
                    eprintln!("Bounding Circle: {bcircle}");
                    eprintln!("cell_width: {cell_width}");
                    eprintln!("x_norm: {x_norm}, y_norm: {y_norm}, r_norm: {r_norm}");
                    eprintln!("quad_x: {quad_x}, quad_y: {quad_y}");
                    eprintln!("home_cell_x: {home_cell_x}, home_cell_y: {home_cell_y}")
                }
                !pred
            },
            "Object spaning over negative values cell x-values, see below for more detail"
        );

        // Once we have determined the quad, we only need to check for overlap on 3
        // cells, sides and diagonal
        let mut cell_ids = vec![home_cell_id];
        match (quad_x < 0.5, quad_y < 0.5) {
            (true, true) => {
                // top left
                // Overlap check left cell
                if quad_x - r_norm < 0.0 {
                    let cell_id = (home_cell_x - 1, home_cell_y, 0);
                    let phantom = CellId::new(cell_id, CellIdType::Phantom, object_id);
                    control_bits |=
                        Self::get_control_bit_for_bounding_volume_cell_id(phantom.cell_id);
                    cell_ids.push(phantom);
                }

                // Overlap check top cell
                if quad_y - r_norm < 0.0 {
                    let cell_id = (home_cell_x, home_cell_y - 1, 0);
                    let phantom = CellId::new(cell_id, CellIdType::Phantom, object_id);
                    control_bits |=
                        Self::get_control_bit_for_bounding_volume_cell_id(phantom.cell_id);
                    cell_ids.push(phantom);
                }

                // Overlap check with the top left cell
                let home_cell_top_left_corner = Vector3::new(x_norm.floor(), y_norm.floor(), 0.0);
                if home_cell_top_left_corner.distance2(xy_norm) < r_norm.powi(2) {
                    let cell_id = (home_cell_x - 1, home_cell_y - 1, 0);
                    let phantom = CellId::new(cell_id, CellIdType::Phantom, object_id);
                    control_bits |=
                        Self::get_control_bit_for_bounding_volume_cell_id(phantom.cell_id);
                    cell_ids.push(phantom);
                }
            }
            (false, true) => {
                // top right
                // Overlap check right cell
                if quad_x + r_norm > 1.0 {
                    let cell_id = (home_cell_x + 1, home_cell_y, 0);
                    let phantom = CellId::new(cell_id, CellIdType::Phantom, object_id);
                    control_bits |=
                        Self::get_control_bit_for_bounding_volume_cell_id(phantom.cell_id);
                    cell_ids.push(phantom);
                }

                // Overlap check top cell
                if quad_y - r_norm < 0.0 {
                    let cell_id = (home_cell_x, home_cell_y - 1, 0);
                    let phantom = CellId::new(cell_id, CellIdType::Phantom, object_id);
                    control_bits |=
                        Self::get_control_bit_for_bounding_volume_cell_id(phantom.cell_id);
                    cell_ids.push(phantom);
                }

                // Overlap check top right cell
                let home_cell_top_right_corner =
                    Vector3::new(x_norm.floor() + 1.0, y_norm.floor(), 0.0);
                if home_cell_top_right_corner.distance2(xy_norm) < r_norm.powi(2) {
                    let cell_id = (home_cell_x + 1, home_cell_y - 1, 0);
                    let phantom = CellId::new(cell_id, CellIdType::Phantom, object_id);
                    control_bits |=
                        Self::get_control_bit_for_bounding_volume_cell_id(phantom.cell_id);
                    cell_ids.push(phantom);
                }
            }
            (true, false) => {
                // bottom left
                // Overlap check left cell
                if quad_x - r_norm < 0.0 {
                    let cell_id = (home_cell_x - 1, home_cell_y, 0);
                    let phantom = CellId::new(cell_id, CellIdType::Phantom, object_id);
                    control_bits |=
                        Self::get_control_bit_for_bounding_volume_cell_id(phantom.cell_id);
                    cell_ids.push(phantom);
                }

                // Overlap check bottom cell
                if quad_y + r_norm > 1.0 {
                    let cell_id = (home_cell_x, home_cell_y + 1, 0);
                    let phantom = CellId::new(cell_id, CellIdType::Phantom, object_id);
                    control_bits |=
                        Self::get_control_bit_for_bounding_volume_cell_id(phantom.cell_id);
                    cell_ids.push(phantom);
                }

                // Overlap check bottom left cell
                let home_cell_bottom_left_corner =
                    Vector3::new(x_norm.floor(), y_norm.floor() + 1.0, 0.0);
                if home_cell_bottom_left_corner.distance2(xy_norm) < r_norm.powi(2) {
                    let cell_id = (home_cell_x - 1, home_cell_y + 1, 0);
                    let phantom = CellId::new(cell_id, CellIdType::Phantom, object_id);
                    control_bits |=
                        Self::get_control_bit_for_bounding_volume_cell_id(phantom.cell_id);
                    cell_ids.push(phantom);
                }
            }
            (false, false) => {
                // bottom right
                // Overlap check right cell
                if quad_x + r_norm > 1.0 {
                    let cell_id = (home_cell_x + 1, home_cell_y, 0);
                    let phantom = CellId::new(cell_id, CellIdType::Phantom, object_id);
                    control_bits |=
                        Self::get_control_bit_for_bounding_volume_cell_id(phantom.cell_id);
                    cell_ids.push(phantom);
                }

                // Overlap check bottom cell
                if quad_y + r_norm > 1.0 {
                    let cell_id = (home_cell_x, home_cell_y + 1, 0);
                    let phantom = CellId::new(cell_id, CellIdType::Phantom, object_id);
                    control_bits |=
                        Self::get_control_bit_for_bounding_volume_cell_id(phantom.cell_id);
                    cell_ids.push(phantom);
                }

                // Overlap check bottom right cell
                let home_cell_bottom_right_corner =
                    Vector3::new(x_norm.floor() + 1.0, y_norm.floor() + 1.0, 0.0);
                if home_cell_bottom_right_corner.distance2(xy_norm) < r_norm.powi(2) {
                    let cell_id = (home_cell_x + 1, home_cell_y + 1, 0);
                    let phantom = CellId::new(cell_id, CellIdType::Phantom, object_id);
                    control_bits |=
                        Self::get_control_bit_for_bounding_volume_cell_id(phantom.cell_id);
                    cell_ids.push(phantom);
                }
            }
        }

        let object_id = ObjectId { control_bits };
        return (object_id, cell_ids);
    }

    fn cumsum(l: &[&CellId]) -> Vec<(u32, u32)> {
        let last_index = l.len() as u32 - 1;
        let (_, _, _, sum) = l.iter().fold(
            (0, 0, 0_u32, vec![]),
            |(i, prev_cell_id, count, mut acc), object| {
                let is_last = i == last_index;
                let transition = prev_cell_id != Self::hash(object.cell_id);
                if transition {
                    acc.push((i - count, count));
                    if is_last {
                        acc.push((i - 1, 1));
                    }
                    return (i + 1, Self::hash(object.cell_id), 1, acc);
                } else if is_last {
                    acc.push((i - count, count + 1));
                }
                return (i + 1, Self::hash(object.cell_id), count + 1, acc);
            },
        );
        return sum;
    }

    fn hash(cell_id: (u32, u32, u32)) -> u32 {
        cell_id.0 + cell_id.1 * 1_000 + cell_id.2 * 1_000_000
    }

    fn can_we_skip_collision_test(t: u8, object_id_a: &ObjectId, object_id_b: &ObjectId) -> bool {
        let home_cell_id_a = (object_id_a.control_bits & HOME_CELL_MASK) >> 4;
        let home_cell_id_b = (object_id_b.control_bits & HOME_CELL_MASK) >> 4;
        debug_assert!(
            home_cell_id_a < 4,
            "Expected home cell id to be less than 4 but found {home_cell_id_a}"
        );
        debug_assert!(
            home_cell_id_b < 4,
            "Expected home cell id to be less than 4 but found {home_cell_id_b}"
        );

        let home_cell_id_type_a: u8 = 1 << home_cell_id_a;
        let home_cell_id_type_b: u8 = 1 << home_cell_id_b;

        let bounding_volume_cell_a = object_id_a.control_bits & BOUNDING_VOLUME_MASK;
        let bounding_volume_cell_b = object_id_b.control_bits & BOUNDING_VOLUME_MASK;
        let common_bounding_volume_cells = bounding_volume_cell_a & bounding_volume_cell_b;

        let home_cell_a_among_common_cells =
            (home_cell_id_type_a & common_bounding_volume_cells) > 0;
        let home_cell_b_among_common_cells =
            (home_cell_id_type_b & common_bounding_volume_cells) > 0;

        let pred_a = (home_cell_id_a + 1) < t && home_cell_a_among_common_cells;
        let pred_b = (home_cell_id_b + 1) < t && home_cell_b_among_common_cells;

        return pred_a || pred_b;
    }
}

impl BroadPhase<[Vec<CollisionCandidates>; 4]> for SpatialSubdivision {
    fn collision_detection<'a, I>(&self, bodies: I) -> [Vec<CollisionCandidates>; 4]
    where
        I: Iterator<Item = &'a RigidBody>,
    {
        let bodies: Vec<&RigidBody> = bodies.collect();
        let (mut bcircles, largest_radius, min_x, min_y) = bodies
            .par_iter()
            .filter_map(|b| match b.body_type {
                RigidBodyType::Circle { radius } => {
                    let radius = radius * 1.41;
                    Some((
                        BoundingCircle {
                            center: b.position,
                            radius,
                        },
                        radius,
                        b.position.x,
                        b.position.y,
                    ))
                }
                RigidBodyType::Rectangle { width, height } => {
                    let radius = Vector3::new(width / 2.0, height / 2.0, 0.0).magnitude() * 1.41;
                    //let radius = (width.max(height) / 2.0)*1.41;
                    Some((
                        BoundingCircle {
                            center: b.position,
                            radius,
                        },
                        radius,
                        b.position.x,
                        b.position.y,
                    ))
                }
                _ => panic!("Unkown body type {}", b.body_type),
            })
            .fold(
                || (Vec::new(), 0.0_f32, f32::MAX, f32::MAX),
                // TODO: No need to open up radius, x and y. Can only use circle
                |mut acc, (circle, radius, x, y)| {
                    acc.0.push(circle);
                    acc.1 = acc.1.max(radius);
                    acc.2 = acc.2.min(x - radius);
                    acc.3 = acc.3.min(y - radius);
                    acc
                },
            )
            .reduce(
                || (Vec::new(), 0.0, f32::MAX, f32::MAX),
                |mut acc1, mut acc2| {
                    acc1.0.append(&mut acc2.0);
                    acc1.1 = acc1.1.max(acc2.1);
                    acc1.2 = acc1.2.min(acc2.2);
                    acc1.3 = acc1.3.min(acc2.3);
                    acc1
                },
            );

        // Handle floating point errors by rounding the offset to the larger or smaller number
        let offset = Vector3::new(min_x.floor(), min_y.floor(), 0.0);
        bcircles.par_iter_mut().for_each(|b| {
            b.center -= offset;
        });

        debug_assert!(
            {
                let bad_bodies: Vec<(usize, &BoundingCircle, &&RigidBody)> = bcircles
                    .iter()
                    .enumerate()
                    .filter(|(_, b)| (b.center.x - b.radius) < 0.0 || (b.center.y - b.radius) < 0.0)
                    .map(|(i, b)| (i, b, &bodies[i]))
                    .collect();
                let pred = bad_bodies.len() != 0;
                if pred {
                    eprintln!("Offset: {offset:?}");
                    eprintln!("Offending bodies:");
                    bad_bodies
                        .iter()
                        .for_each(|(i, bc, cb)| eprintln!("ID: {i}, {bc}, {cb}"));
                }
                !pred
            },
            "Expected all objects to only overlap the positive x and y axis"
        );

        let cell_width = largest_radius * 2.0 * 1.5;
        let (object_id_array, cell_id_array_nested): (Vec<ObjectId>, Vec<Vec<CellId>>) = bcircles
            .par_iter()
            .enumerate()
            .map(|(i, b)| Self::create_cell_object(&b, cell_width, i))
            .unzip();

        let mut cell_id_array: Vec<&CellId> = cell_id_array_nested.par_iter().flatten().collect();

        // Sort the by cell id and emphasize x,y then z and Home over Phantom.
        cell_id_array.sort_by(|a, b| {
            let cell_a = Self::hash(a.cell_id);
            let cell_b = Self::hash(b.cell_id);
            if cell_a == cell_b {
                a.cell_object_type.cmp(&b.cell_object_type)
            } else {
                cell_a.cmp(&cell_b)
            }
        });

        let mut pass1 = vec![];
        let mut pass2 = vec![];
        let mut pass3 = vec![];
        let mut pass4 = vec![];
        let cell_index = Self::cumsum(&cell_id_array);
        cell_index
            .iter()
            .filter(|(_, count)| *count > 1)
            .map(|(index, count)| {
                let start = *index as usize;
                let end = start + *count as usize;
                &cell_id_array[start..end]
            })
            .map(|slice| {
                debug_assert!(
                    {
                        // Verify that all cell objects belong to same cell
                        let cell_id = Self::hash(slice[0].cell_id);
                        slice.iter().fold(true, |acc, cell| {
                            (Self::hash(cell.cell_id) == cell_id) && acc
                        })
                    },
                    "Expected all objects in slice to belong to the same home cell type: {slice:?}"
                );
                let mut collision_set = HashSet::new();
                let pass_num = Self::get_control_bit_for_bounding_volume_cell_id(slice[0].cell_id);
                for i in 0..slice.len() {
                    let cell_id_a = slice[i];
                    let object_id_a = &object_id_array[cell_id_a.object_id];
                    for j in (i + 1)..slice.len() {
                        let cell_id_b = slice[j];
                        let object_id_b = &object_id_array[cell_id_b.object_id];
                        match Self::can_we_skip_collision_test(pass_num, &object_id_a, object_id_b)
                        {
                            true => (),
                            false => {
                                collision_set.insert(cell_id_a.object_id);
                                collision_set.insert(cell_id_b.object_id);
                            }
                        }
                    }
                }
                let collision_list: Vec<usize> = collision_set.into_iter().collect();
                return (pass_num, collision_list);
            })
            .filter(|(_, collisions)| collisions.len() > 0)
            .for_each(|(pass_num, collisions)| match pass_num {
                CONTROL_BIT_BOUNDING_VOLUME_1 => pass1.push(CollisionCandidates::new(collisions)),
                CONTROL_BIT_BOUNDING_VOLUME_2 => pass2.push(CollisionCandidates::new(collisions)),
                CONTROL_BIT_BOUNDING_VOLUME_3 => pass3.push(CollisionCandidates::new(collisions)),
                CONTROL_BIT_BOUNDING_VOLUME_4 => pass4.push(CollisionCandidates::new(collisions)),
                _ => unreachable!("Pass number should always be 1,2,4 or 8"),
            });

        debug_assert!(
            pass1
                .iter()
                .map(|cc| cc.indices.len() > 0)
                .fold(true, |acc, b| acc && b),
            "Expected all entries in pass 1 to have non-zero lengths: {pass1:?}"
        );
        debug_assert!(
            pass2
                .iter()
                .map(|cc| cc.indices.len() > 0)
                .fold(true, |acc, b| acc && b),
            "Expected all entries in pass 2 to have non-zero lengths: {pass2:?}"
        );
        debug_assert!(
            pass3
                .iter()
                .map(|cc| cc.indices.len() > 0)
                .fold(true, |acc, b| acc && b),
            "Expected all entries in pass 3 to have non-zero lengths: {pass3:?}"
        );
        debug_assert!(
            pass4
                .iter()
                .map(|cc| cc.indices.len() > 0)
                .fold(true, |acc, b| acc && b),
            "Expected all entries in pass 4 to have non-zero lengths: {pass4:?}"
        );

        debug_assert!(
            assert_object_id_in_candidate_list_exists_in_no_other_candidate_list(&pass1),
            "Expected each object id to appear at most once within the same pass(1):\n{pass1:?}"
        );
        debug_assert!(
            assert_object_id_in_candidate_list_exists_in_no_other_candidate_list(&pass2),
            "Expected each object id to appear at most once within the same pass(2):\n{pass2:?}"
        );
        debug_assert!(
            assert_object_id_in_candidate_list_exists_in_no_other_candidate_list(&pass3),
            "Expected each object id to appear at most once within the same pass(3):\n{pass3:?}"
        );
        debug_assert!(
            assert_object_id_in_candidate_list_exists_in_no_other_candidate_list(&pass3),
            "Expected each object id to appear at most once within the same pass(4):\n{pass4:?}"
        );

        return [pass1, pass2, pass3, pass4];
    }
}

fn assert_object_id_in_candidate_list_exists_in_no_other_candidate_list(
    pass: &Vec<CollisionCandidates>,
) -> bool {
    let mut index_counts: HashMap<usize, usize> = HashMap::new();
    for candidates in pass.iter() {
        for &index in &candidates.indices {
            *index_counts.entry(index).or_insert(0) += 1;
        }
    }
    let count: Vec<usize> = index_counts
        .into_iter()
        .filter_map(|(index, count)| if count != 1 { Some(index) } else { None })
        .collect();
    count.len() == 0
}

#[cfg(test)]
mod tests {

    #[allow(non_snake_case)]
    mod compute_overlapping_cell_types {
        use super::super::BoundingCircle;
        use super::super::CellId;
        use super::super::CellIdType;
        use super::super::SpatialSubdivision;
        use cgmath::Vector3;
        macro_rules! create_cell_object_tests {
            ($($name:ident: $xy:expr, $r:expr, $cell_width: expr, $expected_output:expr)*) => {
                $(
                    #[test]
                    fn $name() {
                        let (x,y) = $xy;
                        let expected_output: Vec<CellId> = $expected_output;
                        let bcircle = BoundingCircle { center: Vector3::new(x,y,0.0), radius: $r};
                        let (_object_id, cell_ids) = SpatialSubdivision::create_cell_object(&bcircle, $cell_width, 0);

                        assert_eq!(expected_output.len(), cell_ids.len(), "Expected output length {} ({expected_output:?}) but found {} ({cell_ids:?})", expected_output.len(), cell_ids.len());
                        assert_eq!(cell_ids[0].cell_object_type, CellIdType::Home, "Expected the first object to be home cell but is phantom cell");
                        expected_output.iter().for_each(|o| assert!(cell_ids.contains(o), "Expected to find {o:?} in output {cell_ids:?} but didn't"));
                        let home_cells: Vec<CellId> = cell_ids.into_iter().filter(|o| o.cell_object_type == CellIdType::Home).collect();
                        assert_eq!(home_cells.len(), 1);
                    }
                )*
            }
        }

        create_cell_object_tests! {
            given_cell_id_1_0_0_when_center_is_top_left_quad_of_cell_expect_overlap_with_left:
                (0.11,0.025), 0.015, 0.1, vec![
                    CellId::new((1,0,0), CellIdType::Home, 0),
                    CellId::new((0,0,0), CellIdType::Phantom, 0),]
            given_cell_id_0_1_0_when_center_is_top_left_quad_of_cell_expect_overlap_with_top:
                (0.025,0.11), 0.015, 0.1, vec![
                    CellId::new((0,1,0), CellIdType::Home, 0),
                    CellId::new((0,0,0), CellIdType::Phantom, 0),]
            given_cell_id_1_1_0_when_center_is_top_left_quad_of_cell_expect_overlap_with_top_and_left:
                (0.11,0.11), 0.0141, 0.1, vec![
                    CellId::new((1,1,0), CellIdType::Home, 0),
                    CellId::new((1,0,0), CellIdType::Phantom, 0),
                    CellId::new((0,1,0), CellIdType::Phantom, 0),]
            given_cell_id_1_1_0_when_center_is_top_left_quad_of_cell_expect_overlap_with_left_top_and_topleft:
                (0.11,0.11), 0.02, 0.1, vec![
                    CellId::new((1,1,0), CellIdType::Home, 0),
                    CellId::new((0,0,0), CellIdType::Phantom, 0),
                    CellId::new((1,0,0), CellIdType::Phantom, 0),
                    CellId::new((0,1,0), CellIdType::Phantom, 0),]

            given_object_in_top_right_quad_of_cell_when_object_overlap_with_right_expect_overlap_with_right:
                (0.39,0.025), 0.02, 0.1, vec![
                    CellId::new((3,0,0), CellIdType::Home, 0),
                    CellId::new((4,0,0), CellIdType::Phantom, 0),]
            given_object_in_top_right_quad_of_cell_when_object_overlap_with_top_expect_overlap_with_top:
                (0.375,0.11), 0.02, 0.1, vec![
                    CellId::new((3,1,0), CellIdType::Home, 0),
                    CellId::new((3,0,0), CellIdType::Phantom, 0),]
            given_object_in_top_right_quad_of_cell_when_object_overlap_with_top_left_and_topleft_expect_overlap_with_top_left_topleft:
                (0.39,0.11), 0.02, 0.1, vec![
                    CellId::new((3,1,0), CellIdType::Home, 0),
                    CellId::new((3,0,0), CellIdType::Phantom, 0),
                    CellId::new((4,0,0), CellIdType::Phantom, 0),
                    CellId::new((4,1,0), CellIdType::Phantom, 0),]

            given_object_in_bottom_left_quad_of_cell_when_object_overlap_with_left_expect_overlap_with_left:
                (0.11,0.125), 0.02, 0.1, vec![
                    CellId::new((1,1,0), CellIdType::Home, 0),
                    CellId::new((0,1,0), CellIdType::Phantom, 0),]
            given_object_in_bottom_left_quad_of_cell_when_object_overlap_with_bottom_expect_overlap_with_bottom:
                (0.125,0.19), 0.02, 0.1, vec![
                    CellId::new((1,1,0), CellIdType::Home, 0),
                    CellId::new((1,2,0), CellIdType::Phantom, 0),]
            given_object_in_bottom_left_quad_of_cell_when_object_overlap_with_left_bottom_and_bottomleft_expect_overlap_with_left_bottom_and_bottomleft:
                (0.11,0.19), 0.02, 0.1, vec![
                    CellId::new((1,1,0), CellIdType::Home, 0),
                    CellId::new((0,1,0), CellIdType::Phantom, 0),
                    CellId::new((0,2,0), CellIdType::Phantom, 0),
                    CellId::new((1,2,0), CellIdType::Phantom, 0),]

            given_object_in_bottom_right_quad_of_cell_when_object_overlap_with_right_expect_overlap_with_right:
                (0.19,0.175), 0.02, 0.1, vec![
                    CellId::new((1,1,0), CellIdType::Home, 0),
                    CellId::new((2,1,0), CellIdType::Phantom, 0),]
            given_object_in_bottom_right_quad_of_cell_when_object_overlap_with_bottom_expect_overlap_with_bottom:
                (0.175,0.19), 0.02, 0.1, vec![
                    CellId::new((1,1,0), CellIdType::Home, 0),
                    CellId::new((1,2,0), CellIdType::Phantom, 0),]
            given_object_in_bottom_right_quad_of_cell_when_object_overlap_with_right_bottom_and_bottomright_expect_overlap_with_left_bottom_and_bottomright:
                (0.19,0.19), 0.02, 0.1, vec![
                    CellId::new((1,1,0), CellIdType::Home, 0),
                    CellId::new((2,1,0), CellIdType::Phantom, 0),
                    CellId::new((2,2,0), CellIdType::Phantom, 0),
                    CellId::new((1,2,0), CellIdType::Phantom, 0),]
        }
    }

    #[allow(non_snake_case)]
    mod cumsum {
        use super::super::CellId;
        use super::super::CellIdType;
        use super::super::SpatialSubdivision;
        macro_rules! cum_sum_tests {
            ($($name:ident: $cell_id_array: expr, $expected_output:expr)*) => {
                $(
                    #[test]
                    fn $name() {
                        let expected_output: Vec<(u32,u32)> = $expected_output;
                        let cell_id_array = $cell_id_array;
                        let cell_id_array_: Vec<&CellId> = cell_id_array.iter().collect();
                        let cumsum = SpatialSubdivision::cumsum(&cell_id_array_);
                        assert_eq!(expected_output, cumsum, "Expected output {expected_output:?} but found {cumsum:?}");
                    }
                )*
            }
        }

        cum_sum_tests! {
            test_transitions_: vec![
                CellId::new((0,0,0), CellIdType::Home, 0),
                CellId::new((0,0,0), CellIdType::Phantom, 1),
                CellId::new((1,0,0), CellIdType::Home, 1),
                CellId::new((1,0,0), CellIdType::Phantom, 0),
            ],vec![(0,2), (2,2)]

            test_last_object_is_included_: vec![
                CellId::new((0,0,0), CellIdType::Home, 0),
                CellId::new((0,0,0), CellIdType::Phantom, 1),
                CellId::new((1,0,0), CellIdType::Home, 1),
                CellId::new((1,0,0), CellIdType::Phantom, 0),
                CellId::new((0,1,0), CellIdType::Phantom, 0),
            ],vec![(0,2), (2,2), (3,1)]

            test_3_objects_: vec![
                CellId::new((0,0,0), CellIdType::Home, 2),
                CellId::new((1,0,0), CellIdType::Home, 1),
                CellId::new((1,0,0), CellIdType::Phantom, 0),
                CellId::new((0,1,0), CellIdType::Phantom, 0),
                CellId::new((1,1,0), CellIdType::Home, 0),
                CellId::new((1,1,0), CellIdType::Phantom, 1),
            ],vec![(0,1), (1,2), (3,1),(4,2)]
        }
    }

    mod can_we_skip_collision_test {
        use super::super::ObjectId;
        use super::super::SpatialSubdivision;
        macro_rules! can_we_skip_collision_test_tests {
            ($($name:ident: $t: expr, $object_id_a: expr, $object_id_b: expr, $expected_output: expr)*) => {
                $(
                    #[test]
                    fn $name() {
                        assert_eq!(SpatialSubdivision::can_we_skip_collision_test($t, &$object_id_a, &$object_id_b),
                            $expected_output,
                            "Expected {} but found {}", $expected_output, !$expected_output);
                    }
                )*
            }
        }

        can_we_skip_collision_test_tests! {
            given_two_objects_with_different_home_cells_but_share_all_bounding_cells_during_pass_1_expect_false:
                1, ObjectId { control_bits: 0b0010_0101 }, ObjectId { control_bits: 0b0000_0101 }, false
            given_two_objects_with_different_home_cells_but_share_all_bounding_cells_during_pass_3_expect_true:
                3, ObjectId { control_bits: 0b0010_0101 }, ObjectId { control_bits: 0b0000_0101 }, true

            given_two_objects_with_different_home_cells_but_share_bounding_cell_types_during_pass_3_expect_false:
                3, ObjectId { control_bits: 0b0011_1010 }, ObjectId { control_bits: 0b0010_1010 }, false
            given_two_objects_with_different_home_cells_but_share_bounding_cell_types_during_pass_4_expect_true:
                4, ObjectId { control_bits: 0b0011_1100 }, ObjectId { control_bits: 0b0010_1100 }, true

            given_two_objects_with_different_home_cells_but_share_subset_of_cell_types_during_pass_1_expect_false:
                1, ObjectId { control_bits: 0b0010_0101 }, ObjectId { control_bits: 0b0000_0001 }, false
            given_two_objects_with_different_home_cells_but_share_subset_of_cell_types_during_pass_3_expect_true:
                3, ObjectId { control_bits: 0b0010_0101 }, ObjectId { control_bits: 0b0000_0001 }, true

            given_two_objects_with_different_home_cells_and_do_not_have_home_cells_among_commong_cells_expect_false:
                1, ObjectId { control_bits: 0b0001_0011 }, ObjectId { control_bits: 0b0010_0101 }, false

        }
    }
}
