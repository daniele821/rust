use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{
        DisableLineWrap, EnableLineWrap, EnterAlternateScreen, LeaveAlternateScreen,
        disable_raw_mode, enable_raw_mode,
    },
};
use std::{io, thread::sleep, time::Duration};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    execute!(
        std::io::stdout(),
        EnterAlternateScreen,
        EnableMouseCapture,
        DisableLineWrap,
    )?;

    sleep(Duration::from_secs(3));

    execute!(
        io::stdout(),
        LeaveAlternateScreen,
        DisableMouseCapture,
        EnableLineWrap,
    )?;
    disable_raw_mode()?;
    Ok(())
}
