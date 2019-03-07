use std::collections::BTreeSet;
use std::fmt::Display;
use std::fs::File;
use std::io::{self, stdin, stdout, BufRead, BufReader, Write};
use std::mem::replace;

use rand::{
    seq::{IteratorRandom, SliceRandom},
    thread_rng, Rng,
};
use termion::{
    clear, color,
    cursor::{self, Goto},
    event::Key,
    input::TermRead,
    raw::IntoRawMode,
    screen::AlternateScreen,
    style,
};

const TERM_HEIGHT: u16 = 24;
const TERM_WIDTH: u16 = 80;

const START_X: u16 = 2;
const START_Y: u16 = 1;
const HALF_X: u16 = 4;
const HALF_Y: u16 = 3;
const LIST_START_X: u16 = 45;

const VOWELS: &[char] = &['A', 'E', 'I', 'O', 'U', 'Y'];
const CONSONANTS: &[char] = &[
    'B', 'C', 'D', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'V', 'W', 'X',
    'Z',
];

const DICT_PATH: &str = "/usr/share/dict/words";

type Result = io::Result<()>;

fn draw_hex<T: Display>(screen: &mut impl Write, x_min: u16, y_min: u16, letter: T) -> Result {
    write!(
        screen,
        // 00B7 is centered dot
        "{l0}{e:\u{00B7}<7}{l1}{e:\u{00B7}<9}{l2}{sp5:\u{00B7}^11}\
         {l3}{sp3:\u{00B7}>6}{c}{sp3:\u{00B7}<6}{l4}{sp5:\u{00B7}^11}\
         {l5}{e:\u{00B7}<9}{l6}{e:\u{00B7}<7}",
        // contents and styles
        c = letter,
        // separators
        e = "",
        sp3 = "   ",
        sp5 = "     ",
        // lines
        l0 = Goto(x_min + 3, y_min + 0),
        l1 = Goto(x_min + 2, y_min + 1),
        l2 = Goto(x_min + 1, y_min + 2),
        l3 = Goto(x_min + 0, y_min + 3),
        l4 = Goto(x_min + 1, y_min + 4),
        l5 = Goto(x_min + 2, y_min + 5),
        l6 = Goto(x_min + 3, y_min + 6),
    )
}

fn draw_middle_hex(screen: &mut impl Write, letter: char) -> Result {
    let yellow = color::Fg(color::Yellow);
    let bold = style::Bold;
    let reset = style::Reset;
    write!(screen, "{}", yellow)?;
    draw_hex(
        screen,
        START_X + 3 * HALF_X - 1,
        START_Y + 2 * HALF_Y + 1,
        &format!("{}{}{}{}", bold, letter, reset, yellow),
    )?;
    write!(screen, "{}", reset)?;
    Ok(())
}

fn draw_score(screen: &mut impl Write, score: usize) -> Result {
    write!(
        screen,
        "{goto}Score: {score:0>3}",
        goto = Goto(70, TERM_HEIGHT),
        score = score
    )
}

fn draw_err(screen: &mut impl Write, game: &Game) -> Result {
    if let Some(err) = &game.error {
        write!(
            screen,
            "{goto}{red}{err: <width$}{reset}",
            goto = Goto(START_X, TERM_HEIGHT - 1),
            red = color::Bg(color::Red),
            width = (TERM_WIDTH - 2) as usize,
            err = err,
            reset = style::Reset
        )
    } else {
        Ok(())
    }
}

fn draw_board(screen: &mut impl Write, game: &Game) -> Result {
    // clear everything
    write!(
        screen,
        "{clear}{start}{hide}",
        clear = clear::All,
        start = Goto(1, 1),
        hide = cursor::Hide
    )?;
    // draw some hexagons
    draw_hex(screen, START_X, START_Y + HALF_Y + 1, game.letters[1])?;
    draw_hex(screen, START_X + 3 * HALF_X - 1, START_Y, game.letters[2])?;
    draw_hex(
        screen,
        START_X + 6 * HALF_X - 2,
        START_Y + HALF_Y + 1,
        game.letters[3],
    )?;
    draw_hex(screen, START_X, START_Y + 3 * HALF_Y + 2, game.letters[4])?;
    draw_hex(
        screen,
        START_X + 3 * HALF_X - 1,
        START_Y + 4 * HALF_Y + 2,
        game.letters[5],
    )?;
    draw_hex(
        screen,
        START_X + 6 * HALF_X - 2,
        START_Y + 3 * HALF_Y + 2,
        game.letters[6],
    )?;

    // draw center one
    draw_middle_hex(screen, game.letters[0])?;

    // draw input box
    write!(
        screen,
        "{}{}{:width$}{}",
        Goto(START_X, TERM_HEIGHT),
        color::Bg(color::LightBlack),
        game.input,
        style::Reset,
        width = 67
    )?;

    // write the words out
    for (idx, word) in game.words.iter().enumerate() {
        write!(screen, "{}{}", Goto(LIST_START_X, 2 + idx as u16), word)?;
    }

    draw_score(screen, game.score)?;
    draw_err(screen, game)?;

    Ok(())
}

fn pick_letters() -> [char; 7] {
    let mut rng = thread_rng();

    let num_vowels = rng.gen_range(1, 3);

    let vowels = VOWELS.choose_multiple(&mut rng, num_vowels);
    let consonants = CONSONANTS.choose_multiple(&mut rng, 7 - num_vowels);

    let mut out = ['\0'; 7];
    vowels
        .into_iter()
        .chain(consonants.into_iter())
        .cloned()
        // randomizes the order
        .choose_multiple_fill(&mut rng, &mut out[..]);
    out
}

struct Game {
    input: String,
    letters: [char; 7],
    words: BTreeSet<String>,
    dict: BTreeSet<String>,
    score: usize,
    error: Option<String>,
}

impl Game {
    fn new(dict: BTreeSet<String>) -> Self {
        Self {
            input: String::new(),
            letters: pick_letters(),
            words: BTreeSet::new(),
            dict,
            score: 0,
            error: None,
        }
    }

    fn restart(&mut self) {
        self.input.clear();
        self.letters = pick_letters();
        self.words.clear();
        self.score = 0;
    }

    fn check(&self) -> Option<String> {
        let mut has_center = false;
        for c in self.input.chars() {
            if c == self.letters[0] {
                has_center = true;
            } else if !self.letters.contains(&c) {
                return Some(format!("{} is not in the letter set.", c));
            }
        }

        match self.input.len() {
            0 => return Some("No input entered.".into()),
            1..=3 => return Some("Words must be at least 4 characters.".into()),
            _ => (),
        }

        if !has_center {
            return Some(format!(
                "Words must include the center letter ({}).",
                self.letters[0]
            ));
        }

        if !self.dict.contains(&self.input.to_lowercase()) {
            return Some(format!("{} is not in the dictionary.", self.input));
        }

        None
    }

    fn score(&self) -> usize {
        if self.letters.iter().all(|&c| self.input.contains(c)) {
            3
        } else {
            1
        }
    }

    fn submit(&mut self) {
        if let Some(err) = self.check() {
            self.error = Some(err);
            self.input.clear();
        } else {
            let score = self.score();
            if self.words.insert(replace(&mut self.input, String::new())) {
                self.score += score;
            } else {
                self.error = Some("You already found that word!".into());
            }
        }
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

fn main() -> Result {
    let stdin = stdin();
    let mut screen = AlternateScreen::from(stdout()).into_raw_mode()?;
    screen.flush()?;

    // TODO: add loading screen
    let mut game = Game::new(dict()?);

    draw_board(&mut screen, &game)?;
    screen.flush()?;

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

        draw_board(&mut screen, &game)?;
        screen.flush()?;
    }

    write!(screen, "{}", cursor::Show)?;
    screen.flush()?;

    Ok(())
}
