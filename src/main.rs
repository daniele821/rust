use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent},
    execute,
    terminal::{
        Clear, ClearType, DisableLineWrap, EnableLineWrap, EnterAlternateScreen,
        LeaveAlternateScreen, disable_raw_mode, enable_raw_mode, size,
    },
};
use std::{
    io::{self, Write},
    time::Duration,
};

fn draw<W: Write>(w: &mut W) -> io::Result<()> {
    let (cols, rows) = size()?;
    execute!(w, Clear(ClearType::All))?;
    for y in 0..rows {
        for x in 0..cols {
            write!(w, "#")?;
        }
        if y == rows - 1 {
            write!(w, "\r")?;
        } else {
            write!(w, "\r\n")?;
        }
    }
    w.flush()
}

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    execute!(
        io::stdout(),
        EnterAlternateScreen,
        EnableMouseCapture,
        DisableLineWrap,
    )?;

    let mut stdout = io::stdout();
    draw(&mut stdout)?;

    loop {
        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(KeyEvent {
                    code: KeyCode::Esc, ..
                }) => break,
                Event::Resize(_, _) => {
                    draw(&mut stdout)?;
                }
                _ => {}
            }
        }
    }

    execute!(
        io::stdout(),
        LeaveAlternateScreen,
        DisableMouseCapture,
        EnableLineWrap,
    )?;
    disable_raw_mode()?;
    Ok(())
}
