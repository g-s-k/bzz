#![warn(clippy::pedantic)]

use std::collections::BTreeSet;
use std::fs::File;
use std::io::{self, stdin, stdout, BufRead, BufReader, Write};
use std::thread;

use termion::{cursor, event::Key, input::TermRead, raw::IntoRawMode, screen::AlternateScreen};

mod error;
mod model;
mod view;

use error::Error;
use model::Game;

const DICT_PATH: &str = "/usr/share/dict/words";

fn dict() -> Result<BTreeSet<String>, io::Error> {
    let f = File::open(DICT_PATH)?;
    BufReader::new(f)
        .lines()
        .filter(|w| match w {
            Err(_) => true,
            Ok(s) => s.chars().all(|c| c.is_alphabetic() && c.is_lowercase()),
        })
        .collect()
}

fn main() -> Result<(), Error> {
    // get this started asap
    let dict_handle = thread::spawn(dict);

    let mut game = Game::new();
    let stdin = stdin();

    let mut screen = AlternateScreen::from(stdout()).into_raw_mode()?;
    screen.flush()?;

    // draw the game board (will be full, but not interactive until the indexing is done)
    view::draw_board(&mut screen, &game)?;
    screen.flush()?;

    // wait for the word list to get indexed
    game.set_dict(dict_handle.join()??);

    for c in stdin.keys() {
        // clear previous error
        game.clear_error();

        match c? {
            // exit
            Key::Esc => break,

            // restart game
            Key::Ctrl('n') => game.restart(),

            // input
            Key::Backspace => game.backspace(),
            Key::Char('\n') => game.submit(),
            Key::Char(' ') => game.clear(),
            Key::Char(c) if c.is_alphanumeric() => game.push(c.to_ascii_uppercase()),

            // noise
            _ => continue,
        }

        view::draw_board(&mut screen, &game)?;
        screen.flush()?;
    }

    write!(screen, "{}", cursor::Show)?;
    screen.flush()?;

    Ok(())
}
