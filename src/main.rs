use crossterm::{
    event::{self, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    prelude::{CrosstermBackend, Terminal},
    widgets::{Block, Borders},
};
use std::{io, time::Duration};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut running = true;
    while running {
        terminal.draw(|frame| {
            let area = frame.area();
            let block = Block::default()
                .title(" Hello, Ratatui! ")
                .borders(Borders::ALL);
            frame.render_widget(block, area);
        })?;

        // Handle user input
        if event::poll(Duration::from_millis(200))? {
            if let event::Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    running = false;
                }
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
