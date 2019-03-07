use std::any::Any;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::{self, stdin, stdout, BufRead, BufReader, Write};
use std::thread;

use termion::{
    cursor,
    event::Key,
    input::TermRead,
    raw::IntoRawMode,
    screen::AlternateScreen,
};

mod model;
mod view;

use model::Game;

const DICT_PATH: &str = "/usr/share/dict/words";

enum Error {
    IO(io::Error),
    Thread(Box<Any + Send>)
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IO(err)
    }
}

impl From<Box<dyn Any + Send>> for Error {
    fn from(err: Box<dyn Any + Send>) -> Self {
        Error::Thread(err)
    }
}

fn dict() -> std::result::Result<BTreeSet<String>, io::Error> {
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
    let mut game = Game::new();
    let stdin = stdin();
    let mut screen = AlternateScreen::from(stdout()).into_raw_mode()?;
    screen.flush()?;

    let j_handle = thread::spawn(dict);

    view::draw_board(&mut screen, &game)?;
    screen.flush()?;

    game.dict = j_handle.join()??;

    for c in stdin.keys() {
        // clear previous error
        game.error.take();

        match c? {
            // exit
            Key::Esc => break,

            // restart game
            Key::Ctrl('n') => game.restart(),

            // input
            Key::Backspace => {
                game.input.pop();
            }
            Key::Char('\n') => game.submit(),
            Key::Char(' ') => game.input.clear(),
            Key::Char(c) if c.is_alphanumeric() => game.input.push(c.to_ascii_uppercase()),

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
