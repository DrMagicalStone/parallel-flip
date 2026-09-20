use std::ops::Deref;

use itertools::Itertools;
use typed_index_collections::{TiSlice, TiVec, ti_vec};

use crate::graph::edge::Edge;
use crate::graph::graph::{Graph, UIndex};
use crate::graph::graph_entry::GraphEntry;

use crate::graph::indexes::{IndexedEdge, IndexedEdgeIndex, VertexIndex};
use crate::graph::vec2c::Vec2C;
use crate::graph::vertex::Coord;
use crate::graph::{
    quadrilateral::Quadrilateral, shared_triangle_relation::SharedTriangleEntry, vertex::Vertex,
};


use derive_more::{From, Into};



#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, From, Into)]
pub struct TriangleIndex(usize);

type Triangle = (Vertex, Vertex, Vertex);


type QuadrilateralIndex = UIndex;



impl Graph {
    pub fn new(
        vertexes: & TiSlice<VertexIndex, Vertex>,
        edges_internal: & TiSlice<IndexedEdgeIndex, IndexedEdge>,
        edges_border: & TiSlice<IndexedEdgeIndex, IndexedEdge>,
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

        let quads = find_quadrilaterals(vertexes, edges, edges_internal, vertex_to_edge);
        let numbers_of_flippable = quads.iter().filter(|q| q.0.is_flippable()).count();
        Graph {
            data: TiVec::from(quads),
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
        vertex_to_edge[edge.get_vertex_index_pair()[0]]
            .push(index);
        vertex_to_edge[edge.get_vertex_index_pair()[1]]
            .push(index);
    });
    vertex_to_edge
}

fn get_border_f(vertexes: &[Vertex]) -> Vec<(Vertex, Vertex)> {
    let sorted = {
        let mut copy = vertexes.to_vec().into_boxed_slice();
        copy.sort();
        copy
    };

    let mut border: Vec<Vertex> = Vec::new();
    sorted
        .iter()
        .chain(sorted.iter().rev().skip(1))
        .for_each(|v| {
            while let Some([_, _]) = border.last_chunk().take_if(|[last_2, last]| {
                (*last - *last_2)
                    .orientation(*v - *last)
                    .is_counter_clockwise()
            }) {
                border.pop();
            }
            border.push(*v);
        });

    border
        .into_iter()
        .tuple_windows()
        .collect::<Vec<(Vertex, Vertex)>>()
}

fn find_quadrilaterals(
    vertexes: & TiSlice<VertexIndex, Vertex>,
    edges: & TiSlice<IndexedEdgeIndex, IndexedEdge>,
    edges_internal: & TiSlice<IndexedEdgeIndex, IndexedEdge>,
    vertex_to_edge: & TiSlice<VertexIndex, &[IndexedEdgeIndex]>,
) -> Vec<GraphEntry> {
    let mut quads: TiVec<QuadrilateralIndex, (Quadrilateral, [Option<IndexedEdgeIndex>; 4])> =
        TiVec::with_capacity(edges_internal.len());
    edges_internal.iter().fold(&mut quads, |quads, edge: & IndexedEdge| {
        let vertex_pair = edge.get_vertex_index_pair();
        fn rotate_vec2c(ori: Vec2C) -> Vec2C {
            let ori = ori.get();
            Vec2C::new(-ori.1, ori.0)
        }

        let distance_comparator = rotate_vec2c(vertexes[vertex_pair[1]] - vertexes[vertex_pair[0]]);

        let (mut triangle_on_pos_side, mut triangle_on_neg_side): (Vec<_>, Vec<_>) = vertex_to_edge
            [vertex_pair[0]]
        .iter()
        .cartesian_product(vertex_to_edge[vertex_pair[1]].iter())
        .filter_map(|(a, b)| {
            assert!(edges[*a].contains(vertex_pair[0]));
            assert!(edges[*b].contains(vertex_pair[1]));
            if edges[*a].get_another_vertex_index(vertex_pair[0]) == edges[*b].get_another_vertex_index(vertex_pair[1]) {
                Some((a, b, distance_comparator * Edge::from_indexed_edge(edges[*a], vertexes).as_vec2c()))
            } else {
                None
            }
        })
        .partition(|&t| t.2 > Coord::ZERO);
        triangle_on_neg_side.sort_by_key(|v| -v.2);
        triangle_on_pos_side.sort_by_key(|v| v.2);
        {
            quads.push((
                Quadrilateral::new([
                    vertexes[vertex_pair[0]],
                    vertexes[edges[*triangle_on_neg_side[0].0].get_another_vertex_index(vertex_pair[0])],
                    vertexes[vertex_pair[1]],
                    vertexes[edges[*triangle_on_pos_side[0].0].get_another_vertex_index(vertex_pair[1])],
                ]),
                [
                    if triangle_on_neg_side[0].0 < &IndexedEdgeIndex::from(edges_internal.len()) {
                        Some(*triangle_on_neg_side[0].0)
                    } else {
                        None
                    },
                    if triangle_on_neg_side[0].1 < &IndexedEdgeIndex::from(edges_internal.len()) {
                        Some(*triangle_on_neg_side[0].1)
                    } else {
                        None
                    },
                    if triangle_on_pos_side[0].1 < &IndexedEdgeIndex::from(edges_internal.len()) {
                        Some(*triangle_on_pos_side[0].1)
                    } else {
                        None
                    },
                    if triangle_on_pos_side[0].0 < &IndexedEdgeIndex::from(edges_internal.len()) {
                        Some(*triangle_on_pos_side[0].0)
                    } else {
                        None
                    },
                ],
            ));
        }
        quads
    });
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
            GraphEntry(
                *q,
                SharedTriangleEntry::new_r(args[0], args[1], args[2], args[3]),
            )
        })
        .collect();
    quads
}

fn find_triangles(
    edges_border: &[(Vertex, Vertex)],
    edges_internal: &[(Vertex, Vertex)],
) -> Vec<Triangle> {
    let mut edges: Vec<(Vertex, Vertex, usize)> =
        Vec::with_capacity(edges_border.len() + edges_internal.len());
    edges.append(
        &mut edges_border
            .iter()
            .map(|(a, b)| (*a, *b, 1 as usize))
            .collect(),
    );
    edges.append(
        &mut edges_internal
            .iter()
            .map(|(a, b)| (*a, *b, 2 as usize))
            .collect(),
    );

    let mut triangles: Vec<Triangle> =
        Vec::with_capacity((edges_border.len() + edges_internal.len() * 2) / 3);

    while let Some(any_edge_with_one_remaining_triangle) = edges
        .iter()
        .find_position(|(_, _, triangles_remaining)| *triangles_remaining == 1)
    {
        let any_edge_with_one_remaining_triangle = {
            let index = any_edge_with_one_remaining_triangle.0;
            let any_edge_with_one_remaining_triangle = *any_edge_with_one_remaining_triangle.1;
            edges.swap_remove(index);
            any_edge_with_one_remaining_triangle
        };
        let one_or_two_triangle = find_one_or_two_triangle(
            (
                any_edge_with_one_remaining_triangle.0,
                any_edge_with_one_remaining_triangle.1,
            ),
            edges.as_slice(),
        );
        let one_or_two_triangle = [Some(one_or_two_triangle.0), one_or_two_triangle.1];
        let one_or_two_triangle = one_or_two_triangle.iter().filter_map(|v| *v);
        one_or_two_triangle.clone().for_each(|(v, _, _)| {
            triangles.push((
                any_edge_with_one_remaining_triangle.0,
                any_edge_with_one_remaining_triangle.1,
                v,
            ));
        });
        one_or_two_triangle
            .flat_map(|(_, index_a, index_b)| [index_a, index_b])
            .sorted()
            .rev()
            .for_each(|index| {
                if edges[index].2 == 1 {
                    edges.swap_remove(index);
                } else {
                    edges[index].2 = edges[index].2 - 1;
                }
            });
    }
    triangles
}

fn find_one_or_two_triangle(
    edge_on: (Vertex, Vertex),
    edges: &[(Vertex, Vertex, usize)],
) -> ((Vertex, usize, usize), Option<(Vertex, usize, usize)>) {
    fn find_edges_on_vertex(
        vertex: Vertex,
        edges: &[(Vertex, Vertex, usize)],
    ) -> Vec<(usize, Vertex)> {
        edges
            .iter()
            .enumerate()
            .filter_map(|(index, edge)| match (vertex == edge.0, vertex == edge.1) {
                (true, false) => Some((index, edge.1)),
                (false, true) => Some((index, edge.0)),
                (false, false) => None,
                (true, true) => panic!("Incorrect graph."),
            })
            .collect()
    }
    let index_edges_on_one_vertex = find_edges_on_vertex(edge_on.0, edges);

    let index_edges_on_another_vertex = find_edges_on_vertex(edge_on.1, edges);

    fn rotate_vec2c(ori: Vec2C) -> Vec2C {
        let ori = ori.get();
        Vec2C::new(-ori.1, ori.0)
    }

    let distance_comparator = rotate_vec2c(edge_on.1 - edge_on.0);

    let (mut found_triangles_on_one_side, mut found_triangles_on_another_side): (Vec<_>, Vec<_>) =
        index_edges_on_one_vertex
            .iter()
            .cartesian_product(index_edges_on_another_vertex)
            .filter(|((_, v1), (_, v2))| *v1 == *v2)
            .map(|(&(i1, v1), (i2, _))| (i1, i2, (v1 - edge_on.0) * distance_comparator, v1))
            .partition(|v| v.2 > Coord::ZERO);

    found_triangles_on_one_side.sort_by_key(|k| k.2);
    found_triangles_on_another_side.sort_by_key(|k| -k.2);

    let found_triangles = (
        found_triangles_on_one_side.first(),
        found_triangles_on_another_side.first(),
    );

    match found_triangles {
        (Some(t1), Some(t2)) => ((t1.3, t1.0, t1.1), Some((t2.3, t2.0, t2.1))),
        (Some(t1), None) => ((t1.3, t1.0, t1.1), None),
        (None, Some(t2)) => ((t2.3, t2.0, t2.1), None),
        (None, None) => panic!("Incorrect graph or incorrect edge elimination"),
    }
}
