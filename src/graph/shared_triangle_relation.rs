use std::{fmt::{Debug, Display}, write, writeln};

use crate::graph::{basic_alge::Cyclic4Indexed, basic_geom::QuadrilateralEdgeIndex, graph::UIndex};

/// The relation that sharing a triangle is the most important relation between quadrilaterals in parallel flip operations.
/// A SharedTriangleEntry storages the at most 4 quadrilaterals in a shared-triangle relation with itself "on" its 4 edges
/// by storing the indexes and the vertex's name of the quadrilaterals
/// which on the shared triangle but not on the quadrilateral's diagonal.
/// See [`crate::graph`]
#[derive(PartialEq, Eq, Hash)]
pub struct SharedTriangleEntry {
    pub(crate) index_quadrilateral_on_edges: Cyclic4Indexed<QuadrilateralEdgeIndex, Option<UIndex>>,
}

impl SharedTriangleEntry {
    pub fn new_r(
        index_quadrilateral_on_edge_ab: Option<UIndex>,
        index_quadrilateral_on_edge_bc: Option<UIndex>,
        index_quadrilateral_on_edge_cd: Option<UIndex>,
        index_quadrilateral_on_edge_da: Option<UIndex>,
    ) -> SharedTriangleEntry {
        SharedTriangleEntry {
            index_quadrilateral_on_edges: Cyclic4Indexed::new([
                index_quadrilateral_on_edge_ab,
                index_quadrilateral_on_edge_bc,
                index_quadrilateral_on_edge_cd,
                index_quadrilateral_on_edge_da,
            ]),
        }
    }

    pub fn new(
        index_quadrilateral_on_edges: [Option<UIndex>; 4]
    ) -> SharedTriangleEntry {
        SharedTriangleEntry {
            index_quadrilateral_on_edges: Cyclic4Indexed::new(index_quadrilateral_on_edges),
        }
    }

    pub fn flip(&mut self, rotate_left: bool) {
        if rotate_left {
            self.index_quadrilateral_on_edges.value.rotate_left(1);
        } else {
            self.index_quadrilateral_on_edges.value.rotate_right(1);
        }
    }

    pub fn get_index_quad_on_edge_raw(&self) -> &[Option<UIndex>; 4] {
        &self.index_quadrilateral_on_edges.value
    }

    pub fn get_index_quad_on_edge_mut_raw(&mut self) -> &mut [Option<UIndex>; 4] {
        &mut self.index_quadrilateral_on_edges.value
    }

    pub fn get_quadrilateral_on_edges(&self) -> &Cyclic4Indexed<QuadrilateralEdgeIndex, Option<UIndex>> {
        &self.index_quadrilateral_on_edges
    }

    pub fn get_quadrilateral_on_edges_mut(&mut self) -> &mut Cyclic4Indexed<QuadrilateralEdgeIndex, Option<UIndex>> {
        &mut self.index_quadrilateral_on_edges
    }
}

impl Display for SharedTriangleEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "")
    }
}

impl Debug for SharedTriangleEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SharedTriangleEntry")
            .field("index_quad_on_edges", &self.index_quadrilateral_on_edges)
            .finish()
    }
}

impl Clone for SharedTriangleEntry {
    fn clone(&self) -> Self {
        Self {
            index_quadrilateral_on_edges: self.index_quadrilateral_on_edges.clone(),
        }
    }
}

impl Copy for SharedTriangleEntry {}

