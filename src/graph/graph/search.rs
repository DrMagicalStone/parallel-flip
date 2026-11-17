use std::vec;

use itertools::enumerate;
use pathfinding::num_traits::Zero;
use typed_index_collections::{TiSlice, TiVec, ti_vec};

use crate::{
    graph::graph::{Graph, UIndex, flip::sort_quadrilaterals_get_numbers_of_flippable},
    search::graph_as_flip_graph_node::AStarSearchable,
};

impl AStarSearchable<UIndex> for Graph {
    fn successors(&self) -> impl IntoIterator<Item = (Self, UIndex)> {
        self.get_all_possible_parallel_flips_with_no_sort()
            .into_iter()
            .map(|mut g| {
                g.0.numbers_of_flippable =
                    sort_quadrilaterals_get_numbers_of_flippable(&mut g.0.data);
                (g.0, g.1)
            })
    }

    fn heuristic(&self, _target: Self) -> UIndex {
        UIndex::ZERO //Todo
    }
}

impl Graph {
    pub(super) fn get_all_possible_parallel_flips_with_no_sort(
        &self,
    ) -> impl IntoIterator<Item = (Self, UIndex, TiVec<UIndex, bool>)> {
        let mut indexes_to_flip: TiVec<UIndex, bool> =
            ti_vec![false; self.get_numbers_of_flippable().into()];
        let mut sucessors_vec = Vec::new();
        fn enumerate_indexes(
            i: UIndex,
            indexes: &mut TiSlice<UIndex, bool>,
            output_successors: &mut Vec<(Graph, UIndex, TiVec<UIndex, bool>)>,
            original: &Graph,
        ) {
            if <UIndex as Into<usize>>::into(i) == indexes.len() {
                let mut cloned = original.clone();
                cloned.assume_no_conflict_parallel_flip(&indexes);
                output_successors.push((cloned, UIndex(1), indexes.to_vec()));
                return;
            }

            indexes[i] = false;
            enumerate_indexes(i + 1, indexes, output_successors, original);

            indexes[i] = true;
            if original.can_flip_in_parallel_flip(&indexes) {
                enumerate_indexes(i + 1, indexes, output_successors, original);
            }
            indexes[i] = false;
        }
        enumerate_indexes(UIndex::ZERO, &mut indexes_to_flip, &mut sucessors_vec, self);
        sucessors_vec
    }
}
