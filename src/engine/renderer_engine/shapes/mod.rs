use super::vertex::Vertex;

pub mod triangle;
pub mod circle;

pub trait Shape {
    fn id() -> String;
    fn compute_vertices() -> Vec<Vertex>;
    fn compute_indices() -> Vec<u16>;
}
