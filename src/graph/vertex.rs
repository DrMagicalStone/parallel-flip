use std::{
    fmt::{Debug, Display},
    ops::{Deref, DerefMut, Index, IndexMut},
};

pub struct Vertex {
    x: u32,
    y: u32,
}

impl Display for Vertex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Debug for Vertex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vertex")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

impl Clone for Vertex {
    fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
        }
    }
}

impl Copy for Vertex {}

impl Vertex {
    pub fn new(x: u32, y: u32) -> Self {
        Vertex { x: x, y: y }
    }

    pub fn get(&self) -> (u32, u32) {
        (self.x, self.y)
    }
}
