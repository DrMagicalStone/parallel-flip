use std::fmt::{Debug, Display};

use crate::graph::{combinatorial::edge::Edge, vertex::Vertex};

pub struct Line<'h> {
    param_a: u32,
    param_b: u32,
    param_c: u32,
    point_array: &'h [Vertex],
}

impl<'h> Display for Line<'h> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({} x + {} y + {} = 0)",
            self.param_a, self.param_b, self.param_c
        )
    }
}

impl<'h> Debug for Line<'h> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Line")
            .field("param_a", &self.param_a)
            .field("param_b", &self.param_b)
            .field("param_c", &self.param_c)
            .field("point_array", &self.point_array)
            .finish()
    }
}

impl<'h> PartialEq for Line<'h> {
    fn eq(&self, other: &Self) -> bool {
        self.param_a == other.param_a
            && self.param_b == other.param_b
            && self.param_c == other.param_c
            && std::ptr::eq(self.point_array, other.point_array)
    }
}

impl<'h> Eq for Line<'h> {}

impl<'h> Line<'h> {
    pub(in crate::graph) fn new(edge_from: &'h Edge<'h>) -> Line<'h> {
        let point_index_pair = edge_from.get_point_index_pair();
        let point_array = edge_from.get_point_array();
        let (xa, ya) = &point_array[point_index_pair.0].get();
        let (xb, yb) = &point_array[point_index_pair.1].get();
        Line {
            param_a: ya - yb,
            param_b: xb - xa,
            param_c: xa * yb - xb * ya,
            point_array,
        }
    }

    pub fn get_side_discriminant(&self, vertex: &Vertex) -> u32 {
        let (x, y) = vertex.get();

        self.param_a * x + self.param_b * y + self.param_c
    }

    pub fn edge_intersect(&self, edge: &Edge) -> bool {
        let (a, b) = (
            self.point_array[edge.get_point_index_pair().0],
            self.point_array[edge.get_point_index_pair().1],
        );
        self.get_side_discriminant(&a) * self.get_side_discriminant(&b) < 0
    }
}
