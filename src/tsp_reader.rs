use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn read_file(file_path: String) -> Vec<Vec<i32>> {
    let file = File::open(file_path).expect("Unable to read tsp file");
    let mut file = BufReader::new(file);
    let mut buf = String::new();
    let mut tsp_graph = Vec::new();

    // skip first 6 lines, there's probably a smarter way to do this but fuck it
    for _ in 0..6 {
        let _ = file.read_line(&mut buf);
    }

    for line in file.lines() {
        let unwrapped_line = line.unwrap();
        let split_line = unwrapped_line.split_whitespace();
        let node: Vec<i32> = split_line
            .skip(1)
            .map(|coord| coord.parse::<i32>().unwrap())
            .collect();
        tsp_graph.push(node);
    }

    // drop last element - it's just empty EOF line
    tsp_graph.pop();

    tsp_graph
}
