use std::{
    cmp::Ordering, fmt::{Debug, Display}, ops::{Add, Mul, Neg, Sub}, write,
};

use crate::graph::{edge::Edge, vertex::{Coord, Vertex}};

///A vec2c is a "2"-dimensional vector of "Coord" type.
///
/// A vec2c records itself based on cartesian coordinate system.
///
/// The multiplication of two vec2c represents their inner product.
///
/// The method named cross and the function with the same name represents to
/// the z-coordinate components of cross product of two vec2c
/// while embedding the whole (Euclidean) plane to a 3-dimentional Euclidean space's with a right hand cartesian coordinate system x-y plane.
/// Orientation of two vec2c is defined based on the same premises as cross product and depends on the value that cross provides.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Vec2C {
    x: Coord,
    y: Coord,
}

impl Display for Vec2C {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vec2C({}, {})", self.x, self.y)
    }
}

impl PartialOrd for Vec2C {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.x.partial_cmp(&other.x) {
            Some(Ordering::Equal) => {}
            ord => return ord,
        }
        self.y.partial_cmp(&other.y)
    }
}

impl Ord for Vec2C {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.x.cmp(&other.x) {
            Ordering::Equal => {}
            ord => return ord,
        }
        self.y.cmp(&other.y)
    }
}

impl Add for Vec2C {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Vec2C {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Neg for Vec2C {
    type Output = Vec2C;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}

impl Mul for Vec2C {
    type Output = Coord;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl Mul<Coord> for Vec2C {
    type Output = Self;

    fn mul(self, rhs: Coord) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl Vec2C {
    pub fn new(x: Coord, y: Coord) -> Self {
        Vec2C { x: x, y: y }
    }

    pub fn from_edge(edge: Edge) -> Self {
        edge.as_vec2c()
    }

    pub fn get(&self) -> (Coord, Coord) {
        (self.x, self.y)
    }

    pub fn cross(self, rhs: Self) -> Coord {
        self.x * rhs.y - self.y * rhs.x
    }

    pub fn orientation(self, rhs: Self) -> Orientation {
        match self.cross(rhs).cmp(&Coord::ZERO) {
            Ordering::Less => Orientation::Clockwise,
            Ordering::Equal => Orientation::Parallel,
            Ordering::Greater => Orientation::CounterClockwise,
        }
    }
}

pub enum Orientation {
    Clockwise,
    Parallel,
    CounterClockwise,
}

impl Orientation {
    pub fn is_clockwise(&self) -> bool {
        match self {
            Self::Clockwise => true,
            _ => false,
        }
    }

    pub fn is_parallel(&self) -> bool {
        match self {
            Self::Parallel => true,
            _ => false,
        }
    }

    pub fn is_counter_clockwise(&self) -> bool {
        match self {
            Self::CounterClockwise => true,
            _ => false,
        }
    }
}

pub fn cross(a: Vec2C, b: Vec2C) -> Coord {
    a.cross(b)
}

pub fn orientation(a: Vec2C, b: Vec2C) -> Orientation {
    a.orientation(b)
}
