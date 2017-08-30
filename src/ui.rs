use std::fmt;
use std::collections::VecDeque;

use termion::{cursor, style, clear, color};

use query::Query;
use query_result::QueryResult;

static PROMPT: &'static str = ">>> ";

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

// TODO: clean up the format param names a bit...
impl fmt::Display for Screen {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", clear::All, cursor::Goto(1, 1))?;
        write!(f, "{bold}{prompt}{query}{reset}\n\r",
            bold = style::Bold, prompt = PROMPT, query = self.query_string, reset = style::Reset)?;

        for result in &self.pre_selected {
            write!(f, "{} {} ", color::Bg(color::Black), color::Bg(color::Reset))?;
            write_result(f, result, style::NoUnderline)?;
        }

        for result in self.selected.as_ref() {
            write!(f, "{background}{bold}>{reset}{background} ",
                background = color::Bg(color::Black),
                bold = style::Bold,
                reset = style::Reset)?;

            write_result(f, result, color::Bg(color::Black))?;

            write!(f, "{reset}\r", reset = color::Bg(color::Reset))?;
        }

        for result in &self.post_selected {
            write!(f, "{} {} ", color::Bg(color::Black), color::Bg(color::Reset))?;
            write_result(f, result, style::NoUnderline)?;
        }

        let cursor_position = self.query_string.len() + PROMPT.len() + 1;

        write!(f, "{}", cursor::Goto(cursor_position as u16, 1))
    }
}

fn write_result<S: fmt::Display>(f: &mut fmt::Formatter, result: &QueryResult, style: S) -> fmt::Result {
    for (i, c) in result.path.char_indices() {
        if result.positions.contains(&i) {
            write!(f, "{bold}{c}{reset}{style}",
                bold = style::Bold,
                c = c,
                reset = style::Reset,
                style = style
            )?;
        } else {
            write!(f, "{}", c)?;
        }
    }

    write!(f, "\n\r")
}