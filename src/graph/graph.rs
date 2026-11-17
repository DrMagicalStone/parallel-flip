mod flip;
mod new;
pub mod quadrilateral_edge_view;
pub mod quadrilateral_triangle_view;
mod search;

use std::{
    borrow::{Borrow, BorrowMut},
    ops::{Add, Deref, DerefMut, Sub},
};

use pathfinding::num_traits::Zero;
use typed_index_collections::{TiSlice, TiVec};

use crate::graph::{quadrilateral::Quadrilateral, shared_triangle_relation::SharedTriangleEntry};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UIndex(pub u32);

/// See graph.rs in src for more info.
#[derive(PartialEq, Eq, Hash, Debug)]
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

impl AsRef<TiSlice<UIndex, GraphEntry>> for Graph {
    fn as_ref(&self) -> &TiSlice<UIndex, GraphEntry> {
        &self.data
    }
}

impl UIndex {
    pub const ZERO: Self = Self(0);
}

impl Zero for UIndex {
    fn zero() -> Self {
        Self::ZERO
    }

    fn is_zero(&self) -> bool {
        *self == Self::ZERO
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

impl Add<Self> for UIndex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        UIndex(self.0 + rhs.0)
    }
}

impl Sub<Self> for UIndex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        UIndex(self.0 - rhs.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GraphEntry {
    data: (Quadrilateral, SharedTriangleEntry),
}

impl Deref for GraphEntry {
    type Target = (Quadrilateral, SharedTriangleEntry);

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for GraphEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

impl AsRef<(Quadrilateral, SharedTriangleEntry)> for GraphEntry {
    fn as_ref(&self) -> &(Quadrilateral, SharedTriangleEntry) {
        &*self
    }
}

impl AsMut<(Quadrilateral, SharedTriangleEntry)> for GraphEntry {
    fn as_mut(&mut self) -> &mut (Quadrilateral, SharedTriangleEntry) {
        &mut *self
    }
}

impl Borrow<(Quadrilateral, SharedTriangleEntry)> for GraphEntry {
    fn borrow(&self) -> &(Quadrilateral, SharedTriangleEntry) {
        &*self
    }
}

impl BorrowMut<(Quadrilateral, SharedTriangleEntry)> for GraphEntry {
    fn borrow_mut(&mut self) -> &mut (Quadrilateral, SharedTriangleEntry) {
        &mut *self
    }
}

impl GraphEntry {
    pub fn new(quadrilateral: Quadrilateral, relation: SharedTriangleEntry) -> Self {
        Self {
            data: (quadrilateral, relation),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use std::collections::HashMap;

    use typed_index_collections::TiSlice;

    use crate::graph::{
        basic_alge::Cyclic4,
        basic_geom::{QuadrilateralEdgeIndex, QuadrilateralVertexIndex},
        edge::Edge,
        graph::{Graph, GraphEntry, UIndex, quadrilateral_edge_view::QuadrilateralEdgeView},
    };

    pub fn test_if_graph_legal(graph: &Graph) {
        let for_each_graph_entrys_test_if_indexes_on_edges_to_others = |(i_0, q_0): (
            UIndex,
            &GraphEntry,
        )| {
            QuadrilateralEdgeIndex::EDGES.iter().for_each(|e | {
            let Some(view) = QuadrilateralEdgeView::for_graph_entry_in_entry_array_on_index_in_center_graph_entry_and_edge_of_graph_entry_as_diagonal_of_center_graph_entry(q_0, *e, graph.as_ref()) else {
                return;
            };
            assert_eq!(*view.get_quadrilateral_index_on_edge_of_view(), i_0);
        });
        };

        let for_each_graph_entrys_test_if_each_triangle_in_different_quadrilateral_consistent =
            |q: &GraphEntry| {
                QuadrilateralEdgeIndex::EDGES.iter().for_each(|e: &QuadrilateralEdgeIndex | {
            let Some(view) = QuadrilateralEdgeView::for_graph_entry_in_entry_array_on_index_in_center_graph_entry_and_edge_of_graph_entry_as_diagonal_of_center_graph_entry(q, *e, graph.as_ref()) else {
                return;
            };
            if <Cyclic4 as Into<u8>>::into((*e).into()) < 2 {
                assert_eq!(*view.get_prev_vertex(), q.0.get_vertexes()[QuadrilateralVertexIndex::C]);
                assert_eq!(*view.get_next_vertex(), q.0.get_vertexes()[QuadrilateralVertexIndex::A]);
            } else {
                assert_eq!(*view.get_prev_vertex(), q.0.get_vertexes()[QuadrilateralVertexIndex::A]);
                assert_eq!(*view.get_next_vertex(), q.0.get_vertexes()[QuadrilateralVertexIndex::C]);
            }
        });
            };

        let test_if_each_border_edge_be_indexed_by_correct_numbers_of_quadrilateral =
            |graph: &TiSlice<UIndex, GraphEntry>| {
                enum EdgeType {
                    EdgeOnOneQuadrilateral(UIndex),
                    EdgeOnTwoQuadrilateral([Option<UIndex>; 2]),
                }

                impl EdgeType {
                    fn insert_edge_on_two_quadrilateral(
                        map: &mut HashMap<Edge, EdgeType>,
                        entry: &GraphEntry,
                        entry_index: UIndex,
                        edge_index: QuadrilateralEdgeIndex,
                    ) {
                        let edge = Edge::new([
                            entry.0.get_vertexes()[edge_index.get_prev_and_next_vertex()[0]],
                            entry.0.get_vertexes()[edge_index.get_prev_and_next_vertex()[1]],
                        ]);
                        let i;
                        if <Cyclic4 as Into<u8>>::into(edge_index.into()) % 2u8 == 0 {
                            i = 1usize;
                        } else {
                            i = 0usize;
                        }
                        if let Some(t) = map.get_mut(&edge) {
                            match t {
                                EdgeType::EdgeOnOneQuadrilateral(_i) => {
                                    panic!()
                                }
                                EdgeType::EdgeOnTwoQuadrilateral(index_on_two_edges) => {
                                    if let None = index_on_two_edges[i] {
                                        index_on_two_edges[i] = Some(entry_index);
                                    } else {
                                        panic!();
                                    }
                                }
                            }
                        } else {
                            let mut index_on_two_edges = [None; 2];
                            index_on_two_edges[i] = Some(entry_index);
                            map.insert(edge, EdgeType::EdgeOnTwoQuadrilateral(index_on_two_edges));
                        }
                    }
                }

                let mut edge_to_type: HashMap<Edge, EdgeType> = HashMap::new();

                graph.iter_enumerated().for_each(|(i, q)| {
                    if q.1.get_quadrilateral_on_edges()[QuadrilateralEdgeIndex::AB].is_none()
                        && q.1.get_quadrilateral_on_edges()[QuadrilateralEdgeIndex::BC].is_none()
                    {
                        edge_to_type.insert(
                            Edge::new([
                                q.0.get_vertexes()[QuadrilateralVertexIndex::A],
                                q.0.get_vertexes()[QuadrilateralVertexIndex::B],
                            ]),
                            EdgeType::EdgeOnOneQuadrilateral(i),
                        );
                        edge_to_type.insert(
                            Edge::new([
                                q.0.get_vertexes()[QuadrilateralVertexIndex::B],
                                q.0.get_vertexes()[QuadrilateralVertexIndex::C],
                            ]),
                            EdgeType::EdgeOnOneQuadrilateral(i),
                        );
                    } else if q.1.get_quadrilateral_on_edges()[QuadrilateralEdgeIndex::AB].is_none()
                    {
                        EdgeType::insert_edge_on_two_quadrilateral(
                            &mut edge_to_type,
                            q,
                            i,
                            QuadrilateralEdgeIndex::AB,
                        );
                    } else if q.1.get_quadrilateral_on_edges()[QuadrilateralEdgeIndex::BC].is_none()
                    {
                        EdgeType::insert_edge_on_two_quadrilateral(
                            &mut edge_to_type,
                            q,
                            i,
                            QuadrilateralEdgeIndex::BC,
                        );
                    }

                    if q.1.get_quadrilateral_on_edges()[QuadrilateralEdgeIndex::CD].is_none()
                        && q.1.get_quadrilateral_on_edges()[QuadrilateralEdgeIndex::DA].is_none()
                    {
                        edge_to_type.insert(
                            Edge::new([
                                q.0.get_vertexes()[QuadrilateralVertexIndex::C],
                                q.0.get_vertexes()[QuadrilateralVertexIndex::D],
                            ]),
                            EdgeType::EdgeOnOneQuadrilateral(i),
                        );
                        edge_to_type.insert(
                            Edge::new([
                                q.0.get_vertexes()[QuadrilateralVertexIndex::D],
                                q.0.get_vertexes()[QuadrilateralVertexIndex::A],
                            ]),
                            EdgeType::EdgeOnOneQuadrilateral(i),
                        );
                    } else if q.1.get_quadrilateral_on_edges()[QuadrilateralEdgeIndex::CD].is_none()
                    {
                        EdgeType::insert_edge_on_two_quadrilateral(
                            &mut edge_to_type,
                            q,
                            i,
                            QuadrilateralEdgeIndex::CD,
                        );
                    } else if q.1.get_quadrilateral_on_edges()[QuadrilateralEdgeIndex::DA].is_none()
                    {
                        EdgeType::insert_edge_on_two_quadrilateral(
                            &mut edge_to_type,
                            q,
                            i,
                            QuadrilateralEdgeIndex::DA,
                        );
                    }
                });

                edge_to_type.iter().for_each(|(e, t)| {
                    if let EdgeType::EdgeOnTwoQuadrilateral(index_on_two_edges) = t {
                        let i_0 = index_on_two_edges[0].unwrap();
                        let i_1 = index_on_two_edges[1].unwrap();

                        if graph[i_0].0.get_vertexes()[QuadrilateralVertexIndex::B]
                            == e.get_vertexes()[0]
                        {
                            assert_eq!(graph[i_0].1.get_index_quad_on_edge_raw()[0], Some(i_1));
                        } else {
                            assert_eq!(graph[i_0].1.get_index_quad_on_edge_raw()[2], Some(i_1));
                        }

                        if graph[i_1].0.get_vertexes()[QuadrilateralVertexIndex::A]
                            == e.get_vertexes()[0]
                        {
                            assert_eq!(graph[i_1].1.get_index_quad_on_edge_raw()[1], Some(i_0));
                        } else {
                            assert_eq!(graph[i_1].1.get_index_quad_on_edge_raw()[3], Some(i_0));
                        }
                    }
                });
            };

        graph
            .data
            .iter_enumerated()
            .for_each(for_each_graph_entrys_test_if_indexes_on_edges_to_others);
        graph.data.iter().for_each(
            for_each_graph_entrys_test_if_each_triangle_in_different_quadrilateral_consistent,
        );
        test_if_each_border_edge_be_indexed_by_correct_numbers_of_quadrilateral(graph.as_ref());
    }
}
