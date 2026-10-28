use ratatui::{
    crossterm::event::{KeyCode, KeyEvent},
    widgets::ListState,
};

use crate::app::{App, Overlay};

pub async fn handle_key_event(app: &mut App, key_event: KeyEvent) {
    let len_of_networks = app.networks.len();
    match key_event.code {
        KeyCode::Char('q') => app.quit(),
        KeyCode::Esc => {
            if app.overlay == Overlay::None {
                app.overlay = Overlay::Menu {
                    state: ListState::default().with_selected(Some(0)),
                };
            } else {
                app.overlay = Overlay::None;
            }
        }

        KeyCode::Up if len_of_networks != 0 => {
            if app.selected_network != 0 {
                app.selected_network -= 1
            } else {
                app.selected_network = len_of_networks - 1
            }
        }
        KeyCode::Down if len_of_networks != 0 => {
            if app.selected_network != len_of_networks - 1 {
                app.selected_network += 1
            } else {
                app.selected_network = 0
            }
        }
        _ => {}
    };
}
