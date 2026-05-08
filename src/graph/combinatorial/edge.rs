use crate::graph::geometric::line::Line;
use crate::graph::vertex::Vertex;

pub struct Edge<'h> {
    vertex_array: &'h [Vertex],

    vertex_index_pair: (usize, usize),
}

impl<'h> PartialEq for Edge<'h> {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.vertex_array, other.vertex_array)
            && self.vertex_index_pair == other.vertex_index_pair
    }
}

impl<'h> Eq for Edge<'h> {}

impl<'h> Edge<'h> {
    pub(in crate::graph) fn new(
        vertex_index_pair: (usize, usize),
        vertex_array: &'h [Vertex],
    ) -> Self {
        Self {
            vertex_index_pair,
            vertex_array,
        }
    }

    pub fn get_point_array(&self) -> &[Vertex] {
        self.vertex_array
    }

    pub fn get_point_index_pair(&self) -> &(usize, usize) {
        &self.vertex_index_pair
    }
}
