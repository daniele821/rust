use crossterm::{
    event::DisableMouseCapture,
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Bar, BarChart, BarGroup, Block, Borders},
};
use std::{io, thread::sleep, time::Duration};

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Store formatted labels in Vec<String>
    let labels: Vec<String> = (1..=30).map(|i| i.to_string()).collect();

    // Assign colors to bars (e.g., every 5th bar is red, others are white)
    let bars: Vec<Bar<'_>> = labels
        .iter()
        .enumerate()
        .map(|(i, label)| {
            let color = if (i + 1) % 5 == 0 {
                Color::Red
            } else {
                Color::White
            };
            Bar::default()
                .value((i + 1) as u64)
                .style(Style::default().fg(color))
        })
        .collect();

    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(f.size());

        let bar_chart = BarChart::default()
            .block(
                Block::default()
                    .title("Colored Bar Chart")
                    .borders(Borders::ALL),
            )
            .data(BarGroup::default().bars(&bars))
            .bar_width(2)
            .bar_gap(1);

        f.render_widget(bar_chart, chunks[0]);
    })?;

    sleep(Duration::from_secs(2));

    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}
