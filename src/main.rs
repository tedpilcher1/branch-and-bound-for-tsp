use tsp_reader::read_file;
use tsp_utils::compute_cost_matrix;

mod branch_and_bound;
mod tsp_reader;
mod tsp_types;
mod tsp_utils;

fn main() {
    // load tsp graph from file
    let file_path = "tsplib/a280.tsp".to_string();
    let tsp_graph = read_file(file_path);

    // compute cost matrix
    let compute_matrix = compute_cost_matrix(&tsp_graph);

    // apply branch and bound
}
