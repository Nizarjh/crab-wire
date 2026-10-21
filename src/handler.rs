use ratatui::crossterm::event::{KeyCode, KeyEvent};

use crate::app::{App, Screen};

pub async fn handle_key_event(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Char('q') => app.quit(),
        KeyCode::Esc => app.screen = Screen::Menu,
        KeyCode::Up => {
            if app.selected_network != 0 {
                app.selected_network -= 1
            } else {
                app.selected_network = app.networks.len() - 1
            }
        }
        KeyCode::Down => {
            if app.selected_network != app.networks.len() - 1 {
                app.selected_network += 1
            } else {
                app.selected_network = 0
            }
        }
        _ => {}
    };
}
