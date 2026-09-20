//! The graph module aims to study parallel flipping of (geometric) triangulations of planar graphs.
//! A graph (in this case, a geometric triangulation) is considered as be composed of quadrilaterals.
//! Each two triangles who share the same edge make up a quadrilateral.
//! In a (geometric) triangulation, each triangle make up at least 1 quadrilateral (with 1 other triangle).
//! The only exception is a triangulation with only 3 vertexes and 3 edges that make up only a triangle which is not considered in this project.
//! As shown in the figure on crate/graph/graph.svg, a quadrilateral shares its two triangles with at most 4 other quadrilaterals.
//!
//! The relation that sharing a triangle is the most important relation between quadrilaterals in parallel flip operations.
//! The 4 vertexes of each quadrilateral are marked as A, B, C ans D in the order of [A, B, C, D] and
//! they always in a counter-clockwise order while the 4 edges are marked as AB, BC, CD, DE in the order of [AB, BC, CD, DA].
//! The diagonal of each quadrilateral which exist in the graph is always edge AC which means to flip a quadrilateral is to "rotate" it
//! which means to alter the 4 vertexes [A, B, C, D] into the original [D, A, B, C].
//! For a quadrilateral, to determine another quadrilateral in a shared-triangle relation with itself by determining
//! which edge is the other quadrilateral's diagonal (the edge the quadrilateral on).
//! Hence, a quadrilateral shall save the index of the at most 4 shared-triangle quadrilaterals on its 4 edges.
//! For example for the quadrilateral on the center, edge AB leads to quadrilateral 1.
//! However, to perform a flip, it is also needed to know for the other quadrilateral (in this case, quadrilateral 1),
//! if the shared triangle is triangle ABC or CDA.
//! By saving the information to the quadrilateral itself (in this case, the center quadrilateral),
//! only information saved in the quadrilateral itself is needed to perform a flip on a quadrilateral.
//! A quadrilateral (in this case, the center quadrilateral) save the information above by saving the vertex's name
//! which on the shared triangle but not on the quadrilateral's (in this case, quadrilateral 1, vertex B or D) diagonal.
//!
//! For both accuracy and performance, the graph module avoids float points and use integers only.

pub(crate) mod graph;
pub(crate) mod intersection_graph;
pub(crate) mod quadrilateral;
pub(crate) mod shared_triangle_relation;

pub(crate) mod vec2c;
pub(crate) mod vertex;
pub(crate) mod edge;
pub(crate) mod basic_geom;
pub(crate) mod basic_alge;
pub(crate) mod graph_entry;
pub(crate) mod quadrilateral_triangle_view;
pub(crate) mod quadrilateral_edge_view;
pub(crate) mod indexes;

#[cfg(test)]
mod tests {}
