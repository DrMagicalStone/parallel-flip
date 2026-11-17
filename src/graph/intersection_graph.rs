mod enumerate;
mod new;

use core::panic;
use std::{collections::BTreeSet, marker::PhantomData, ops::Deref};

use rand::rngs::StdRng;
use typed_index_collections::{TiSlice, TiVec};

use crate::graph::{
    indexes::{IndexedEdge, IndexedEdgeIndex, VertexIndex},
    intersection_graph::EdgeState::{Intersecting, NoIntersecting},
    vertex::Vertex,
};

///
pub struct IntersectionGraphIter<'a> {
    intersection_graph: &'a IntersectionGraph<'a>,
    rng: StdRng,
    backtrak_depth_in_count_of_backtraked_edges: Counter<fn(u32) -> u32, u32>,
    backtraks_for_each_backtrak_depth: Counter<fn(u32) -> u32, u32>,
    random_tries_for_each_backtrak: Counter<fn(u32) -> u32, u32>,
    edge_states: TiVec<IndexedEdgeIndex, EdgeState>,
    on_graph_edges: Vec<IndexedEdgeIndex>,
    no_intersecting_edge_priority_list: BTreeSet<IndexedEdgeIndex>,
}

pub struct IntersectionGraph<'a> {
    intersection_graph_items: TiVec<IndexedEdgeIndex, IntersectionGraphItem>,
    border_edges: &'a TiSlice<IndexedEdgeIndex, IndexedEdge>,
    vertex_list: &'a TiSlice<VertexIndex, Vertex>,
}

impl<'a> Deref for IntersectionGraphIter<'a> {
    type Target = IntersectionGraph<'a>;

    fn deref(&self) -> &Self::Target {
        self.intersection_graph
    }
}

impl<'a> AsRef<IntersectionGraph<'a>> for IntersectionGraphIter<'a> {
    fn as_ref(&self) -> &IntersectionGraph<'a> {
        &*self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EdgeState {
    NoIntersecting,
    Intersecting(usize),
}

impl EdgeState {
    fn is_free(&self) -> bool {
        if let NoIntersecting = *self {
            true
        } else {
            false
        }
    }

    fn increase_intersect_count_and_if_changed_to_intersecting(&mut self) -> bool {
        match self {
            NoIntersecting => {
                *self = Intersecting(1);
                true
            }
            Intersecting(i) => {
                *self = Intersecting(*i + 1);
                false
            }
        }
    }

    fn decrease_intersect_count_and_if_changed_to_no_intersecting(&mut self) -> bool {
        match self {
            Intersecting(i) => {
                if *i > 1 {
                    *self = Intersecting(*i - 1);
                    false
                } else {
                    *self = NoIntersecting;
                    true
                }
            }
            NoIntersecting => panic!(
                "It is incorrect to decrease intersect count for an edge is currently not intersecting with any other edge."
            ),
        }
    }
}

struct IntersectionGraphItem {
    edge: IndexedEdge,
    intersecting_edges_index: Box<[IndexedEdgeIndex]>,
}

struct Counter<F, A>
where
    F: FnMut(A) -> u32,
{
    pub(crate) remaining: u32,
    pub(crate) target_provider: F,
    _marker: PhantomData<fn(A)>,
}

impl<F, A> Counter<F, A>
where
    F: FnMut(A) -> u32,
{
    fn new(target_provider: F) -> Self {
        Self {
            remaining: 0,
            target_provider,
            _marker: PhantomData,
        }
    }

    fn has_remaining_count_down(&mut self) -> bool {
        let has_remaining = self.remaining > 0;
        self.remaining -= 1;
        has_remaining
    }

    fn get_remaining(&self) -> u32 {
        self.remaining
    }

    fn set_remaining(&mut self, remaining: u32) {
        self.remaining = remaining;
    }

    fn reset_remaining(&mut self, arg: A) {
        self.remaining = (self.target_provider)(arg);
    }
}
