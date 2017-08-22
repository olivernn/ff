extern crate termion;
extern crate walkdir;
extern crate ff;

use std::io::{Write, stdout, stdin};

use walkdir::{WalkDir, DirEntry, WalkDirIterator};

use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;

use ff::index::Index;

fn main() {
    let mut index = Index::new();

    for entry in WalkDir::new("/Users/oliven/code/ff").into_iter().filter_entry(|e| !is_hidden(e)) {
        let entry = entry.unwrap();
        let path = entry.path();        
        let path = path.strip_prefix("/Users/oliven/code/ff").expect("should work");

        index.push(path.to_str().expect("should work"));
    }


    let mut query = index.query();

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    writeln!(stdout, "index built...").unwrap();

    writeln!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();

    stdout.flush().unwrap();

    for c in stdin.keys() {
        //write!(stdout, "{}{}", termion::cursor::Goto(1, 1), termion::clear::CurrentLine).unwrap();

        match c.unwrap() {
            Key::Ctrl('c') => break,
            Key::Char(c) => {
                writeln!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();
                writeln!(stdout, "got char {}", c).unwrap();
                query.advance(c);
                for result in query.results().take(10) {
                    writeln!(stdout, "{}:{}\r", result.score, result.path).unwrap();
                }
            },
            Key::Backspace => {
                writeln!(stdout, "{}{}", termion::clear::All, termion::cursor::Goto(1, 1)).unwrap();
                query.back();
                for result in query.results().take(10) {
                    writeln!(stdout, "{}:{}\r", result.score, result.path).unwrap();
                }
            }
            _ => println!("other")
        }

        stdout.flush().unwrap();
    }

    write!(stdout, "{}", termion::cursor::Show).unwrap();
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}
