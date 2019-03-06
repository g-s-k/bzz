use std::io::{self, stdin, stdout, Write};
use termion::{
    clear, color,
    cursor::{self, Goto},
    event::Key,
    input::TermRead,
    raw::IntoRawMode,
    screen::AlternateScreen,
    style,
};

const START_X: u16 = 4;
const START_Y: u16 = 2;
const HALF_X: u16 = 4;
const HALF_Y: u16 = 4;

type Result = io::Result<()>;

fn draw_hex(screen: &mut impl Write, x_min: u16, y_min: u16, letter: &str) -> Result {
    write!(
        screen,
        "{l0}{s0}{l1}{s2}       {s1}{l2}{s2}         {s1}{l3}{s2}           \
         {s1}{l4}{s2}      {c}      {s2}{l5}{s1}           \
         {s2}{l6}{s1}         {s2}{l7}{s1}{s0}{s2}{l8}",
        // contents and styles
        c = letter,
        // separators
        s0 = "_".repeat(7),
        s1 = '\\',
        s2 = '.',
        // lines
        l0 = Goto(x_min + 4, y_min),
        l1 = Goto(x_min + 3, y_min + 1),
        l2 = Goto(x_min + 2, y_min + 2),
        l3 = Goto(x_min + 1, y_min + 3),
        l4 = Goto(x_min + 0, y_min + 4),
        l5 = Goto(x_min + 1, y_min + 5),
        l6 = Goto(x_min + 2, y_min + 6),
        l7 = Goto(x_min + 3, y_min + 7),
        l8 = Goto(x_min + 4, y_min + 8),
    )
}

fn draw_middle_hex(screen: &mut impl Write, letter: char) -> Result {
    let yellow = color::Fg(color::Yellow);
    let bold = style::Bold;
    let reset = style::Reset;
    write!(screen, "{}", yellow)?;
    draw_hex(
        screen,
        START_X + 3 * HALF_X,
        START_Y + 2 * HALF_Y,
        &format!("{}{}{}{}", bold, letter, reset, yellow),
    )?;
    write!(screen, "{}", reset)?;
    Ok(())
}

fn draw_board(screen: &mut impl Write) -> Result {
    // clear everything
    write!(
        screen,
        "{clear}{start}{hide}",
        clear = clear::All,
        start = Goto(1, 1),
        hide = cursor::Hide
    )?;
    // draw some hexagons
    draw_hex(screen, START_X, START_Y + HALF_Y, "A")?;
    draw_hex(screen, START_X + 3 * HALF_X, START_Y, "B")?;
    draw_hex(screen, START_X + 6 * HALF_X, START_Y + HALF_Y, "C")?;
    draw_hex(screen, START_X, START_Y + 3 * HALF_Y, "D")?;
    draw_hex(screen, START_X + 3 * HALF_X, START_Y + 4 * HALF_Y, "E")?;
    draw_hex(screen, START_X + 6 * HALF_X, START_Y + 3 * HALF_Y, "F")?;

    // draw center one
    draw_middle_hex(screen, 'G')?;

    Ok(())
}

fn main() -> Result {
    let stdin = stdin();
    let mut screen = AlternateScreen::from(stdout()).into_raw_mode()?;
    screen.flush()?;

    draw_board(&mut screen)?;
    screen.flush()?;

    for c in stdin.keys() {
        match c? {
            Key::Esc => break,
            _ => continue,
        }
    }

    Ok(())
}
