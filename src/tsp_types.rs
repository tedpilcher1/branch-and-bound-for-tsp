use std::str::SplitWhitespace;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Node {
    pub x: i32,
    pub y: i32,
    pub pos_in_graph: usize,
}

impl<'a> TryFrom<(SplitWhitespace<'a>, usize)> for Node {
    type Error = ();
    fn try_from(value: (SplitWhitespace<'a>, usize)) -> Result<Self, Self::Error> {
        if value.0.clone().count() == 1 {
            return Err(());
        }

        let node_vals: Vec<i32> = value
            .0
            .skip(1)
            .map(|coord| coord.parse::<i32>().unwrap())
            .collect();

        Ok(Node {
            x: node_vals[0],
            y: node_vals[1],
            pos_in_graph: value.1,
        })
    }
}

#[derive(Hash, PartialEq, Eq)]
pub struct TourNode<'a> {
    pub tour: Vec<&'a Node>,
    pub lower_bound: i32,
    pub unvisited_nodes: Vec<&'a Node>,
}
