use std::str::SplitWhitespace;

#[derive(Debug)]
pub struct Node {
    pub x: i32,
    pub y: i32,
}

impl<'a> TryFrom<SplitWhitespace<'a>> for Node {
    type Error = ();
    fn try_from(value: SplitWhitespace<'a>) -> Result<Self, Self::Error> {
                
        if value.clone().count() == 1 {
            return Err(());
        }

        let node_vals: Vec<i32> = value
            .skip(1)
            .map(|coord| coord.parse::<i32>().unwrap())
            .collect();

        Ok(Node {
            x: node_vals[0],
            y: node_vals[1],
        })
    }
}
