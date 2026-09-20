mod flip;
mod new;

use std::ops::{Add, Sub};

use typed_index_collections::TiVec;

use crate::graph::{graph_entry::GraphEntry, quadrilateral::Quadrilateral, shared_triangle_relation::SharedTriangleEntry};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UIndex(pub u32);

/// See graph.rs in src for more info.
#[derive(PartialEq, Eq, Hash)]
pub struct Graph {
    data: TiVec<UIndex, GraphEntry>,
    numbers_of_flippable: UIndex,
}

impl Clone for Graph {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            numbers_of_flippable: self.numbers_of_flippable,
        }
    }
}

impl From<usize> for UIndex {
    fn from(value: usize) -> Self {
        UIndex(value as u32)
    }
}

impl Into<usize> for UIndex {
    fn into(self) -> usize {
        self.0 as usize
    }
}

impl Add<u32> for UIndex {
    type Output = Self;

    fn add(self, rhs: u32) -> Self::Output {
        UIndex(self.0 + rhs)
    }
}

impl Sub<u32> for UIndex {
    type Output = Self;

    fn sub(self, rhs: u32) -> Self::Output {
        UIndex(self.0 - rhs)
    }
}