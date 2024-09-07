use crate::tsp_types::Node;

fn calculate_distance(node1: &Node, node2: &Node) -> f32 {
    let dx = (node1.x - node2.x) as f32;
    let dy = (node1.y - node2.y) as f32;

    (dx.powf(2.0) + dy.powf(2.0)).sqrt()
}

pub fn compute_cost_matrix(tsp_graph: &Vec<Node>) -> Vec<Vec<f32>> {
    let num_nodes = tsp_graph.len();
    let mut cost_matrix: Vec<Vec<f32>> = vec![vec![0.0; num_nodes]; num_nodes];

    for i in 0..num_nodes {
        for j in 0..num_nodes {
            if i == j {
                cost_matrix[i][j] = 0.0;
            } else {
                cost_matrix[i][j] = calculate_distance(&tsp_graph[i], &tsp_graph[j]);
            }
        }
    }

    cost_matrix
}
