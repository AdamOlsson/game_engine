use cgmath::Vector3;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::engine::physics_engine::collision::{collision_body::{CollisionBody, CollisionBodyType}, collision_candidates::CollisionCandidates};

use super::BroadPhase;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum CellType {
    Home, 
    Phantom, 
}

struct Cell {
    pub cell_type: CellType,
    pub cell_id: usize,
    pub object_id: usize,
}

#[derive(Clone)]
struct CollisionCell {
    pub start: usize,
    pub h_occupants: usize,
    pub p_occupants: usize,
}

struct BoundingCircle {
    pub center: Vector3<f32>,
    pub radius: f32,
}

pub struct SpatialSubdivision {
    window_size: (f32, f32),
}


impl SpatialSubdivision {
    pub fn new(window_size: (f32, f32)) -> Self {
        Self { window_size }
    }

    fn assign_cells(bcircle: &BoundingCircle, cell_width: f32) -> Vec<Cell> {
        // TODO: Given an objects bounding circle, assign home cell and which cells this object
        // overlaps
        vec![]
    }

    fn construct_cell_id_array(bcircles: &Vec<BoundingCircle>, cell_width: f32) -> Vec<Cell> {
        let nested_cell_id_array: Vec<Cell> = bcircles.par_iter()
            .map(|b| Self::assign_cells(&b, cell_width))
            .flatten()
            .collect();
        return nested_cell_id_array;
    }

    fn count_occupants(objects: &[Cell], num_cells: usize) -> Vec<CollisionCell> {
        let mut cell_counts: Vec<CollisionCell> = vec![
            CollisionCell { start: 0, h_occupants: 0, p_occupants: 0 }; num_cells];

        objects.iter().for_each(|obj| {
            let cell = &mut cell_counts[obj.cell_id];
            match obj.cell_type {
                CellType::Home => cell.h_occupants += 1,
                CellType::Phantom => cell.p_occupants += 1,
            }
        });

        cell_counts
    }

}

impl BroadPhase for SpatialSubdivision {
    fn collision_detection(&self, bodies: &Vec<CollisionBody>) -> Vec<CollisionCandidates> {
        let bcircles: Vec<BoundingCircle> = bodies.par_iter().filter(|b| match b.body_type {
            CollisionBodyType::Circle { .. } => true,
            _ => false,
        })
        .map(|b| match b.body_type {
            CollisionBodyType::Circle { radius } => BoundingCircle{ center: b.position , radius: radius*1.41 },
            _ => panic!(),
        })
        .collect(); 
      
        let largest_radius = bcircles.iter().fold(0.0 as f32,|acc, b| b.radius.max(acc)); 
        let cell_width = largest_radius*1.5;
        let grid_width = (self.window_size.0 / cell_width).ceil() as usize;
        let grid_height = (self.window_size.1 / cell_width).ceil() as usize;
        let num_cells = grid_width*grid_height;

        let mut cell_id_array = Self::construct_cell_id_array(&bcircles, cell_width);
        cell_id_array.sort_by(
            |c1,c2| if c1.cell_id == c2.cell_id {
                c1.cell_type.cmp(&c2.cell_type)
            } else {
                c1.cell_id.cmp(&c2.cell_id)
            });

        let cell_occupant_count = Self::count_occupants(&cell_id_array, num_cells); 
        panic!();

        return vec![];
    }
}
