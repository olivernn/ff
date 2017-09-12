extern crate termion;
extern crate ff;

use std::io::{Write, stdout, stdin};
use std::env;

use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;

use ff::index;
use ff::ui::Screen;
use ff::query_result::{QueryResult};

fn main() {
    let root = env::current_dir().expect("unable to get current dir");
    let index = index::build(root);

    let mut query = index.query();
    let mut screen = Screen::new();
    let mut output: Option<QueryResult> = None;

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(stdout, "{}", screen).expect("failed to render screen");
    stdout.flush().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Ctrl('c') => break,
            Key::Char('\n') => {
                output = screen.selected;
                break
            },
            Key::Char(c) => {
                query.advance(c);
                screen.current_query(&query);
            },
            Key::Backspace => {
                query.back();
                screen.current_query(&query);
            },
            Key::Down => {
                screen.move_selection_down();
            },
            Key::Up => {
                screen.move_selection_up();
            },
            _ => println!("other")
        }

        write!(stdout, "{}", screen).expect("failed to render screen");
        stdout.flush().unwrap();
    }

    writeln!(stdout, "{}", termion::cursor::Show);

    match output {
        Some(result) => {
            writeln!(stdout, "{}", result.path)
        },
        _ => {
            Ok(())
        }
    };
}
