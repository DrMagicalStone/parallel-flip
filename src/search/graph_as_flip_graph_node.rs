use pathfinding::{directed::astar::astar, num_traits::Zero};

use crate::graph::graph::UIndex;

pub trait AStarSearchable<C>: PartialEq + Sized
where
    C: Zero + Ord + Copy,
{
    fn successors(&self) -> impl IntoIterator<Item = (Self, C)>;

    fn heuristic(&self, target: Self) -> C;
}
