use priority_queue::PriorityQueue;

use crate::tsp_types::{Node, TourNode};

fn run(tsp_graph: &Vec<Node>) {
    // let mut best_tour = ...;
    let mut best_cost: f32 = std::f32::MAX;
    let mut pq: PriorityQueue<TourNode, i32> = PriorityQueue::new();

    // create root node / get root node
    // for now, going to assume first tsp graph node is root/start
    // TODO implement From trait for TourNode using wrapper of tour, lower_bound
    let tour = vec![&tsp_graph[0]];
    let lower_bound = calc_lower_bound(tsp_graph, &tour);
    let root_node = TourNode { tour, lower_bound };

    // insert root node into pq
    pq.push(root_node, lower_bound);

    // while pq not empty
    while !pq.is_empty() {
        // current_node = extract min from pq

        // if current_node.lower_bound >= best_cost, then continue (prune branch)

        // if current_node.unvisited_cities is empty then

        // Have complete tour

        // if cost(current_node.path) < best_cost then

        // found new best tour, update vars

        // continue

        // for each node in current_node.unvisited

        // NewNode = {
        //     Path: CurrentNode.Path + City,
        //     UnvisitedCities: CurrentNode.UnvisitedCities - {City}
        // }
        // NewNode.LowerBound = CalculateLowerBound(G, NewNode.Path)

        // if NewNode.LowerBound < BestCost then
        //     PQ.insert(NewNode)
    }

    // return best_tour, best_cost
}

fn calc_lower_bound(tsp_graph: &Vec<Node>, tour: &Vec<&Node>) -> i32 {
    todo!()
}

fn cost() -> f32 {
    todo!()
}
