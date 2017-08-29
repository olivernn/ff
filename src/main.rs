extern crate termion;
extern crate time;
extern crate ff;

use std::io::{Write, stdout, stdin};
use std::env;

use time::PreciseTime;

use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;

use ff::index;
use ff::ui::Screen;

fn main() {
    let start = PreciseTime::now();
    let root = env::current_dir().expect("unable to get current dir");
    let index = index::build(root);
    let indexing_time = start.to(PreciseTime::now());

    let mut query = index.query();
    let mut screen = Screen::new();

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    writeln!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();
    writeln!(stdout, "index built in {}", indexing_time).unwrap();

    stdout.flush().unwrap();

    for c in stdin.keys() {
        //write!(stdout, "{}{}", termion::cursor::Goto(1, 1), termion::clear::CurrentLine).unwrap();

        match c.unwrap() {
            Key::Ctrl('c') => break,
            Key::Char(c) => {
                writeln!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();
                query.advance(c);
                screen.current_query(&query);
            },
            Key::Backspace => {
                writeln!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();
                query.back();
                screen.current_query(&query);
            },
            Key::Down => {
                screen.move_selection_down();
            },
            Key::Up => {
                screen.move_selection_up();
            }
            _ => println!("other")
        }

        write!(stdout, "{}", screen).expect("failed to write to screen");
        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
