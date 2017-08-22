use std::collections::{HashSet, BinaryHeap};
use std::cmp::{Eq, PartialEq, PartialOrd, Ord, Ordering};
use std::hash::{Hash, Hasher};

use index::{Graph, Node, Edge};
use min_set::MinSet;

struct Cursor {
    node: Node,
    score: usize,
    positions: Vec<usize>
}

impl PartialOrd for Cursor {
    fn partial_cmp(&self, other: &Cursor) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Cursor {
    fn cmp(&self, other: &Cursor) -> Ordering {
        self.score.cmp(&other.score)
    }
}

impl Hash for Cursor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.node.hash(state);
    }
}

impl PartialEq for Cursor {
    fn eq(&self, other: &Cursor) -> bool {
        self.node == other.node
    }
}

impl Eq for Cursor {

}

impl Cursor {
    fn extend(&self, edge: &Edge) -> Cursor {
        let mut positions = self.positions.clone();
        positions.push(edge.position);

        Cursor {
            node: edge.path.to_owned(),
            score: self.score + edge.score,
            positions: positions
        }
    }
}

struct Step {
    character: char,
    cursors: HashSet<Cursor>
}

impl Step {
    fn new(character: char) -> Step {
        Step {
            character: character,
            cursors: HashSet::new()
        }
    }

    fn push(&mut self, cursor: Cursor) {
        match self.cursors.take(&cursor) {
            Some(ref existing) if existing > &cursor => {
                self.cursors.insert(cursor);
            },

            Some(existing) => {
                self.cursors.insert(existing);
            }

            None => {
                self.cursors.insert(cursor);
            }
        }
    }
}

pub struct Match {
    path: String,
    score: usize,
    positions: Vec<usize>
}

impl From<Cursor> for Match {
    fn from(cursor: Cursor) -> Match {
        Match {
            path: cursor.node,
            score: cursor.score,
            positions: cursor.positions
        }
    }
}

impl PartialOrd for Match {
    fn partial_cmp(&self, other: &Match) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Match {
    fn cmp(&self, other: &Match) -> Ordering {
        self.score.cmp(&other.score)
    }
}

impl Hash for Match {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.path.hash(state);
    }
}

impl PartialEq for Match {
    fn eq(&self, other: &Match) -> bool {
        self.path == other.path
    }
}

impl Eq for Match {

}

pub struct Matches {
    heap: BinaryHeap<Match>
}

impl Matches {
    fn new() -> Matches {
        Matches { heap: BinaryHeap::new() }
    }
    fn insert(&mut self, m: Match) {
        self.heap.push(m)
    }
}

impl Iterator for Matches {
    type Item =  Match;

    fn next(&mut self) -> Option<Self::Item> {
        self.heap.pop()
    }
}

pub struct Query<'a> {
    graph: &'a Graph,
    steps: Vec<Step>
}

impl<'a> Query<'a> {
    pub fn new(graph: &'a Graph) -> Query<'a> {
        let step = Step::new('^');

        Query {
            graph: graph,
            steps: vec![step]
        }
    }

    pub fn matches(&self) -> Matches {
        let mut match_set: MinSet<Match> = MinSet::new();

        let mut matches = Matches::new();

        for cursor in &self.current_step().cursors {
            let edge_map = &self.graph[&cursor.node];

            edge_map.get(&'$').and_then(|edges| {
                for edge in edges {
                    match_set.insert(cursor.extend(edge).into())
                };

                Some(())
            });
        }

        for m in match_set.into_iter() {
            matches.insert(m);
        }

        return matches;
    }

    pub fn advance(&mut self, character: char) {
        let mut next_step = Step::new(character);

        for cursor in &self.current_step().cursors {
            let edge_map = &self.graph[&cursor.node];

            edge_map.get(&character).and_then(|edges| {
                for edge in edges {
                    next_step.push(cursor.extend(&edge));
                };

                Some(()) // we just have to return _something_
            });
        }

        self.steps.push(next_step);
    }

    pub fn back(&mut self) {
        self.steps.pop();
    }

    fn current_step(&self) -> &Step {
        self.steps.last().expect("it should be impossible to have no steps")
    }
}