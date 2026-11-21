use std::ops::Deref;
use std::println;

use itertools::Itertools;
use typed_index_collections::{TiSlice, TiVec, ti_vec};

use crate::graph::edge::Edge;
use crate::graph::graph::flip::sort_quadrilaterals_get_numbers_of_flippable;
use crate::graph::graph::{Graph, GraphEntry, UIndex};

use crate::graph::indexes::{IndexedEdge, IndexedEdgeIndex, VertexIndex};
use crate::graph::vec2c::Vec2C;
use crate::graph::vertex::Coord;
use crate::graph::{
    quadrilateral::Quadrilateral, shared_triangle_relation::SharedTriangleEntry, vertex::Vertex,
};

use derive_more::{From, Into};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, From, Into)]
pub struct TriangleIndex(usize);

type QuadrilateralIndex = UIndex;

impl Graph {
    pub fn new(
        vertexes: &TiSlice<VertexIndex, Vertex>,
        edges_internal: &TiSlice<IndexedEdgeIndex, IndexedEdge>,
        edges_border: &TiSlice<IndexedEdgeIndex, IndexedEdge>,
    ) -> Graph {
        let edges = &*{
            let mut edges = edges_internal.to_vec();

            edges.append(&mut edges_border.to_vec());
            edges
        };
        let edges_internal = edges.split_at(IndexedEdgeIndex(edges_internal.len())).0;

        let vertex_to_edge = map_vertex_to_edge(vertexes, &*edges);
        let vertex_to_edge = &*vertex_to_edge
            .iter()
            .map(|v| &**v)
            .collect::<TiVec<VertexIndex, &[IndexedEdgeIndex]>>();

        let mut quads = TiVec::from(find_quadrilaterals(
            vertexes,
            edges,
            edges_internal,
            vertex_to_edge,
        ));
        sort_quadrilaterals_get_numbers_of_flippable(&mut quads);
        let numbers_of_flippable = quads.iter().filter(|q| q.0.is_flippable()).count();
        Graph {
            data: quads,
            numbers_of_flippable: UIndex::from(numbers_of_flippable),
        }
    }
}

fn map_vertex_to_edge<'a>(
    vertexes: &TiSlice<VertexIndex, Vertex>,
    edges: &'a TiSlice<IndexedEdgeIndex, IndexedEdge>,
) -> TiVec<VertexIndex, Vec<IndexedEdgeIndex>> {
    let mut vertex_to_edge: TiVec<VertexIndex, Vec<IndexedEdgeIndex>> =
        ti_vec![Vec::new(); vertexes.len()];
    edges.iter_enumerated().for_each(|(index, edge)| {
        vertex_to_edge[edge.get_vertex_index_pair()[0]].push(index);
        vertex_to_edge[edge.get_vertex_index_pair()[1]].push(index);
    });
    vertex_to_edge
}

fn find_quadrilaterals(
    vertexes: &TiSlice<VertexIndex, Vertex>,
    edges: &TiSlice<IndexedEdgeIndex, IndexedEdge>,
    edges_internal: &TiSlice<IndexedEdgeIndex, IndexedEdge>,
    vertex_to_edge: &TiSlice<VertexIndex, &[IndexedEdgeIndex]>,
) -> Vec<GraphEntry> {
    let mut quads: TiVec<QuadrilateralIndex, (Quadrilateral, [Option<IndexedEdgeIndex>; 4])> =
        edges_internal
            .iter()
            .map(|edge: &IndexedEdge| {
                let vertex_pair = edge.get_vertex_index_pair();

                let distance_comparator =
                    (vertexes[vertex_pair[1]] - vertexes[vertex_pair[0]]).get_orthogonal();

                let (mut triangle_on_pos_side, mut triangle_on_neg_side): (Vec<_>, Vec<_>) =
                    vertex_to_edge[vertex_pair[0]]
                        .iter()
                        .cartesian_product(vertex_to_edge[vertex_pair[1]].iter())
                        .filter_map(|(a, b)| {
                            debug_assert!(edges[*a].contains(vertex_pair[0]));
                            debug_assert!(edges[*b].contains(vertex_pair[1]));
                            if edges[*a].get_another_vertex_index(vertex_pair[0])
                                == edges[*b].get_another_vertex_index(vertex_pair[1])
                            {
                                Some((
                                    a,
                                    b,
                                    distance_comparator
                                        * Edge::new([
                                            vertexes[vertex_pair[0]],
                                            vertexes[edges[*a]
                                                .get_another_vertex_index(vertex_pair[0])],
                                        ])
                                        .as_vec2c(),
                                ))
                            } else {
                                None
                            }
                        })
                        .partition(|&t| t.2 > Coord::ZERO);
                triangle_on_neg_side.sort_by_key(|v| -v.2);
                triangle_on_pos_side.sort_by_key(|v| v.2);
                debug_assert!(triangle_on_neg_side.len() > 0);
                debug_assert!(triangle_on_pos_side.len() > 0);
                {
                    let (quadrilateral, need_rotate) = Quadrilateral::new([
                        vertexes[vertex_pair[0]],
                        vertexes[edges[*triangle_on_neg_side[0].0]
                            .get_another_vertex_index(vertex_pair[0])],
                        vertexes[vertex_pair[1]],
                        vertexes[edges[*triangle_on_pos_side[0].0]
                            .get_another_vertex_index(vertex_pair[0])],
                    ]);
                    let mut relation = [
                        if triangle_on_neg_side[0].0 < &IndexedEdgeIndex::from(edges_internal.len())
                        {
                            Some(*triangle_on_neg_side[0].0)
                        } else {
                            None
                        },
                        if triangle_on_neg_side[0].1 < &IndexedEdgeIndex::from(edges_internal.len())
                        {
                            Some(*triangle_on_neg_side[0].1)
                        } else {
                            None
                        },
                        if triangle_on_pos_side[0].1 < &IndexedEdgeIndex::from(edges_internal.len())
                        {
                            Some(*triangle_on_pos_side[0].1)
                        } else {
                            None
                        },
                        if triangle_on_pos_side[0].0 < &IndexedEdgeIndex::from(edges_internal.len())
                        {
                            Some(*triangle_on_pos_side[0].0)
                        } else {
                            None
                        },
                    ];
                    if need_rotate {
                        relation.rotate_left(2);
                    }
                    (quadrilateral, relation)
                }
            })
            .collect();
    let quads: Vec<_> = quads
        .iter()
        .enumerate()
        .map(|(_, (q, v))| {
            let args: Vec<_> = v
                .into_iter()
                .map(|e| {
                    if let Some(quad_on_edge) = e {
                        Some(UIndex::from(Into::<usize>::into(*quad_on_edge)))
                    } else {
                        None
                    }
                })
                .collect();
            GraphEntry::new(*q, SharedTriangleEntry::new(args[0..4].try_into().unwrap()))
        })
        .collect();
    quads
}

#[cfg(test)]
mod tests {
    use core::panic;
    use std::collections::HashMap;

    use ctor::ctor;
    use typed_index_collections::TiSlice;

    use crate::{
        graph::{
            basic_alge::Cyclic4,
            basic_geom::{QuadrilateralEdgeIndex, QuadrilateralVertexIndex},
            edge::Edge,
            graph::{
                Graph, GraphEntry, UIndex, quadrilateral_edge_view::QuadrilateralEdgeView,
                tests::test_if_graph_legal,
            },
            indexes::{IndexedEdge, IndexedEdgeIndex, NormalIndexedEdge, VertexIndex},
            quadrilateral::Quadrilateral,
            vertex::Vertex,
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
    fn new_graphs_from_json_and_test_if_the_graph_correct() {
        scan_all_examples_file_name().for_each(|problem_file_name| {
            let problem = load_problem(problem_file_name);

            let vertexes = assemb_vertexes_of_a_problem(&problem);

            let border_edges = scan_border_edges(&vertexes);

            let triangulations =
                assemb_triangulations_edges(&problem.triangulations, &border_edges);

            let graphs = build_graphs_of_a_problem(&triangulations, &border_edges, &vertexes);

            graphs.iter().for_each(test_if_graph_legal);

            graphs.iter().zip(triangulations.iter()).for_each(|(g, t)| {
                test_if_graph_equal_to_triangulation_from_json(g, t, &border_edges, &vertexes)
            });
        });
    }

    #[test]
    fn new_graphs_from_json_and_test_parallel_flip() {
        scan_all_examples_file_name().for_each(|problem_file_name| {
            let problem = load_problem(problem_file_name);

            let vertexes = assemb_vertexes_of_a_problem(&problem);

            let border_edges = scan_border_edges(&vertexes);

            let triangulations =
                assemb_triangulations_edges(&problem.triangulations, &border_edges);

            let graphs = build_graphs_of_a_problem(&triangulations, &border_edges, &vertexes);

            graphs.iter().for_each(test_if_graph_legal);
        });
    }

    fn test_if_graph_equal_to_triangulation_from_json(
        graph: &Graph,
        triangulation: &TiSlice<IndexedEdgeIndex, IndexedEdge>,
        border_edges: &TiSlice<IndexedEdgeIndex, IndexedEdge>,
        vertexes: &TiSlice<VertexIndex, Vertex>,
    ) {
        let vertexes_indexes: HashMap<Vertex, VertexIndex> =
            vertexes.iter_enumerated().map(|(i, v)| (*v, i)).collect();
        let mut indexed_edges_counts: HashMap<NormalIndexedEdge, bool> = triangulation
            .iter()
            .chain(border_edges.iter())
            .map(|e| (e.normalize(), false))
            .collect();

        graph.data.iter().for_each(|e| {
            let get_indexed_edge = |q: &Quadrilateral, e: &QuadrilateralEdgeIndex| {
                let v_p = vertexes_indexes[&q.get_vertexes()[e.get_prev_and_next_vertex()[0]]];
                let v_n = vertexes_indexes[&q.get_vertexes()[e.get_prev_and_next_vertex()[1]]];
                IndexedEdge::new([v_p, v_n]).normalize()
            };

            QuadrilateralEdgeIndex::EDGES.iter().for_each(|edge| {
                (*indexed_edges_counts
                    .get_mut(&get_indexed_edge(&e.0, edge))
                    .unwrap()) = true;
            });
        });

        assert!(
            indexed_edges_counts
                .values()
                .into_iter()
                .all(|visited| *visited)
        )
    }
}
