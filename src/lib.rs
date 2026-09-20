use crate::{graph::intersection_graph::{self, IntersectionGraph, IntersectionGraphIter}, util::{assemb_triangulations_edges, assemb_vertexes_of_a_problem, build_graphs_of_a_problem, load_problem, scan_border_edges}};

mod graph;
mod util;
mod search;



pub fn find_center_triangulation(problem_file_name: &str, solution_file_nme: &str) {

    let problem = load_problem(problem_file_name);

    let vertexes = assemb_vertexes_of_a_problem(&problem);

    let border_edges = scan_border_edges(&vertexes);

    let triangulations = assemb_triangulations_edges(&problem.triangulations, &vertexes);

    let graphs = build_graphs_of_a_problem(&triangulations, &border_edges, &vertexes);

    let intersection_graph = IntersectionGraph::new(&vertexes, &triangulations, &border_edges);

    intersection_graph.into_iter();
}