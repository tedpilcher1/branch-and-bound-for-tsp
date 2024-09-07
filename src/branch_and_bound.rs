use std::collections::HashMap;

use priority_queue::PriorityQueue;

use crate::tsp_types::{Node, TourNode};

fn run(tsp_graph: &Vec<Node>) -> (Vec<&Node>, i32) {
    // let mut best_tour = ...;
    let mut best_tour: Vec<&Node> = Vec::new();
    // we're formulating this as max cost not min - kinda dumb but whatever
    let mut best_cost: i32 = 0;
    let mut pq: PriorityQueue<TourNode, i32> = PriorityQueue::new();

    // create root node / get root node
    // for now, going to assume first tsp graph node is root/start
    // TODO implement From trait for TourNode using wrapper of tour, lower_bound
    let tour = vec![&tsp_graph[0]];

    // the pq uses greater number = higher priority, hence following line
    let lower_bound = std::i32::MAX - calc_lower_bound(tsp_graph, &tour);
    let root_node = TourNode {
        tour,
        lower_bound,
        unvisited_nodes: tsp_graph.iter().skip(1).collect(),
    };

    // insert root node into pq
    pq.push(root_node, lower_bound);

    // while pq not empty
    while !pq.is_empty() {
        // current_node = extract min from pq
        let (current_node, lower_bound) = pq.pop().unwrap(); // shouldn't panic as we checked pq isn't empty

        // if current_node.lower_bound >= best_cost, then continue (prune branch)
        if lower_bound < best_cost {
            continue;
        }

        // if current_node.unvisited_cities is empty then
        if current_node.tour.len() == tsp_graph.len() {
            // Have complete tour

            // if cost(current_node.path) < best_cost then
            let cost = cost(&current_node.tour);
            if cost > best_cost {
                // found new best tour, update vars
                best_cost = cost;
                best_tour = current_node.tour;
            }

            // continue
            continue;
        }

        for i in 0..current_node.unvisited_nodes.len() {
            let node = current_node.unvisited_nodes[i];

            let mut tour = current_node.tour.clone();
            tour.push(node);

            let lower_bound = std::i32::MAX - calc_lower_bound(tsp_graph, &tour);
            let unvisited_nodes: Vec<&Node> = current_node
                .unvisited_nodes
                .iter()
                .filter(|curr_node| **curr_node != node)
                .cloned()
                .collect();

            let new_node = TourNode {
                tour,
                lower_bound,
                unvisited_nodes,
            };

            if new_node.lower_bound > best_cost {
                pq.push(new_node, lower_bound);
            }
        }
    }

    // return best_tour, best_cost
    return (best_tour, std::i32::MAX - best_cost);
}

fn calc_lower_bound(tsp_graph: &Vec<Node>, tour: &Vec<&Node>) -> i32 {
    todo!()
}

fn cost(tour: &Vec<&Node>) -> i32 {
    todo!()
}
