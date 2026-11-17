use std::debug_assert;

use crate::graph::graph::Graph;
use crate::graph::indexes::{IndexedEdge, IndexedEdgeIndex};
use crate::graph::intersection_graph::{
    Counter, EdgeState, IntersectionGraph, IntersectionGraphIter,
};
use rand::seq::IteratorRandom;
use typed_index_collections::TiVec;

impl<'a> Iterator for IntersectionGraphIter<'a> {
    type Item = Graph;

    fn next(&mut self) -> Option<Self::Item> {
        if self
            .backtrak_depth_in_count_of_backtraked_edges
            .get_remaining()
            == 0
        {
            self.reset();
        }
        self.decrease_backtrak_depth_once()
    }
}

impl<'a> IntersectionGraphIter<'a> {
    fn reset(&mut self) {
        self.backtrak_depth_in_count_of_backtraked_edges
            .reset_remaining(0);
        self.backtraks_for_each_backtrak_depth.reset_remaining(0);
        self.random_tries_for_each_backtrak.set_remaining(1);
        self.try_fill_graph_with_edge_from(IndexedEdgeIndex(0));
    }

    fn decrease_backtrak_depth_once(&mut self) -> Option<Graph> {
        if let Some(graph) = self.backtrack_once() {
            return Some(graph);
        }
        if !self
            .backtrak_depth_in_count_of_backtraked_edges
            .has_remaining_count_down()
        {
            return None;
        }
        self.pop_edge()?;
        self.backtrack_once()
    }

    fn backtrack_once(&mut self) -> Option<Graph> {
        if let Some(graph) = self.randomly_generate_once() {
            return Some(graph);
        }
        if !self
            .backtraks_for_each_backtrak_depth
            .has_remaining_count_down()
        {
            return None;
        }
        loop {
            let first_no_intersecting_edge_after_last_backtracked_edge_index =
                self.backtrack_until_found_free_edge()?;

            self.try_fill_graph_with_edge_from(
                first_no_intersecting_edge_after_last_backtracked_edge_index,
            );
            if self.is_all_edges_intersecting_or_on_graph()
                || (self.on_graph_edges.len()
                    == self
                        .backtrak_depth_in_count_of_backtraked_edges
                        .get_remaining() as usize)
            {
                self.random_tries_for_each_backtrak.remaining =
                    (self.random_tries_for_each_backtrak.target_provider)(0);
                return self.randomly_generate_once();
            }
        }
    }

    fn randomly_generate_once(&mut self) -> Option<Graph> {
        // if !self
        //     .random_tries_for_each_backtrak
        //     .has_remaining_count_down()
        // {
        //     return None;
        // }
        let original_on_graph_edges_len = self.on_graph_edges.len();
        for _ in self.on_graph_edges.len()..self.get_internal_edge_numbers_of_a_graph() {
            let e = self
                .no_intersecting_edge_priority_list
                .iter()
                .choose(&mut self.rng)
                .unwrap();
            self.push_edge(*e);
        }
        debug_assert!(self.no_intersecting_edge_priority_list.is_empty());
        let graph = self.new_graph_from_current_status();
        for _ in original_on_graph_edges_len..self.on_graph_edges.len() {
            self.pop_edge();
        }
        Some(graph)
    }

    fn backtrack_until_found_free_edge(&mut self) -> Option<IndexedEdgeIndex> {
        while !self.on_graph_edges.is_empty() {
            if let Some(first_no_intersecting_edge_after_last_backtracked_edge_index) =
                self.pop_edge()
            {
                return Some(first_no_intersecting_edge_after_last_backtracked_edge_index);
            }
        }
        None
    }

    fn try_fill_graph_with_edge_from(&mut self, index_from: IndexedEdgeIndex) {
        while let Some(current_edge_index) = {
            if self.on_graph_edges.len()
                < self
                    .backtrak_depth_in_count_of_backtraked_edges
                    .get_remaining() as usize
            {
                self.no_intersecting_edge_priority_list
                    .range(index_from..)
                    .next()
            } else {
                None
            }
        } {
            self.push_edge(*current_edge_index);
        }
    }

    fn pop_edge(&mut self) -> Option<IndexedEdgeIndex> {
        let last_edge = self.on_graph_edges.pop()?;
        let mut r: Option<IndexedEdgeIndex> = None;
        for e in self
            .intersection_graph
            .get_intersecting_edges_index(last_edge)
        {
            if self
                .get_edge_state_mut(*e)
                .decrease_intersect_count_and_if_changed_to_no_intersecting()
            {
                let inserted = self.no_intersecting_edge_priority_list.insert(*e);
                debug_assert!(inserted);
                if r.is_none() && (*e > last_edge) {
                    r = Some(*e);
                }
            }
        }
        r
    }

    fn push_edge(&mut self, index: IndexedEdgeIndex) {
        self.on_graph_edges.push(index);
        self.intersection_graph
            .get_intersecting_edges_index(index)
            .iter()
            .for_each(|e| {
                if self
                    .get_edge_state_mut(*e)
                    .increase_intersect_count_and_if_changed_to_intersecting()
                {
                    let removed = self.no_intersecting_edge_priority_list.remove(e);
                    debug_assert!(removed);
                }
            });
    }

    fn new_graph_from_current_status(&self) -> Graph {
        Graph::new(
            self.vertex_list,
            &*self
                .on_graph_edges
                .iter()
                .map(|i| *self.get_edge_on_index(*i))
                .collect::<TiVec<_, _>>(),
            self.border_edges,
        )
    }

    fn is_all_edges_intersecting_or_on_graph(&self) -> bool {
        self.no_intersecting_edge_priority_list.is_empty()
    }

    fn get_edge_state(&self, edge_index: IndexedEdgeIndex) -> &EdgeState {
        &self.edge_states[edge_index]
    }

    fn get_edge_state_mut(&mut self, edge_index: IndexedEdgeIndex) -> &mut EdgeState {
        &mut self.edge_states[edge_index]
    }

    fn get_internal_edge_numbers_of_a_graph(&self) -> usize {
        3 * self.vertex_list.len() - 6 - self.border_edges.len() - (self.border_edges.len() - 3)
    }
}

impl<'a> IntersectionGraph<'a> {
    pub fn get_intersecting_edges_index(
        &self,
        edge_index: IndexedEdgeIndex,
    ) -> &[IndexedEdgeIndex] {
        &*self.intersection_graph_items[edge_index].intersecting_edges_index
    }

    fn get_edge_on_index(&self, edge_index: IndexedEdgeIndex) -> &IndexedEdge {
        &self.intersection_graph_items[edge_index].edge
    }
}

impl<'a> IntersectionGraph<'a> {
    fn iter_with(
        &'a self,
        backtrak_depth_in_count_of_backtraked_edges: fn(u32) -> u32,
        backtraks_for_each_backtrak_depth: fn(u32) -> u32,
        random_tries_for_each_backtrak: fn(u32) -> u32,
    ) -> IntersectionGraphIter<'a> {
        IntersectionGraphIter::new(
            self,
            backtrak_depth_in_count_of_backtraked_edges,
            backtraks_for_each_backtrak_depth,
            random_tries_for_each_backtrak,
        )
    }
}

impl<'a> IntoIterator for &'a IntersectionGraph<'a> {
    type Item = Graph;

    type IntoIter = IntersectionGraphIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let always_return_100 = |_i| 100; // temporary
        self.iter_with(|i| i, always_return_100, always_return_100)
    }
}

#[cfg(test)]
mod tests {
    use std::{assert_eq, collections::HashSet};

    use ctor::ctor;

    use crate::{
        graph::{
            edge::Edge, graph::tests::test_if_graph_legal, intersection_graph::IntersectionGraph,
        },
        util::{
            assemb_triangulations_edges, assemb_vertexes_of_a_problem, load_problem,
            scan_all_examples_file_name, scan_border_edges,
        },
    };

    #[ctor(unsafe)]
    fn init_color_backtrace() {
        color_backtrace::install();
    }

    #[test]
    fn new_intersection_graph_from_triangulations_and_test_enumerate() {
        scan_all_examples_file_name().for_each(|problem_file_name| {
            let problem = load_problem(problem_file_name);

            let vertexes = assemb_vertexes_of_a_problem(&problem);

            let border_edges = scan_border_edges(&vertexes);

            let triangulations =
                assemb_triangulations_edges(&problem.triangulations, &border_edges);

            let intersection_graph =
                IntersectionGraph::new(&vertexes, &triangulations, &border_edges);

            intersection_graph
                .into_iter()
                .for_each(|g| test_if_graph_legal(&g));
        });
    }
}
