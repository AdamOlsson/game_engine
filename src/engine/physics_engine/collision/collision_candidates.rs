
use core::fmt;
use std::fmt::Display;

use cgmath::{InnerSpace, Vector3};

#[derive(Debug)]
pub struct CollisionCandidates {
    pub indices: Vec<usize>
}

impl CollisionCandidates {
    pub fn new(indices: Vec<usize>) -> Self {
        Self {
            indices,
        }
    }
    pub fn len(&self) -> usize {
        self.indices.len()
    }
}



