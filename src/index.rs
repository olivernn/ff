use std::collections::HashMap;
use std::convert::From;
use std::fmt;

use jump::{Jumps, Jump};
use query::Query;
use query_result::QueryResult;

pub type Node = String;

#[derive(Debug)]
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

pub struct Index {
    graph: Graph
}

impl Index {
    pub fn new() -> Index {
        Index {
            graph: HashMap::new()
        }
    }

    pub fn push(&mut self, s: &str) {
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

    pub fn query(&self) -> Query {
        Query::new(&self.graph)
    }
}

impl fmt::Debug for Index {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n");
        for (location, edge_map) in &self.graph {
            writeln!(f, "'{}' =>", location);
            for (character, edges) in edge_map {
                writeln!(f, "\t{} =>", character);
                for edge in edges {
                    writeln!(f, "\t\t{:?}", edge);
                };
            };
        };

        writeln!(f, "hello")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use query::*;

    #[test]
    fn query_match() {
        let mut index = Index::new();
        index.push("fab/cab/dab");
        index.push("foo/bar/baz");

        let mut query = index.query();

        query.advance('f');
        query.advance('a');

        let results: Vec<QueryResult> = query.results().collect();

        assert_eq!(2, results.len());
        assert_eq!("fab/cab/dab", results[0].path);
        assert_eq!("foo/bar/baz", results[1].path);
    }
}