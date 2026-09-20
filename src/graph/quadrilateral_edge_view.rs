use typed_index_collections::TiSlice;

use crate::graph::{basic_alge::Cyclic4, basic_geom::QuadrilateralEdgeIndex, graph::UIndex, graph_entry::GraphEntry, quadrilateral_triangle_view::QuadrilateralTriangleView, vertex::Vertex};


/// Provides accesses of edges and vertexes in the view of a specified edge.
pub struct QuadrilateralEdgeView<'a> {

    quadrilateral_of_view: &'a mut GraphEntry,
    edge_index_of_view: QuadrilateralEdgeIndex

}

impl<'a> QuadrilateralEdgeView<'a> {

    pub fn for_graph_entry_in_entry_array_on_index_in_center_graph_entry_and_edge_of_graph_entry_as_diagonal_of_center_graph_entry(center_graph_entry: &GraphEntry, edge_of_graph_entry_on: QuadrilateralEdgeIndex, entry_array: &'a mut TiSlice<UIndex, GraphEntry>) -> Option<Self> {
        let triangle_view = QuadrilateralTriangleView::for_graph_entry_in_entry_array_on_index_in_center_graph_entry_and_intersect_with_center_graph_entry(center_graph_entry, edge_of_graph_entry_on, entry_array)?;
        let edge_of_view = if <Cyclic4 as Into<u8>>::into(edge_of_graph_entry_on.into()) % 2u8 == 0 {
            triangle_view.get_vertex_index_of_view().get_prev_and_next_edge()[0]
        } else {
            triangle_view.get_vertex_index_of_view().get_prev_and_next_edge()[1]
        };
        let quadrilateral_of_view = &mut entry_array[center_graph_entry.1.get_quadrilateral_on_edges()[edge_of_graph_entry_on]?];
        Some(Self { quadrilateral_of_view , edge_index_of_view: edge_of_view })
    }

    pub fn get_edge_index_of_view(&self) -> &QuadrilateralEdgeIndex {
        &self.edge_index_of_view
    }

    pub fn get_quadrilateral_index_on_edge_of_view(&self) -> &UIndex {
        self.quadrilateral_of_view.1.get_quadrilateral_on_edges()[self.edge_index_of_view].as_ref().unwrap()
    }

    pub fn get_quadrilateral_index_on_edge_of_view_mut(&mut self) -> &mut UIndex {
        self.quadrilateral_of_view.1.get_quadrilateral_on_edges_mut()[self.edge_index_of_view].as_mut().unwrap()
    }

    pub fn get_prev_vertex(&self) -> &Vertex {
        &(self.quadrilateral_of_view.0.get_vertexes()[self.edge_index_of_view.get_prev_and_next_edge()[0]])
    }

    pub fn get_next_vertex(&self) -> &Vertex {
        &(self.quadrilateral_of_view.0.get_vertexes()[self.edge_index_of_view.get_prev_and_next_edge()[1]])
    }

}