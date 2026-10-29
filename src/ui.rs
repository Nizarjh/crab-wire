use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Modifier, Style, palette::tailwind::SLATE},
    text::{Line, Text},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, ListState, Paragraph},
};

use crate::app::{App, ConnectionState, Overlay, Screen};

pub fn render(app: &App, frame: &mut Frame) {
    let area = frame.area();
    match app.screen {
        Screen::Main => {
            render_main(area, app, frame);

            match app.overlay {
                Overlay::None => {}
                Overlay::PasswordInput => todo!(),
                Overlay::ConfirmDisconnect => todo!(),
                Overlay::Menu { mut state } => render_menu(area, frame, &mut state),
            }
        }
        Screen::KnownNetworks => todo!(),
        Screen::NetworkDetails => todo!(),
        Screen::Interfaces => todo!(),
    }
}

fn render_main(area: Rect, app: &App, frame: &mut Frame) {
    let outer = Block::default()
        .title(" Crabwire ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::Yellow));

    let status_block = outer.inner(area);
    frame.render_widget(outer, area);

    let [head, networks, status, help] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(3),
        Constraint::Length(3),
        Constraint::Length(1),
    ])
    .areas(status_block);

    frame.render_widget(
        Paragraph::new("Activate a connection").alignment(Alignment::Center),
        head,
    );

    let list_network = app
        .networks
        .iter()
        .map(|network| ListItem::new(network.ssid.as_str()))
        .collect::<Vec<_>>();
    let inner_network = Block::default()
        .borders(Borders::TOP | Borders::BOTTOM)
        .style(Style::default().fg(Color::Yellow));

    frame.render_widget(List::new(list_network), networks);

    let text = match &app.connection_state {
        ConnectionState::Connected(info) => {
            format!("{}  ○ {}", app.connection_state, info.interface)
        }
        _ => app.connection_state.to_string(),
    };

    frame.render_widget(Paragraph::new(text).block(inner_network), status);

    frame.render_widget(
        Paragraph::new("↑↓ Select   Enter Activate   Esc Menu   q Quit"),
        help,
    );
}

fn render_menu(area: Rect, frame: &mut Frame, state: &mut ListState) {
    let popup_area = area.centered(Constraint::Length(30), Constraint::Length(9));

    let popup_block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::Yellow).bg(Color::Black));

    let menu_area = popup_block.inner(popup_area);

    frame.render_widget(Clear, popup_area);
    frame.render_widget(popup_block, popup_area);

    let [head, area_for_items] =
        Layout::vertical([Constraint::Length(2), Constraint::Min(5)]).areas(menu_area);

    frame.render_widget(
        Paragraph::new("Please select an option").alignment(Alignment::Center),
        head,
    );

    let items = [
        ListItem::new(Line::from("Known Networks").alignment(Alignment::Center)),
        ListItem::new(Line::from("Interfaces").alignment(Alignment::Center)),
        ListItem::new(Text::from("Keybinds").alignment(Alignment::Center)),
        ListItem::new(Line::from("")),
        ListItem::new(Line::from("About").alignment(Alignment::Center)),
    ];

    let list = List::new(items)
        .highlight_style(Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD))
        .style(Style::default().fg(Color::White).bg(Color::Black));

    frame.render_stateful_widget(list, area_for_items, state);
}
