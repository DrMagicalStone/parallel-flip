use std::ops::{Deref, DerefMut, Index, IndexMut};

use crate::graph::{combinatorial::edge::Edge, vertex::Vertex};

pub struct VertexCollection<'a> {
    vertex_array: &'a [Vertex],
}

impl<'a> Deref for VertexCollection<'a> {
    type Target = [Vertex];

    fn deref(&self) -> &Self::Target {
        &self.vertex_array
    }
}

impl<'a> Index<usize> for VertexCollection<'a> {
    type Output = Vertex;

    fn index(&self, index: usize) -> &Self::Output {
        &self.vertex_array[index]
    }
}

impl<'a> VertexCollection<'a> {
    pub fn new(vertex_array: &'a [Vertex]) -> Self {
        Self { vertex_array }
    }

    pub fn new_edge(&self, vertex_index_pair: (usize, usize)) -> Edge {
        Edge::new(vertex_index_pair, &self.vertex_array)
    }
}
