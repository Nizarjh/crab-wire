use std::io::BufWriter;

use anyhow::{Ok, Result};
use crabwire::{
    app::App,
    event::{
        Event::{self},
        EventHandler,
    },
    handler,
    tui::Tui,
};
use ratatui::{Terminal, backend::CrosstermBackend};

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = App::new();
    let terminal = Terminal::new(CrosstermBackend::new(BufWriter::new(std::io::stderr())))?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    while !app.should_quit {
        tui.draw(&mut app)?;
        match tui.events.next().await? {
            Event::Tick => {}
            Event::Key(key) => handler::handle_key_event(&mut app, key).await,
            _ => {}
        }
    }
    tui.exit()?;

    return Ok(());
}
