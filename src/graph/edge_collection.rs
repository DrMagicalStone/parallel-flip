use crate::graph::combinatorial::edge::Edge;
use crate::graph::geometric::line::Line;
use crate::graph::vertex::Vertex;
use crate::graph::vertex_collection::VertexCollection;
use std::fmt::Display;

pub struct EdgeCollection<'a> {
    edges: Vec<Vec<usize>>,
    vertexes: &'a VertexCollection<'a>,
}

impl<'a> Display for EdgeCollection<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl<'a> Clone for EdgeCollection<'a> {
    fn clone(&self) -> Self {
        Self {
            edges: self.edges.clone(),
            vertexes: self.vertexes,
        }
    }
}

impl<'a> EdgeCollection<'a> {
    pub fn new(vertexes: &'a VertexCollection<'a>) -> Self {
        Self {
            edges: vec![Vec::new(); vertexes.len()],
            vertexes,
        }
    }

    pub fn set_edge(&mut self, edge: &Edge) {
        let vertex_pair_index = edge.get_point_index_pair();
        self.edges[vertex_pair_index.0].push(vertex_pair_index.1);
        self.edges[vertex_pair_index.1].push(vertex_pair_index.0);
    }

    pub fn remove_edge(&mut self, edge: &Edge) {
        let position_pair = self.find_edge(edge).unwrap();
        let vertex_pair_index = edge.get_point_index_pair();
        self.edges[vertex_pair_index.0].swap_remove(position_pair.0);
        self.edges[vertex_pair_index.1].swap_remove(position_pair.1);
    }

    pub fn parallel_flip(&mut self, edge_collection_to_flip: &[Edge]) {}

    pub fn edge_intersect_with_any_edge(&self, edge: &Edge) -> bool {
        for (a, edges) in self.edges.iter().enumerate() {
            if let Some(_) = edges
                .iter()
                .find(|b| self.segment_intersect((edge, &Edge::new((a, **b), self.vertexes))))
            {
                return true;
            }
        }
        return false;
    }

    pub fn segment_intersect(&self, segment_pair: (&Edge, &Edge)) -> bool {
        Line::new(&segment_pair.0).edge_intersect(&segment_pair.1)
            && Line::new(&segment_pair.1).edge_intersect(&segment_pair.0)
    }

    fn find_edge(&self, edge: &Edge) -> Option<(usize, usize)> {
        let (a, b) = edge.get_point_index_pair();
        Some((
            self.edges[*a].iter().position(|x| *x == *b)?,
            self.edges[*b].iter().position(|x| *x == *a)?,
        ))
    }
}
