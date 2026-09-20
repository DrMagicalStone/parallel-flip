//! The structs in this mod are indexes for different structs in TiVec (Variable-length array with specified indexes) and TiSlice (basically a slice of a TiVec).
//! These structs can usually identify which TiVec they are indices of.



use derive_more::{From, Into, Add, Sub, Mul};

use crate::graph::{graph::UIndex, vertex::Vertex};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, From, Into, Add, Sub, Mul)]
pub struct VertexIndex(pub usize);

#[derive(Debug, Eq, Clone, Copy, Hash)]
pub struct IndexedEdge {
    vertex_index_pair: [VertexIndex; 2],
}

impl<'a> PartialEq for IndexedEdge {
    
    /// Correctness: each possible IndexedEdge is only newed at most once SO FAR, therefore it is impossible to have two IndexedEdges that representing the same edge but with different vertex_index order.
    fn eq(&self, other: &Self) -> bool {
        self.vertex_index_pair == other.vertex_index_pair
    }
}

impl IndexedEdge {

    pub fn new (vertex_index_pair: [VertexIndex; 2]) -> IndexedEdge {
        Self { vertex_index_pair }
    }

    pub fn contains(&self, vertex_index: VertexIndex) -> bool {
        (self.vertex_index_pair[0] == vertex_index) || (self.vertex_index_pair[1] == vertex_index)
    }

    pub fn get_another_vertex_index(&self, vertex_index: VertexIndex) -> VertexIndex {
        if self.vertex_index_pair[0] == vertex_index {
            self.vertex_index_pair[1]
        } else {
            self.vertex_index_pair[0]
        }
    }

    pub fn get_vertex_index_pair(&self) -> [VertexIndex; 2] {
        self.vertex_index_pair
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, From, Into)]
pub struct IndexedEdgeIndex(pub usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, From, Into)]
pub struct TriangulationIndex(pub usize);

