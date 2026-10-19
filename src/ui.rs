use ratatui::{
    Frame, layout::{Alignment, Constraint, Layout}, style::{Color, Style}, widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::App;

pub fn render(app: &mut App, frame: &mut Frame) {
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
        Constraint::Length(1),
        Constraint::Length(1),
    ])
    .areas(inner_area);


}
