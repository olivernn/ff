extern crate termion;
extern crate walkdir;
extern crate time;
extern crate ff;

use std::io::{Write, stdout, stdin};

use time::PreciseTime;

use walkdir::{WalkDir, DirEntry, WalkDirIterator};

use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;

use ff::index::Index;
use ff::query::Query;

fn main() {
    let mut index = Index::new();

    let start = PreciseTime::now();

    for entry in WalkDir::new("/Users/oliven/code/ff").into_iter().filter_entry(|e| !is_hidden(e)) {
        let entry = entry.unwrap();
        let path = entry.path();        
        let path = path.strip_prefix("/Users/oliven/code/ff").expect("should work");

        index.push(path.to_str().expect("should work"));
    }

    let indexing_time = start.to(PreciseTime::now());

    let mut query = index.query();

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
                render(&mut stdout, &query);
            },
            Key::Backspace => {
                writeln!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();
                query.back();
                render(&mut stdout, &query);
            }
            _ => println!("other")
        }

        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}

fn render<T: Write>(io: &mut T, query: &Query) {
    writeln!(io, "{}", query).unwrap();
    for result in query.results().take(10) {
        writeln!(io, "{}\r", result).unwrap();
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}
