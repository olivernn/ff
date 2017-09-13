extern crate termion;
extern crate ff;
extern crate libc;

use std::io::{Write, stdout, stdin};
use std::env;

use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::input::TermRead;
use termion::screen::AlternateScreen;

use ff::index;
use ff::ui::Screen;
use ff::query_result::QueryResult;

use std::fs::File;
use std::os::unix::io::IntoRawFd;

fn main() {
    let index = build_index();

    let mut query = index.query();
    let mut screen = Screen::new();
    let mut output: Option<QueryResult> = None;

    unsafe {
        let tty = File::open("/dev/tty").unwrap();
        libc::dup2(tty.into_raw_fd(), libc::STDIN_FILENO);
    }

    let stdin = stdin();
    // let mut stdout = stdout().into_raw_mode().unwrap();
    //let mut stdout = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    //let mut stdout = File::open("/dev/tty").expect("unable to open tty").into_raw_mode().expect("unable to convert to raw");
    let mut stdout = AlternateScreen::from(termion::get_tty().expect("get tty").into_raw_mode().expect("into raw mode"));

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

    drop(stdout);

    match output {
        Some(result) => {
            writeln!(std::io::stdout(), "{}", result.path)
        },
        _ => {
            Ok(())
        }
    };

    std::io::stdout().flush();
}

fn build_index() -> index::Index {
    if is_stdin_tty() {
        let root = env::current_dir().expect("unable to get current dir");
        return index::from_path(root);
    } else {
        let stdin = stdin();
        return index::from_buf_reader(stdin.lock());
    }
}

fn is_stdin_tty() -> bool {
    0 < unsafe { libc::isatty(libc::STDIN_FILENO) }
}