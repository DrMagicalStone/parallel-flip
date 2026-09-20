use typed_index_collections::TiSlice;

use crate::graph::{basic_geom::{QuadrilateralEdgeIndex, QuadrilateralVertexIndex}, graph::UIndex, graph_entry::GraphEntry, vertex::Vertex};


/// Provides accesses of edges and vertexes in the view of a specified vertex.
pub struct QuadrilateralTriangleView<'a> {

    quadrilateral_of_view: &'a mut GraphEntry,
    vertex_index_of_view: QuadrilateralVertexIndex

}

impl<'a> QuadrilateralTriangleView<'a> {
    
    pub fn for_graph_entry_in_entry_array_on_index_in_center_graph_entry_and_intersect_with_center_graph_entry(center_graph_entry: &GraphEntry, edge_of_graph_entry_on: QuadrilateralEdgeIndex, entry_array: &'a mut TiSlice<UIndex, GraphEntry>) -> Option<Self> {

        let vertex_0 = center_graph_entry.0.get_vertexes()[edge_of_graph_entry_on.get_prev_and_next_edge()[0]];
        let quadrilateral_of_view = &mut entry_array[center_graph_entry.1.get_quadrilateral_on_edges()[edge_of_graph_entry_on]?];
        let vertex_of_view = if quadrilateral_of_view.0.get_vertexes_in_slice()[0] == vertex_0 {
            QuadrilateralVertexIndex::D
        } else {
            QuadrilateralVertexIndex::B
        };
        Some(Self { quadrilateral_of_view , vertex_index_of_view: vertex_of_view })
    }

    pub fn get_vertex_index_of_view(&self) -> &QuadrilateralVertexIndex {
        &self.vertex_index_of_view
    }

    pub fn get_vertex_of_view(&self) -> &Vertex {
        &(self.quadrilateral_of_view.0.get_vertexes()[self.vertex_index_of_view])
    }

    pub fn get_vertex_of_view_mut(&mut self) -> &mut Vertex {
        &mut (self.quadrilateral_of_view.0.get_vertexes_mut()[self.vertex_index_of_view])
    }

    pub fn get_quadrilateral_index_on_next_edge(&self) -> &Option<UIndex> {
        &self.quadrilateral_of_view.1.get_quadrilateral_on_edges()[self.vertex_index_of_view.get_prev_and_next_edge()[1]]
    }

    pub fn get_quadrilateral_index_on_next_edge_mut(&mut self) -> &mut Option<UIndex> {
        &mut self.quadrilateral_of_view.1.get_quadrilateral_on_edges_mut()[self.vertex_index_of_view.get_prev_and_next_edge()[1]]
    }

    pub fn get_quadrilateral_index_on_prev_edge(&self) -> &Option<UIndex> {
        &self.quadrilateral_of_view.1.get_quadrilateral_on_edges()[self.vertex_index_of_view.get_prev_and_next_edge()[0]]
    }

    pub fn get_quadrilateral_index_on_prev_edge_mut(&mut self) -> &mut Option<UIndex> {
        &mut self.quadrilateral_of_view.1.get_quadrilateral_on_edges_mut()[self.vertex_index_of_view.get_prev_and_next_edge()[0]]
    }
}