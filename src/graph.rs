//!The graph module aims to study (geometric) triangulations of planar graphs.
//! Its functionality spans graph theory (a branch of combinatorics) and geometry.
//! Therefore, the purely combinatorial part and the purely geometric part of this module
//! are separated into two submodules: combinatorial and geometry.
//! Both submodules have a one-way dependency on the parent graph module,
//! and geometry also has a one-way dependency on combinatorial.
//!
//! In particular, the Graph struct in the parent module depends on most of the structs and functions
//! across the entire graph module, including the submodules.

mod combinatorial;
mod edge_collection;
mod geometric;
pub(crate) mod vertex;
mod vertex_collection;

#[cfg(test)]
mod tests {
    use crate::graph::{
        combinatorial::edge,
        edge_collection::EdgeCollection,
        vertex::Vertex,
        vertex_collection::{self, VertexCollection},
    };

    #[test]
    fn tree() {
        let eight_vertex_array = [
            Vertex::new(0, 0),
            Vertex::new(2, 3),
            Vertex::new(1, 6),
            Vertex::new(7, 7),
            Vertex::new(4, 1),
            Vertex::new(6, 5),
            Vertex::new(3, 8),
            Vertex::new(9, 10),
        ];
        let vertex_collection = VertexCollection::new(&eight_vertex_array);
        let edge_collection = EdgeCollection::new(&vertex_collection);
    }

    fn build_triangulation_with_center(
        vertex_collection: &VertexCollection,
        edge_collection: &mut EdgeCollection,
        center_index: usize,
    ) {
        let iter_vertex_array_exclude_center = vertex_collection
            .iter()
            .enumerate()
            .filter(|(i, x)| *i != center_index);
        iter_vertex_array_exclude_center.clone().for_each(|(i, v)| {
            edge_collection.set_edge(&vertex_collection.new_edge((center_index, i)));
        });
        iter_vertex_array_exclude_center
            .flat_map(|a| vertex_collection.iter().enumerate().map(move |b| (a, b)))
            .for_each(|((ai, ax), (bi, bx))| {
                let edge = vertex_collection.new_edge((ai, bi));
                if !edge_collection.edge_intersect_with_any_edge(&edge) {
                    edge_collection.set_edge(&edge);
                }
            });
    }
}
