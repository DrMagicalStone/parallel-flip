use crate::graph::{quadrilateral::Quadrilateral, shared_triangle_relation::SharedTriangleEntry};



#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct GraphEntry(pub Quadrilateral, pub SharedTriangleEntry);

impl GraphEntry {

    pub fn parts(&self) -> (&Quadrilateral, &SharedTriangleEntry) {
        (&self.0, &self.1)
    }
    
    /// Flipping is a kind of rotation in the view of memory layout.
    /// This method makes sure that two Quadrilateral struct which represents the same quadrilateral (considering their diagonals) has the same memory layout.
    pub fn flip(&mut self) {
        let rotate_left = self.0.flip();
        self.1.flip(rotate_left);
    }
}