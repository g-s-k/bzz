use std::fmt::Display;
use std::io::{self, Write};

use termion::{
    clear, color,
    cursor::{self, Goto},
    style,
};

use super::Game;

type Result = io::Result<()>;

const TERM_HEIGHT: u16 = 24;
const TERM_WIDTH: u16 = 80;

const START_X: u16 = 2;
const START_Y: u16 = 1;
const HALF_X: u16 = 4;
const HALF_Y: u16 = 3;
const LIST_START_X: u16 = 45;

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
        l0 = Goto(x_min + 3, y_min),
        l1 = Goto(x_min + 2, y_min + 1),
        l2 = Goto(x_min + 1, y_min + 2),
        l3 = Goto(x_min, y_min + 3),
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
        "{goto}{bgc}{fgc}{bold} Score: {score:0>3} {reset}",
        goto = Goto(69, TERM_HEIGHT),
        bgc = color::Bg(color::LightWhite),
        fgc = color::Fg(color::Black),
        bold = style::Bold,
        score = score,
        reset = style::Reset
    )
}

fn draw_err(screen: &mut impl Write, game: &Game) -> Result {
    if let Some(err) = game.error() {
        write!(
            screen,
            "{goto}{red}{err: <width$}{reset}",
            goto = Goto(0, TERM_HEIGHT - 1),
            red = color::Bg(color::Red),
            width = TERM_WIDTH as usize,
            err = err,
            reset = style::Reset
        )
    } else {
        Ok(())
    }
}

pub fn draw_board(screen: &mut impl Write, game: &Game) -> Result {
    let letters = game.letters();
    // clear everything
    write!(
        screen,
        "{clear}{start}{hide}",
        clear = clear::All,
        start = Goto(1, 1),
        hide = cursor::Hide
    )?;
    // draw some hexagons
    draw_hex(screen, START_X, START_Y + HALF_Y + 1, letters[1])?;
    draw_hex(screen, START_X + 3 * HALF_X - 1, START_Y, letters[2])?;
    draw_hex(
        screen,
        START_X + 6 * HALF_X - 2,
        START_Y + HALF_Y + 1,
        letters[3],
    )?;
    draw_hex(screen, START_X, START_Y + 3 * HALF_Y + 2, letters[4])?;
    draw_hex(
        screen,
        START_X + 3 * HALF_X - 1,
        START_Y + 4 * HALF_Y + 2,
        letters[5],
    )?;
    draw_hex(
        screen,
        START_X + 6 * HALF_X - 2,
        START_Y + 3 * HALF_Y + 2,
        letters[6],
    )?;

    // draw center one
    draw_middle_hex(screen, letters[0])?;

    // draw input box
    write!(
        screen,
        "{}{}{:width$}{}",
        Goto(0, TERM_HEIGHT),
        color::Bg(color::LightBlack),
        game.input(),
        style::Reset,
        width = 68
    )?;

    // write the words out
    for (idx, word) in game.words().enumerate() {
        #[allow(clippy::cast_possible_truncation)]
        write!(screen, "{}{}", Goto(LIST_START_X, 2 + idx as u16), word)?;
    }

    draw_score(screen, game.score())?;
    draw_err(screen, game)?;

    Ok(())
}
