use std::ops::{Add, Deref, Sub};
use derive_more::{From, Into};

use crate::graph::basic_alge::Cyclic4;


/// Names (indexes) of the four vertexes of a quadrilateral.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, From, Into)]
pub struct QuadrilateralVertexIndex(Cyclic4);


/// Names (indexes) of the four edges of a quadrilateral.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, From, Into)]
pub struct QuadrilateralEdgeIndex(Cyclic4);


impl QuadrilateralVertexIndex {

    pub const A: Self = Self(Cyclic4::new(0));
    pub const B: Self = Self(Cyclic4::new(1));
    pub const C: Self = Self(Cyclic4::new(2));
    pub const D: Self = Self(Cyclic4::new(3));

    pub const VERTEXES: [Self; 4] = [Self::A, Self::B, Self::C, Self::D];

    pub fn prev(self) -> Self {
        Self(self.0.prev())
    }

    pub fn next(self) -> Self {
        Self(self.0.next())
    }

    pub fn opposite(self) -> Self {
        Self(self.0.opposite())
    }

    pub fn get_prev_and_next_edge(self) -> [QuadrilateralEdgeIndex; 2] {
        [QuadrilateralEdgeIndex(self.0.prev()), QuadrilateralEdgeIndex(self.0)]
    }
}

impl Add<Cyclic4> for QuadrilateralVertexIndex {
    type Output = Self;

    fn add(self, rhs: Cyclic4) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl Sub<Cyclic4> for QuadrilateralVertexIndex {
    type Output = Self;

    fn sub(self, rhs: Cyclic4) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl QuadrilateralEdgeIndex {

    pub const AB: Self = Self(Cyclic4::new(0));
    pub const BC: Self = Self(Cyclic4::new(1));
    pub const CD: Self = Self(Cyclic4::new(2));
    pub const DA: Self = Self(Cyclic4::new(3));

    pub const EDGES: [Self; 4] = [Self::AB, Self::BC, Self::CD, Self::DA];

    pub fn prev(self) -> Self {
        Self(self.0.prev())
    }

    pub fn next(self) -> Self {
        Self(self.0.next())
    }

    pub fn opposite(self) -> Self {
        Self(self.0.opposite())
    }

    pub fn get_prev_and_next_edge(self) -> [QuadrilateralVertexIndex; 2] {
        [QuadrilateralVertexIndex(self.0), QuadrilateralVertexIndex(self.0.next())]
    }
}

impl Add<Cyclic4> for QuadrilateralEdgeIndex {
    type Output = Self;

    fn add(self, rhs: Cyclic4) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl Sub<Cyclic4> for QuadrilateralEdgeIndex {
    type Output = Self;

    fn sub(self, rhs: Cyclic4) -> Self::Output {
        Self(self.0 - rhs)
    }
}
