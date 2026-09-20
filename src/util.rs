

use std::{
    fs::File, io::{BufReader, BufWriter}, ops::Deref,
};

use itertools::Itertools;
use serde::{Deserialize, Serialize};
use typed_index_collections::{TiSlice, TiVec};

use crate::graph::{
    graph::Graph, indexes::{IndexedEdge, IndexedEdgeIndex, TriangulationIndex, VertexIndex}, vertex::{Coord, Vertex},
};

type FlipsOfAStep = Vec<[usize; 2]>;

type FlipsOfATriangulation = Vec<FlipsOfAStep>;

#[derive(Debug)]
struct SerializableCoord(pub Coord);

impl Deref for SerializableCoord {
    type Target = Coord;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Serialize for SerializableCoord {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        Into::<i32>::into(self.0).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for SerializableCoord {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        Result::Ok(Self(Coord::from(i32::deserialize(deserializer)?)))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Problem {
    pub content_type: String,
    pub instance_uid: String,
    pub vertexes_x: Vec<SerializableCoord>,
    pub vertexes_y: Vec<SerializableCoord>,
    pub triangulations: Vec<Vec<(usize, usize)>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Solution {
    content_type: String,
    instance_uid: String,
    flips: Vec<FlipsOfATriangulation>,
}

pub fn load_problem(file_name: &str) -> Problem {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
}

pub fn save_solution(file_name: &str, solution: &Solution) {
    let file = File::open(file_name).unwrap();
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, solution).unwrap();
}

pub fn assemb_vertexes_of_a_problem(problem: &Problem) -> TiVec<VertexIndex, Vertex> {
    problem
        .vertexes_x
        .iter()
        .zip(problem.vertexes_y.iter())
        .map(|(a, b)| Vertex::new(**a, **b))
        .collect()
}

pub fn assemb_triangulations_edges<'a>(triangulations: &Vec<Vec<(usize, usize)>>, vertexes: &'a TiSlice<VertexIndex, Vertex>) -> TiVec<TriangulationIndex, TiVec<IndexedEdgeIndex, IndexedEdge>> {
    triangulations.iter().map(|triangulation| {
        triangulation.iter().map(|(v0, v1)| (VertexIndex::from(*v0), VertexIndex::from(*v1))).map(|(v0, v1)| {
            IndexedEdge::new([v0, v1])
        }).collect()
    } ).collect()
}

pub fn scan_border_edges(vertexes: & TiSlice<VertexIndex, Vertex>) -> TiVec<IndexedEdgeIndex, IndexedEdge> {
    let sorted = {
        let mut temp = (0..vertexes.len()).map(|i| VertexIndex::from(i)).collect::<Vec<_>>().into_boxed_slice();
        temp.sort_by_key(|&index| vertexes[index]);
        temp
    };

    let mut border: Vec<VertexIndex> = Vec::new();

    sorted
        .iter()
        .chain(sorted.iter().rev().skip(1))
        .for_each(|v| {
            let vertex = vertexes[*v];
            while let Some([_, _]) = border.last_chunk().take_if(|[last_2, last]| {
                let last_2 = vertexes[*last_2];
                let last = vertexes[*last];
                (last - last_2)
                    .orientation(vertex - last)
                    .is_counter_clockwise()
            }) {
                border.pop();
            }
            border.push(*v);
        });
    border
        .into_iter()
        .tuple_windows()
        .map(|(v0, v1)| IndexedEdge::new([v0, v1]))
        .collect()
}

pub fn build_graphs_of_a_problem(triangulations: & TiSlice<TriangulationIndex, TiVec<IndexedEdgeIndex, IndexedEdge>>, border_edges: & TiSlice<IndexedEdgeIndex, IndexedEdge>, vertexes: & TiSlice<VertexIndex, Vertex>) -> Box<[Graph]> {
    triangulations
        .iter()
        .map(|f| Graph::new(&vertexes, &*f, border_edges))
        .collect()
}
