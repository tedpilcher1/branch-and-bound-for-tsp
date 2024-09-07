use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::tsp_types::Node;

const SKIP_LINES: i32 = 6;

pub fn read_file(file_path: String) -> Vec<Node> {
    let file = File::open(file_path).expect("Unable to read tsp file");
    let mut file = BufReader::new(file);
    let mut tsp_graph: Vec<Node> = Vec::new();

    // skip first x lines, there's probably a smarter way to do this but fuck it
    let mut buf = String::new();
    for _ in 0..SKIP_LINES {
        let _ = file.read_line(&mut buf);
    }

    let mut counter = 0;
    for line in file.lines() {
        let unwrapped_line = line.unwrap();
        let split_line = unwrapped_line.split_whitespace();
        match (split_line, counter).try_into() {
            // try_into() is cool
            Ok(node) => {
                // node.pos_in_graph = counter;
                tsp_graph.push(node);
            }
            Err(_) => {} // last line is EOR so can't convert
        }
        counter += 1;
    }

    tsp_graph
}
