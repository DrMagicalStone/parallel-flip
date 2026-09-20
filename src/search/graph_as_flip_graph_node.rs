use std::{hash::Hash, todo};

use crate::graph::{graph::Graph, quadrilateral::Quadrilateral};



#[derive(PartialEq, Eq, Hash)]
pub struct GraphAsFlipGraphNode(Graph);

impl GraphAsFlipGraphNode {
    
    pub fn get_neibors(&self) -> Vec<Self> {
        todo!()
    }
}