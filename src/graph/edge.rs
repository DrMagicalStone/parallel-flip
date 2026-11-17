use typed_index_collections::TiSlice;

use crate::graph::{
    indexes::{IndexedEdge, VertexIndex},
    vec2c::{Vec2C, cross},
    vertex::{Coord, Vertex},
};

/// Usually used for intersection test
#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Edge {
    vertexes: [Vertex; 2],
}

impl Edge {
    pub fn new(vertexes: [Vertex; 2]) -> Edge {
        Edge { vertexes }
    }

    pub fn as_vec2c(&self) -> Vec2C {
        self.vertexes[1] - self.vertexes[0]
    }

    pub fn get_vertexes(&self) -> &[Vertex; 2] {
        &self.vertexes
    }

    pub fn is_intersecting(&self, other: &Self) -> bool {
        let vertex_a = self.vertexes[0];
        let vertex_b = other.vertexes[0];
        let vertex_c = self.vertexes[1];
        let vertex_d = other.vertexes[1];

        let vector_ab = vertex_a - vertex_b;
        let vector_bc = vertex_b - vertex_c;
        let vector_cd = vertex_c - vertex_d;
        let vector_da = vertex_d - vertex_a;

        let z_abc = cross(vector_ab, vector_bc);
        let z_bcd = cross(vector_bc, vector_cd);
        let z_cda = cross(vector_cd, vector_da);
        let z_dab = cross(vector_da, vector_ab);

        ((z_abc > Coord::ZERO)
            && (z_bcd > Coord::ZERO)
            && (z_cda > Coord::ZERO)
            && (z_dab > Coord::ZERO))
            || ((z_abc < Coord::ZERO)
                && (z_bcd < Coord::ZERO)
                && (z_cda < Coord::ZERO)
                && (z_dab < Coord::ZERO))
    }

    pub fn from_indexed_edge(value: IndexedEdge, vertexes: &TiSlice<VertexIndex, Vertex>) -> Self {
        let pair = value.get_vertex_index_pair();
        Edge::new([vertexes[pair[0]], vertexes[pair[1]]])
    }
}
