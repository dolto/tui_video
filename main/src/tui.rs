use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};
use crossterm::{
    terminal::{enable_raw_mode, disable_raw_mode},
    execute,
};
use std::io::stdout;

pub type Tui = Terminal<CrosstermBackend<std::io::Stdout>>;

pub fn init() -> anyhow::Result<Tui> {
    enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout());
    Ok(Terminal::new(backend)?)
}

pub fn restore() -> anyhow::Result<()> {
    disable_raw_mode()?;
    execute!(stdout(), crossterm::terminal::LeaveAlternateScreen)?;
    Ok(())
}
