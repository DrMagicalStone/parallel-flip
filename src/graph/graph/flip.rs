use std::debug_assert;

use typed_index_collections::{TiSlice, TiVec, ti_vec};

use crate::graph::basic_geom::{QuadrilateralEdgeIndex, QuadrilateralVertexIndex};
use crate::graph::graph::quadrilateral_edge_view::QuadrilateralEdgeView;
use crate::graph::graph::quadrilateral_triangle_view::QuadrilateralTriangleView;
use crate::graph::graph::{Graph, GraphEntry, UIndex};

use crate::graph::{
    quadrilateral::Quadrilateral, shared_triangle_relation::SharedTriangleEntry, vertex::Vertex,
};

impl Graph {
    pub fn get_numbers_of_flippable(&self) -> UIndex {
        self.numbers_of_flippable
    }

    pub fn try_parallel_flip(&self, indexes_to_flip: &TiSlice<UIndex, bool>) -> Option<Self> {
        if self.can_flip_in_parallel_flip(indexes_to_flip) {
            let mut copy = self.clone();
            copy.assume_no_conflict_parallel_flip(indexes_to_flip);
            copy.numbers_of_flippable =
                sort_quadrilaterals_get_numbers_of_flippable(&mut copy.data);
            Some(copy)
        } else {
            None
        }
    }

    pub fn can_flip_in_parallel_flip(&self, indexes_to_flip: &TiSlice<UIndex, bool>) -> bool {
        assert!(UIndex::from(indexes_to_flip.len()) == self.numbers_of_flippable);

        let mut index_modified: TiVec<UIndex, bool> =
            ti_vec![false; self.numbers_of_flippable.into()];
        let index_modified = index_modified.as_mut_slice();
        indexes_to_flip
            .iter_enumerated()
            .try_fold((), |_, (i, flip)| match (*flip, index_modified[i]) {
                (true, true) => Err(()),
                (true, false) => {
                    index_modified[i] = true;
                    let related = &(self.data[i].1).get_index_quad_on_edge_raw();
                    related.iter().for_each(|o| {
                        if let &Some(i) = o {
                            if i < UIndex::from(index_modified.len()) {
                                index_modified[i] = true
                            }
                        }
                    });
                    Ok(())
                }
                _ => Ok(()),
            })
            .is_ok()
    }

    pub(super) fn assume_no_conflict_parallel_flip(
        &mut self,
        indexes_to_flip: &TiSlice<UIndex, bool>,
    ) {
        debug_assert!(self.can_flip_in_parallel_flip(indexes_to_flip));

        indexes_to_flip
            .iter_enumerated()
            .filter(|(_, flip)| **flip)
            .map(|(i, _)| i)
            .for_each(|i| {
                let graph_entry = self.data[i];
                
                let [_vertex_a, vertex_b, _vertex_c, vertex_d] = graph_entry.0.get_vertexes_in_slice();
                let related = graph_entry.1.get_quadrilateral_on_edges();

                QuadrilateralEdgeIndex::EDGES.into_iter().zip(
                [
                    (Some(i), related[QuadrilateralEdgeIndex::DA], &vertex_d),
                    (related[QuadrilateralEdgeIndex::CD], Some(i), &vertex_d),
                    (Some(i), related[QuadrilateralEdgeIndex::BC], &vertex_b),
                    (related[QuadrilateralEdgeIndex::AB], Some(i), &vertex_b),
                ].into_iter()).for_each(|(edge_index, (prev_quadrilateral_index, next_quadrilateral_index, new_vertex))| {
                    let Some(mut view) = QuadrilateralTriangleView::for_graph_entry_in_entry_array_on_index_in_center_graph_entry_and_intersect_with_center_graph_entry_mut(&graph_entry, edge_index, &mut self.data) else {return};

                    *view.get_vertex_of_view_mut() = *new_vertex;

                    view.get_graph_entry_of_view_mut().0.update_is_flippable();

                    *view.get_quadrilateral_index_on_prev_edge_mut() = prev_quadrilateral_index;
                    *view.get_quadrilateral_index_on_next_edge_mut() = next_quadrilateral_index;
                    
                });

                let graph_entry = &mut self.data[i];
                flip_graph_entry(graph_entry);
            });
    }
}

pub(super) fn sort_quadrilaterals_get_numbers_of_flippable(
    data: &mut TiSlice<UIndex, GraphEntry>,
) -> UIndex {
    let (mut new_indexes, numbers_of_flippable) =
        replace_index_of_flippable_to_top_get_numbers_of_flippable(data);
    {
        let (flippables, not_flippables) = new_indexes.split_at_mut(numbers_of_flippable);
        const ORIGIN: Vertex = Vertex::ORIGIN;
        let sort_by = |i: &UIndex| {
            let q = data[*i].0;
            let v = q.get_vertexes();
            ORIGIN + ((v[QuadrilateralVertexIndex::A] - ORIGIN) + (v[QuadrilateralVertexIndex::C] - ORIGIN))
        };
        flippables.sort_by_key(sort_by);
        not_flippables.sort_by_key(sort_by);
    }

    let mut visited: TiVec<UIndex, bool> = ti_vec![false; data.len()];

    for index_start in (0..data.len()).map(|x| UIndex::from(x)) {
        if visited[index_start] {
            continue;
        }

        let mut index_current = index_start;
        let mut current = data[index_current];

        loop {
            let index_next = new_indexes[index_current];
            (data[index_next], current) = (current, data[index_next]);
            visited[index_next] = true;
            if index_next == index_start {
                break;
            } else {
                index_current = index_next;
            }
        }
    }

    for index in (0..data.len()).map(|x| UIndex::from(x)) {
        data[index]
            .1
            .get_quadrilateral_on_edges_mut()
            .into_iter()
            .for_each(|index_on_edge| {
                if let Some(index_on_edge) = index_on_edge {
                    *index_on_edge = new_indexes[*index_on_edge];
                }
            });
    }

    numbers_of_flippable
}

fn replace_index_of_flippable_to_top_get_numbers_of_flippable(
    data: &TiSlice<UIndex, GraphEntry>,
) -> (TiVec<UIndex, UIndex>, UIndex) {
    let mut new_indexes: TiVec<UIndex, _> = (0..data.len()).map(|x| UIndex::from(x)).collect();
    let mut index_forward = UIndex(0);
    let mut index_backward = UIndex::from(data.len() - 1);
    'out: loop {
        loop {
            if index_forward == index_backward {
                break 'out;
            }
            if !data[index_forward].0.is_flippable() {
                break;
            }
            index_forward = index_forward + 1;
        }
        loop {
            if index_forward == index_backward {
                break 'out;
            }
            if data[index_backward].0.is_flippable() {
                break;
            }
            index_backward = index_backward - 1;
        }
        (new_indexes[index_forward], new_indexes[index_backward]) =
            (new_indexes[index_backward], new_indexes[index_forward]);
    }
    if index_forward == UIndex::from(data.len())
        && data[new_indexes[index_forward]].0.is_flippable()
    {
        return (new_indexes, index_forward + 1);
    }
    (new_indexes, index_forward)
}

fn alter_related_quads_exchange(data: &mut TiSlice<UIndex, GraphEntry>, from: UIndex, to: UIndex) {
    let graph_entry = data[from];

    QuadrilateralEdgeIndex::EDGES.into_iter().for_each(|edge_index| {
        let Some(mut view) = QuadrilateralEdgeView::for_graph_entry_in_entry_array_on_index_in_center_graph_entry_and_edge_of_graph_entry_as_diagonal_of_center_graph_entry_mut(&graph_entry, edge_index, data) else { return };

        *view.get_quadrilateral_index_on_edge_of_view_mut() = to;
    });
}

/// Flipping is a kind of rotation in the view of memory layout.
/// This method makes sure that two Quadrilateral struct which represents the same quadrilateral (considering their diagonals) has the same memory layout.
fn flip_graph_entry(entry: &mut GraphEntry) {
    let rotate_left = entry.0.flip();
    entry.1.flip(rotate_left);
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use ctor::ctor;

    use crate::util::{
        assemb_triangulations_edges, assemb_vertexes_of_a_problem, build_graphs_of_a_problem,
        load_problem, scan_all_examples_file_name, scan_border_edges,
    };

    #[ctor(unsafe)]
    fn init_color_backtrace() {
        color_backtrace::install();
    }

    #[test]
    fn new_graphs_from_json_and_test_reflexivity() {
        scan_all_examples_file_name().for_each(|problem_file_name| {
            let problem = load_problem(problem_file_name);

            let vertexes = assemb_vertexes_of_a_problem(&problem);

            let border_edges = scan_border_edges(&vertexes);

            let triangulations =
                assemb_triangulations_edges(&problem.triangulations, &border_edges);

            let graphs = build_graphs_of_a_problem(&triangulations, &border_edges, &vertexes);

            graphs.iter().for_each(|g| {
                let all_parallel_flips = g.get_all_possible_parallel_flips_with_no_sort();
                all_parallel_flips.into_iter().for_each(|mut flip| {
                    flip.0.assume_no_conflict_parallel_flip(&flip.2);
                    assert_eq!(flip.0, *g);
                });
            });
        });
    }
}
