use std::{
    cmp::Ordering, fmt::{Debug, Display}, ops::{Add, Sub}, write,
};

use derive_more::{From, Into, Add, Sub, Neg, Mul};


use crate::graph::vec2c::Vec2C;

/// Type of integer for coordinates and geometric calculation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, From, Into, Add, Sub, Neg, Mul)]
#[mul(forward)]
pub(crate) struct Coord(i32);

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl Coord {
    pub const ZERO: Self = Self(0);
}

/// A vertex records its position based on cartesian coordinate system.
///
/// Subtracting two vertices yields a vec2c from the subtracted vertex to the original vertex.
/// Adding a vec2c to a vertex gives the result of translating the vertex along the vec2c.
/// Subtracting a vec2c from a vertex gives the result of translating the vertex along the vec2c opposite to the vec2c being subtracted.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vertex {
    x: Coord,
    y: Coord,
}

impl Display for Vertex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.x.partial_cmp(&other.x) {
            Some(Ordering::Equal) => {}
            ord => return ord,
        }
        self.y.partial_cmp(&other.y)
    }
}

impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.x.cmp(&other.x) {
            Ordering::Equal => {}
            ord => return ord,
        }
        self.y.cmp(&other.y)
    }
}

impl Sub for Vertex {
    type Output = Vec2C;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2C::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Add<Vec2C> for Vertex {
    type Output = Vertex;

    fn add(self, rhs: Vec2C) -> Self::Output {
        let rhs = rhs.get();
        Self::new(self.x + rhs.0, self.y + rhs.1)
    }
}

impl Sub<Vec2C> for Vertex {
    type Output = Vertex;

    fn sub(self, rhs: Vec2C) -> Self::Output {
        self + (-rhs)
    }
}

impl Vertex {

    pub const ORIGIN: Self = Self::new(Coord::ZERO, Coord::ZERO);

    pub const fn new(x: Coord, y: Coord) -> Self {
        Vertex { x: x, y: y }
    }

    pub fn get(&self) -> (Coord, Coord) {
        (self.x, self.y)
    }
}
