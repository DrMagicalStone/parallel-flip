use std::cmp::Reverse;
use std::collections::{BTreeSet, HashMap};
use std::ops::Deref;
use std::{assert_eq, assert_ne, debug_assert, vec};

use ahash::HashSet;
use itertools::Itertools;
use rand::SeedableRng;
use rand::rngs::StdRng;
use typed_index_collections::{TiSlice, TiVec, ti_vec};

use crate::graph::edge::Edge;
use crate::graph::graph::{Graph, UIndex};
use crate::graph::indexes::{
    IndexedEdge, IndexedEdgeIndex, NormalIndexedEdge, TriangulationIndex, VertexIndex,
};
use crate::graph::intersection_graph::{
    self, Counter, EdgeState, IntersectionGraph, IntersectionGraphItem, IntersectionGraphIter,
};

use crate::graph::vec2c::{Vec2C, orientation};
use crate::graph::vertex::Coord;
use crate::graph::{
    quadrilateral::Quadrilateral, shared_triangle_relation::SharedTriangleEntry, vertex::Vertex,
};

impl<'a> IntersectionGraph<'a> {
    pub fn new(
        vertexes: &'a TiSlice<VertexIndex, Vertex>,
        triangulations: &'a TiSlice<TriangulationIndex, TiVec<IndexedEdgeIndex, IndexedEdge>>,
        border_edges: &'a TiSlice<IndexedEdgeIndex, IndexedEdge>,
    ) -> Self {
        let mut counts_in_triangulations: HashMap<NormalIndexedEdge, usize> =
            HashMap::with_capacity(get_all_possible_edges_number(vertexes));

        triangulations.iter().for_each(|triangulation| {
            triangulation
                .iter()
                .map(|e| e.normalize())
                .for_each(|edge| {
                    counts_in_triangulations
                        .entry(edge)
                        .and_modify(|c| *c += 1)
                        .or_insert(1);
                });
        });

        let all_indexed_edges_sorted_by_counts = {
            let mut all_indexed_edges: TiVec<IndexedEdgeIndex, _> =
                form_edges(vertexes, border_edges).collect();
            all_indexed_edges
                .sort_by_key(|e| Reverse(counts_in_triangulations.get(e).unwrap_or(&0)));
            all_indexed_edges
        };

        let intersection_graph_items: TiVec<IndexedEdgeIndex, IntersectionGraphItem> =
            all_indexed_edges_sorted_by_counts
                .iter()
                .map(|e0| {
                    let intersecting_edges_index = all_indexed_edges_sorted_by_counts
                        .iter_enumerated()
                        .filter(|(_i1, e1)| {
                            Edge::from_indexed_edge((*e0).into(), vertexes)
                                .is_intersecting(&Edge::from_indexed_edge((**e1).into(), vertexes))
                        })
                        .map(|(i1, e1)| i1)
                        .collect();
                    IntersectionGraphItem {
                        edge: (*e0).into(),
                        intersecting_edges_index,
                    }
                })
                .collect();

        IntersectionGraph {
            intersection_graph_items,
            border_edges,
            vertex_list: vertexes,
        }
    }

    pub fn get_items(&self) -> &TiSlice<IndexedEdgeIndex, IntersectionGraphItem> {
        &*self.intersection_graph_items
    }
}

impl<'a> IntersectionGraphIter<'a> {
    pub(crate) fn new(
        intersection_graph: &'a IntersectionGraph<'a>,
        backtrak_depth_in_count_of_backtraked_edges: fn(u32) -> u32,
        backtraks_for_each_backtrak_depth: fn(u32) -> u32,
        random_tries_for_each_backtrak: fn(u32) -> u32,
    ) -> Self {
        let data = &intersection_graph.intersection_graph_items;

        let rng: StdRng = StdRng::seed_from_u64(0x123456); // temporary

        IntersectionGraphIter {
            intersection_graph,
            rng,
            backtrak_depth_in_count_of_backtraked_edges: Counter::new(
                backtrak_depth_in_count_of_backtraked_edges,
            ),
            backtraks_for_each_backtrak_depth: Counter::new(backtraks_for_each_backtrak_depth),
            random_tries_for_each_backtrak: Counter::new(random_tries_for_each_backtrak),
            edge_states: ti_vec![EdgeState::NoIntersecting; data.len()],
            on_graph_edges: Vec::new(),
            no_intersecting_edge_priority_list: (0..data.len())
                .map(|i| IndexedEdgeIndex(i))
                .collect(),
        }
    }
}

fn form_edges(
    vertexes: &TiSlice<VertexIndex, Vertex>,
    border_edges: &TiSlice<IndexedEdgeIndex, IndexedEdge>,
) -> impl Iterator<Item = NormalIndexedEdge> {
    let border_edges_filter: HashSet<_> = border_edges.iter().map(|e| e.normalize()).collect();

    vertexes
        .iter_enumerated()
        .map(|(i, _v)| i)
        .tuple_combinations()
        .map(|(v0, v1)| IndexedEdge::new([v0, v1]))
        .filter(|i_e| {
            let e = Edge::from_indexed_edge(*i_e, vertexes);
            !vertexes.iter_enumerated().any(|(i, v)| {
                if i_e.contains(i) {
                    return false;
                }
                let v_1 = e.get_vertexes()[1] - *v;
                let v_0 = *v - e.get_vertexes()[0];
                orientation(v_1, v_0).is_parallel() && (v_0 * v_1 > Coord::ZERO)
            })
        })
        .map(|e| e.normalize())
        .filter(move |e| !border_edges_filter.contains(&e))
}

fn get_all_possible_edges_number(vertexes: &TiSlice<VertexIndex, Vertex>) -> usize {
    vertexes.len() * (vertexes.len() - 1) / 2
}

#[cfg(test)]
mod tests {
    use std::{assert_eq, collections::HashSet};

    use ctor::ctor;

    use crate::{
        graph::{
            edge::Edge,
            indexes::IndexedEdgeIndex,
            intersection_graph::{self, IntersectionGraph, new::form_edges},
        },
        util::{
            assemb_triangulations_edges, assemb_vertexes_of_a_problem, build_graphs_of_a_problem,
            load_problem, scan_all_examples_file_name, scan_border_edges,
        },
    };

    #[ctor(unsafe)]
    fn init_color_backtrace() {
        color_backtrace::install();
    }

    #[test]
    fn new_intersection_graph_from_triangulations_and_test_if_intersecting() {
        scan_all_examples_file_name().for_each(|problem_file_name| {
            let problem = load_problem(problem_file_name);

            let vertexes = assemb_vertexes_of_a_problem(&problem);

            let border_edges = scan_border_edges(&vertexes);

            let triangulations =
                assemb_triangulations_edges(&problem.triangulations, &border_edges);

            let intersection_graph =
                IntersectionGraph::new(&vertexes, &triangulations, &border_edges);

            intersection_graph.get_items().iter().for_each(|item| {
                let intersecting_edges_index: HashSet<_> =
                    item.intersecting_edges_index.iter().cloned().collect();

                intersection_graph.get_items().iter_enumerated().for_each(
                    |(another_edge_index, another_edge)| {
                        assert_eq!(
                            intersecting_edges_index.contains(&another_edge_index),
                            Edge::from_indexed_edge(item.edge, &vertexes).is_intersecting(
                                &Edge::from_indexed_edge(another_edge.edge.into(), &vertexes)
                            )
                        );
                    },
                );
            });
        });
    }
}
