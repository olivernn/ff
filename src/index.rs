use std::collections::HashMap;
use std::convert::From;

use jump::{Jumps, Jump};
use query::Query;

pub type Node = String;

pub struct Edge {
    pub path: String,
    pub score: usize,
    pub position: usize
}

impl From<Jump> for Edge {
    fn from(jump: Jump) -> Edge {
        Edge {
            path: jump.destination.prefix,
            score: jump.score,
            position: jump.destination.index
        }
    }
}

pub type Edges = HashMap<char, Vec<Edge>>;
pub type Graph = HashMap<String, Edges>;

struct Index {
    graph: Graph
}

impl Index {
    fn new() -> Index {
        Index {
            graph: HashMap::new()
        }
    }

    fn push(&mut self, s: &str) {
        let jumps = Jumps::new(s);

        for jump in jumps {
            self.graph
                .entry(jump.source.prefix.to_owned())
                .or_insert(HashMap::new())
                .entry(jump.destination.character)
                .or_insert(Vec::new())
                .push(jump.into());
        }
    }

    fn query(&self) -> Query {
        Query::new(&self.graph)
    }
}