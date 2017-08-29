use std::fmt;
use std::collections::VecDeque;

use query::Query;
use query_result::QueryResult;

pub struct Screen {
    query_string: String,
    pre_selected: VecDeque<QueryResult>,
    selected: Option<QueryResult>,
    post_selected: VecDeque<QueryResult>
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            query_string: String::from(""),
            pre_selected: VecDeque::new(),
            selected: None,
            post_selected: VecDeque::new()
        }
    }

    pub fn current_query(&mut self, query: &Query) {
        self.reset();

        self.query_string = query.query_string();

        for query_result in query.results().take(10) {
            match self.selected {
                Some(_) => self.post_selected.push_back(query_result),
                None => self.selected = Some(query_result)
            }
        }
    }

    pub fn move_selection_down(&mut self) {
        if self.post_selected.is_empty() {
            return
        }

        self.selected = self.selected.take().and_then(|query_result| {
            self.pre_selected.push_back(query_result);
            self.post_selected.pop_front()
        });
    }

    pub fn move_selection_up(&mut self) {
        if self.pre_selected.is_empty() {
            return
        }

        self.selected = self.selected.take().and_then(|query_result| {
            self.post_selected.push_front(query_result);
            self.pre_selected.pop_back()
        })
    }

    fn reset(&mut self) {
        self.pre_selected.clear();
        self.post_selected.clear();
        self.selected = None;
    }
}

impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.query_string)?;

        for result in &self.pre_selected {
            writeln!(f, "{}\r", result)?;
        }

        match self.selected {
            Some(ref result) => writeln!(f, "{}\r", result)?,
            _ => ()
        }

        for result in &self.post_selected {
            writeln!(f, "{}\r", result)?;
        }

        Ok(())
    }
}
