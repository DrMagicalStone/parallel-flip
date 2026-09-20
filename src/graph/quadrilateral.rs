use std::{debug_assert, fmt::{Debug, Display}};

use crate::graph::{basic_alge::Cyclic4Indexed, basic_geom::{QuadrilateralVertexIndex}, vec2c::cross, vertex::{Coord, Vertex}};

/// A quadrilateral storages its four vertexes.
/// Every time a quadrilateral changed and the change may lead to its ability of flipping changes,
/// the quadrilateral will immediately re-calculate it.
#[derive(PartialEq, Eq, Hash)]
pub struct Quadrilateral {
    vertexes: Cyclic4Indexed<QuadrilateralVertexIndex, Vertex>,

    flippable: bool,
}

impl Debug for Quadrilateral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Quadrilateral").field("vertexes", &self.vertexes).field("flippable", &self.flippable).finish()
    }
}

impl Clone for Quadrilateral {
    fn clone(&self) -> Self {
        Self {
            vertexes: self.vertexes.clone(),
            flippable: self.flippable,
        }
    }
}

impl Copy for Quadrilateral {}

impl Quadrilateral {
    pub fn new(vertexes: [Vertex; 4]) -> Self {
        Quadrilateral {
            vertexes: Cyclic4Indexed::new(vertexes),
            flippable: check_is_flippable(vertexes[0], vertexes[1], vertexes[2], vertexes[3]),
        }
    }

    pub fn get_vertexes_in_tuple(&self) -> (Vertex, Vertex, Vertex, Vertex) {
        (self.vertexes.value[0], self.vertexes.value[1], self.vertexes.value[2], self.vertexes.value[3])
    }

    pub fn get_vertexes_in_slice(&self) -> [Vertex; 4] {
        self.vertexes.value
    }

    pub fn get_vertexes(&self) -> &Cyclic4Indexed<QuadrilateralVertexIndex, Vertex> {
        &self.vertexes
    }

    pub fn get_vertexes_mut(&mut self) -> &mut Cyclic4Indexed<QuadrilateralVertexIndex, Vertex> {
        &mut self.vertexes
    }

    pub(crate) fn flip(&mut self) -> bool {
        debug_assert!(self.is_flippable());

        let rotate_left = self.vertexes[QuadrilateralVertexIndex::B] > self.vertexes[QuadrilateralVertexIndex::D];
        if rotate_left {
            self.vertexes.value.rotate_left(1);
        } else {
            self.vertexes.value.rotate_right(1);
        }
        rotate_left
    }

    fn update_is_flippable(&mut self) {
        self.flippable =
            check_is_flippable(self.vertexes.value[0], self.vertexes.value[1], self.vertexes.value[2], self.vertexes.value[3]);
    }

    pub fn is_flippable(&self) -> bool {
        self.flippable
    }
}



fn check_is_flippable(
    vertex_a: Vertex,
    vertex_b: Vertex,
    vertex_c: Vertex,
    vertex_d: Vertex,
) -> bool {
    let vector_ab = vertex_a - vertex_b;
    let vector_bc = vertex_b - vertex_c;
    let vector_cd = vertex_c - vertex_d;
    let vector_da = vertex_d - vertex_a;

    let z_abc = cross(vector_ab, vector_bc);
    let z_bcd = cross(vector_bc, vector_cd);
    let z_cda = cross(vector_cd, vector_da);
    let z_dab = cross(vector_da, vector_ab);

    (z_abc > Coord::ZERO) && (z_bcd > Coord::ZERO) && (z_cda > Coord::ZERO) && (z_dab > Coord::ZERO)
}
