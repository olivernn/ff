use std::collections::BinaryHeap;
use std::cmp::Ordering;

use query::Match;

pub struct QueryResult {
    pub path: String,
    pub score: usize,
    pub positions: Vec<usize>
}

impl From<Match> for QueryResult {
    fn from(m: Match) -> Self {
        QueryResult {
            path: m.path,
            score: m.score,
            positions: m.positions
        }
    }
}

impl PartialEq for QueryResult {
    fn eq(&self, other: &QueryResult) -> bool {
        self.path == other.path
    }
}

impl Eq for QueryResult {

}

impl Ord for QueryResult {
    fn cmp(&self, other: &QueryResult) -> Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for QueryResult {
    fn partial_cmp(&self, other: &QueryResult) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct QueryResults {
    heap: BinaryHeap<QueryResult>
}

impl QueryResults {
    pub fn new() -> Self {
        QueryResults { heap: BinaryHeap::new() }
    }

    pub fn insert(&mut self, query_result: QueryResult) {
        self.heap.push(query_result)
    }
}

impl Iterator for QueryResults {
    type Item = QueryResult;

    fn next(&mut self) -> Option<Self::Item> {
        self.heap.pop()
    }
}