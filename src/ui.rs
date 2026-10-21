use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
};

use crate::app::App;

pub fn render(app: &App, frame: &mut Frame) {
    let area = frame.area();
    let outer = Block::default()
        .title(" Crabwire ")
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .style(Style::default().fg(Color::Yellow));

    let inner_area = outer.inner(area);
    frame.render_widget(outer, area);

    let [head, networks, status, help] = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(3),
        Constraint::Length(3),
        Constraint::Length(1),
    ])
    .areas(inner_area);

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

    frame.render_widget(
        Paragraph::new(format!(
            "{}  ○ {}",
            app.connection_state, app.connection_info.interface
        ))
        .block(inner_network),
        status,
    );

    frame.render_widget(
        Paragraph::new("↑↓ Select   Enter Activate   Esc Menu   q Quit"),
        help,
    );
}
